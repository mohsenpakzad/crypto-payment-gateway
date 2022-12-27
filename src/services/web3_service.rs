use crate::models::ws::WsOutputMessage;
use chrono::{NaiveDateTime, Utc};
use ethers::{prelude::*, types::U256};
use sea_orm::prelude::Decimal;
use std::str::FromStr;
use std::sync::Arc;

pub async fn subscribe_transactions(
    websocket_url: &str,
    wallet_address: &str,
    payment_crypto: Decimal,
    expiration_date: NaiveDateTime,
    session: &mut actix_ws::Session,
) -> bool {
    let client = Provider::<Ws>::connect(websocket_url).await.unwrap();
    let client = Arc::new(client);

    let wallet_address = wallet_address.parse::<Address>().unwrap();

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

                session
                    .text(WsOutputMessage::TransactionReceived(transaction.clone()).into_str())
                    .await
                    .unwrap();

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
