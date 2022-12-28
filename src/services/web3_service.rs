use crate::entities::{wallet, wallet_transaction};
use crate::models::ws::WsOutputMessage;
use crate::services::wallet_transaction_service;
use actix_web::web::Data;
use chrono::{NaiveDateTime, Utc};
use ethers::{prelude::*, types::U256};
use sea_orm::prelude::Decimal;
use sea_orm::{DbConn, Set};
use std::str::FromStr;
use std::sync::Arc;

pub async fn subscribe_transactions(
    websocket_url: &str,
    wallet: &wallet::Model,
    payment_crypto: Decimal,
    expiration_date: NaiveDateTime,
    session: &mut actix_ws::Session,
    db: Data<DbConn>,
) -> bool {
    let client = Provider::<Ws>::connect(websocket_url).await.unwrap();
    let client = Arc::new(client);

    let wallet_address = wallet.address.parse::<Address>().unwrap();

    let payment_crypto = convert_eth_to_wei(payment_crypto);
    log::info!(
        "Payment with amount of {payment_crypto} for wallet with address {wallet_address} started"
    );

    let mut transactions_stream = client.subscribe_pending_txs().await.unwrap();

    let mut payment_successful = false;

    let mut paid_crypto = U256::from_str("0").unwrap();
    while let Some(transaction_hash) = transactions_stream.next().await {
        if Utc::now().naive_utc() > expiration_date {
            log::info!("Payment is expired, unsubscribing...");
            break;
        }
        if let Ok(Some(transaction)) = client.get_transaction(transaction_hash).await {
            if transaction.to == Some(wallet_address) {
                log::info!("New transaction received for wallet address {wallet_address} : {transaction:#?}");

                // broadcast new transaction into socket
                session
                    .text(WsOutputMessage::TransactionReceived(transaction.clone()).into_str())
                    .await
                    .unwrap();

                // store new transaction into db
                let wallet_transaction = wallet_transaction::ActiveModel {
                    hash: Set(transaction_hash.to_string()), //TODO: use full format
                    wallet_id: Set(wallet.id),
                    created_at: Set(Utc::now().naive_utc()),
                    ..Default::default()
                };
                wallet_transaction_service::create(&db, wallet_transaction)
                    .await
                    .unwrap();

                //TODO: move broadcast and db logic to outside the function

                paid_crypto += transaction.value;

                log::info!("Crypto paid amount: {paid_crypto}");
            }

            if paid_crypto >= payment_crypto {
                payment_successful = true;
                break;
            }
        }
    }

    transactions_stream.unsubscribe().await.unwrap();

    payment_successful
}

fn convert_eth_to_wei(mut decimal: Decimal) -> U256 {
    decimal = decimal * Decimal::from_str_exact("1_000_000_000_000_000_000").unwrap();

    let decimal_str = decimal.to_string();
    let decimal_str = decimal_str.split(".").next().unwrap();

    U256::from_dec_str(decimal_str).unwrap()
}
