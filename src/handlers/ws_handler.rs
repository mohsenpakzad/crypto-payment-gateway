use crate::{
    entities::payment::{self, PaymentStatus},
    errors::AppError,
    models::ws::{WsInputMessage, WsOutputMessage},
    services::{
        crypto_currency_service, fiat_currency_service, kucoin_api_service, payment_service,
        wallet_service,
    },
};
use actix_web::{
    get,
    web::{Data, Path, Payload, ServiceConfig},
    HttpRequest, Responder,
};
use actix_ws::Message;
use futures_util::{
    future::{self, Either},
    StreamExt,
};
use sea_orm::{DbConn, Set};
use std::time::{Duration, Instant};
use tokio::{pin, task, time::interval};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[get("/ws/payments/{payment_id}")]
async fn payment_ws_handshake(
    path: Path<i32>,
    req: HttpRequest,
    body: Payload,
    db: Data<DbConn>,
) -> Result<impl Responder, AppError> {
    let payment_id = path.into_inner();

    let payment = payment_service::find_by_id(&db, payment_id)
        .await?
        .ok_or(AppError::PaymentNotFoundWithGivenId)?;

    if payment.status != PaymentStatus::Waiting {
        return Err(AppError::PaymentIsDoneOrExpired(payment.status));
    }

    let (response, session, msg_stream) = actix_ws::handle(&req, body).map_err(Into::into)?;

    // spawn websocket handler (and don't await it) so that the response is returned immediately
    task::spawn_local(payment_ws(payment, session, msg_stream, db));

    Ok(response)
}
/// Process messages received from the client, respond to ping messages, and monitor
/// connection health to detect network issues and free up resources.
pub async fn payment_ws(
    mut payment: payment::Model,
    mut session: actix_ws::Session,
    mut msg_stream: actix_ws::MessageStream,
    db: Data<DbConn>,
) {
    log::info!("connected to websocket");

    let mut last_heartbeat = Instant::now();
    let mut interval = interval(HEARTBEAT_INTERVAL);

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
                        let res = process_text_msg(&payment, &mut session, &text, &db).await;
                        if let Err(err) = res {
                            session
                                .text(WsOutputMessage::Error(err).into_str())
                                .await
                                .unwrap();
                        } else {
                            // if payment getting updated, replace new one
                            if let Some(new_payment) = res.unwrap() {
                                payment = new_payment;
                            }
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
    payment: &payment::Model,
    session: &mut actix_ws::Session,
    text: &str,
    db: &Data<DbConn>,
) -> Result<Option<payment::Model>, AppError> {
    let input_msg = WsInputMessage::try_from(text);

    if let Err(err) = input_msg {
        session.text(err.to_string()).await.unwrap();
        return Ok(None);
    }

    match input_msg.unwrap() {
        WsInputMessage::ChooseCrypto(crypto_currency_id) => {
            let crypto_currency = crypto_currency_service::find_by_id(&db, crypto_currency_id)
                .await?
                .ok_or(AppError::CryptoCurrencyNotFoundWithGivenId)?;

            let fiat_currency = fiat_currency_service::find_by_id(db, payment.fiat_currency_id)
                .await?
                .ok_or(AppError::FiatCurrencyNotFoundWithGivenId)?;

            let crypto_amount = kucoin_api_service::fiat_to_crypto(
                &fiat_currency.symbol,
                payment.amount,
                &crypto_currency.symbol,
            )
            .await
            .unwrap();

            if let Some(dest_wallet_id) = payment.dest_wallet_id {
                wallet_service::free(db, dest_wallet_id).await?;
            }

            let wallet = wallet_service::reserve(db, crypto_currency.network_id).await?;

            let mut payment = payment::ActiveModel::from(payment.clone());
            payment.crypto_currency_id = Set(Some(crypto_currency.id));
            payment.crypto_amount = Set(Some(crypto_amount));
            payment.dest_wallet_id = Set(Some(wallet.id));

            let payment = payment_service::update(db, payment).await?;

            session
                .text(WsOutputMessage::PaymentUpdated(payment.clone()).into_str())
                .await
                .unwrap();

            Ok(Some(payment))
        }
    }
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(payment_ws_handshake);
}
