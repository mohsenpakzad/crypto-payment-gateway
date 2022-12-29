use crate::{
    entities::payment::{self, PaymentStatus},
    errors::{NotFoundError, PaymentError},
    models::ws::{WsInputMessage, WsOutputMessage},
    services::{
        crypto_currency_service, fiat_currency_service, kucoin_api_service, network_service,
        payment_service, wallet_service, web3_service,
    },
};
use actix_web::{
    get,
    web::{Data, Path, Payload, ServiceConfig},
    Error, HttpRequest, Responder,
};
use actix_ws::Message;
use anyhow::Result;
use chrono::Utc;
use futures_util::{
    future::{self, Either},
    StreamExt,
};
use sea_orm::{DbConn, Set};
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tokio::{
    pin,
    task::{self, JoinHandle},
    time::interval,
};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

struct SocketData {
    db: Data<DbConn>,
    payment: Mutex<payment::Model>,
    payment_task_handle: Mutex<Option<JoinHandle<()>>>,
}

#[get("/ws/payments/{payment_id}")]
async fn payment_ws_handshake(
    path: Path<i32>,
    req: HttpRequest,
    body: Payload,
    db: Data<DbConn>,
) -> Result<impl Responder, Error> {
    let payment_id = path.into_inner();

    let payment = payment_service::find_by_id(&db, payment_id)
        .await?
        .ok_or(NotFoundError::PaymentNotFoundWithGivenId)?;

    if payment.status != PaymentStatus::Waiting {
        return Err(PaymentError::PaymentIsDoneOrExpired(payment.status))?;
    }

    let (response, session, msg_stream) = actix_ws::handle(&req, body)?;

    // spawn websocket handler (and don't await it) so that the response is returned immediately
    task::spawn_local(payment_ws(payment, session, msg_stream, db));

    Ok(response)
}
/// Process messages received from the client, respond to ping messages, and monitor
/// connection health to detect network issues and free up resources.
pub async fn payment_ws(
    payment: payment::Model,
    mut session: actix_ws::Session,
    mut msg_stream: actix_ws::MessageStream,
    db: Data<DbConn>,
) {
    log::info!("connected to websocket");

    let mut last_heartbeat = Instant::now();
    let mut interval = interval(HEARTBEAT_INTERVAL);

    let socket_data = Arc::new(SocketData {
        db,
        payment: Mutex::new(payment),
        payment_task_handle: Mutex::new(None),
    });

    let reason = loop {
        // create "next client timeout check" future
        let tick = interval.tick();
        // required for select()
        pin!(tick);

        // waits for either `msg_stream` to receive a message from the client or the heartbeat
        // interval timer to tick, yielding the value of whichever one is ready first
        match future::select(msg_stream.next(), tick).await {
            // received message from WebSocket client
            Either::Left((Some(Ok(msg)), _)) => {
                log::debug!("msg: {msg:?}");

                match msg {
                    Message::Text(text) => {
                        let res =
                            process_text_msg(&mut session, &text, Arc::clone(&socket_data)).await;
                        if let Err(err) = res {
                            session
                                .text(WsOutputMessage::Error(err).into_str())
                                .await
                                .unwrap();
                        }
                    }

                    Message::Binary(_) => {
                        log::warn!("no support for binary message");
                    }

                    Message::Ping(bytes) => {
                        last_heartbeat = Instant::now();
                        let _ = session.pong(&bytes).await;
                    }

                    Message::Pong(_) => {
                        last_heartbeat = Instant::now();
                    }

                    Message::Close(reason) => {
                        break reason;
                    }

                    Message::Continuation(_) => {
                        log::warn!("no support for continuation frames");
                    }

                    // no-op; ignore
                    Message::Nop => {}
                };
            }

            // client WebSocket stream error
            Either::Left((Some(Err(err)), _)) => {
                log::error!("{}", err);
                break None;
            }

            // client WebSocket stream ended
            Either::Left((None, _)) => break None,

            // heartbeat interval ticked
            Either::Right((_inst, _)) => {
                // if no heartbeat ping/pong received recently, close the connection
                if Instant::now().duration_since(last_heartbeat) > CLIENT_TIMEOUT {
                    log::info!(
                        "client has not sent heartbeat in over {CLIENT_TIMEOUT:?}; disconnecting"
                    );

                    break None;
                }

                // send heartbeat ping
                let _ = session.ping(b"").await;
            }
        }
    };

    // attempt to close connection gracefully
    let _ = session.close(reason).await;

    log::info!("disconnected from websocket");
}

async fn process_text_msg(
    session: &mut actix_ws::Session,
    text: &str,
    socket_data: Arc<SocketData>,
) -> Result<()> {
    let input_msg = WsInputMessage::try_from(text)?;

    match input_msg {
        WsInputMessage::ChooseCrypto(crypto_currency_id) => {
            choose_crypto(crypto_currency_id, session, socket_data).await
        }
    }
}

async fn choose_crypto(
    crypto_currency_id: i32,
    session: &mut actix_ws::Session,
    socket_data: Arc<SocketData>,
) -> Result<()> {
    let crypto_currency = crypto_currency_service::find_by_id(&socket_data.db, crypto_currency_id)
        .await?
        .ok_or(NotFoundError::CryptoCurrencyNotFoundWithGivenId)?;

    let fiat_currency = fiat_currency_service::find_by_id(
        &socket_data.db,
        socket_data.payment.lock().unwrap().fiat_currency_id,
    )
    .await?
    .ok_or(NotFoundError::FiatCurrencyNotFoundWithGivenId)?;

    let crypto_amount = kucoin_api_service::fiat_to_crypto(
        &fiat_currency.symbol,
        socket_data.payment.lock().unwrap().amount,
        &crypto_currency.symbol,
    )
    .await
    .unwrap();

    if let Some(dest_wallet_id) = socket_data.payment.lock().unwrap().dest_wallet_id {
        let mut payment_task_handle = socket_data.payment_task_handle.lock().unwrap();
        if payment_task_handle.is_some() {
            log::info!("Abort previous payment task");

            payment_task_handle.as_ref().unwrap().abort();
            *payment_task_handle = None;
        }
        wallet_service::free(&socket_data.db, dest_wallet_id).await?;
    }

    let wallet = wallet_service::reserve(&socket_data.db, crypto_currency.network_id).await?;

    let mut payment = payment::ActiveModel::from(socket_data.payment.lock().unwrap().clone());
    payment.crypto_currency_id = Set(Some(crypto_currency.id));
    payment.crypto_amount = Set(Some(crypto_amount));
    payment.dest_wallet_id = Set(Some(wallet.id));

    let payment = payment_service::update(&socket_data.db, payment).await?;

    *socket_data.payment.lock().unwrap() = payment;

    session
        .text(
            WsOutputMessage::PaymentUpdated(socket_data.payment.lock().unwrap().clone()).into_str(),
        )
        .await
        .unwrap();

    let network = network_service::find_by_id(&socket_data.db, crypto_currency.network_id)
        .await?
        .ok_or(NotFoundError::NetworkNotFoundWithGivenId)?;

    let socket_data_clone = Arc::clone(&socket_data);
    let mut session = session.clone();

    // ***** start payment task *****

    let payment_task_handle = tokio::spawn(async move {
        log::info!(
            "Start subscribing transactions of payment with id: {}",
            socket_data.payment.lock().unwrap().id
        );

        // TODO: maybe remove this?
        let payment = socket_data.payment.lock().unwrap().clone();

        let transaction_result = web3_service::subscribe_transactions(
            &network.websocket_address_url,
            &wallet,
            payment.crypto_amount.unwrap(),
            payment.expired_at,
            &mut session,
            socket_data.db.clone(),
        )
        .await;

        if !transaction_result {
            session
                .text(WsOutputMessage::PaymentExpired(payment).into_str())
                .await
                .unwrap();
            // payment_exp_scheduler will free payment wallet and update it's status
            return;
        }

        wallet_service::free(&socket_data.db, payment.dest_wallet_id.unwrap())
            .await
            .unwrap();

        let mut payment = payment::ActiveModel::from(socket_data.payment.lock().unwrap().clone());
        payment.done_at = Set(Some(Utc::now().naive_utc()));
        payment.status = Set(PaymentStatus::Done);

        let payment = payment_service::update(&socket_data.db, payment)
            .await
            .unwrap();

        log::info!("Payment with id {} is done!", payment.id);

        session
            .text(WsOutputMessage::PaymentDone(payment).into_str())
            .await
            .unwrap();
    });

    *socket_data_clone.payment_task_handle.lock().unwrap() = Some(payment_task_handle);

    Ok(())
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(payment_ws_handshake);
}
