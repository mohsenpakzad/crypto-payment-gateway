use super::{
    crypto_currency_service, fiat_currency_service, kucoin_api_service, user_transaction_service,
};
use crate::entities::payment::PaymentStatus;
use crate::entities::user_transaction::{self, UserTransactionType};
use crate::impl_crud;
use crate::services::wallet_service;
use crate::{
    entities::{payment, prelude::*},
    errors::AppError,
};
use actix_web::web::Data;
use chrono::{Duration, Utc};
use sea_orm::{DbConn, DeleteResult, Set};

impl_crud!(Payment, payment, AppError, i32);

pub fn spawn_payment_exp_scheduler(run_after: Duration, payment_id: i32, db: Data<DbConn>) {
    tokio::spawn(async move {
        tokio::time::sleep(run_after.to_std().unwrap()).await;

        let payment = find_by_id(&db, payment_id).await.unwrap().unwrap();

        if payment.status == PaymentStatus::Waiting {
            log::info!("Payment with id {} is expired", payment_id);

            if let Some(dest_wallet_id) = payment.dest_wallet_id {
                wallet_service::free(&db, dest_wallet_id).await.unwrap();
            }

            let mut payment = payment::ActiveModel::from(payment);
            payment.status = Set(PaymentStatus::Expired);

            update(&db, payment).await.unwrap();

            // TODO: If some money is paid, return it
        }
    });
}

pub fn spawn_crypto_seller(payment: payment::Model, db: Data<DbConn>) {
    tokio::spawn(async move {
        let crypto = crypto_currency_service::find_by_id(&db, payment.crypto_currency_id.unwrap())
            .await
            .unwrap()
            .unwrap();

        let fiat = fiat_currency_service::find_by_id(&db, payment.fiat_currency_id)
            .await
            .unwrap()
            .unwrap();

        // simulate selling crypto...
        tokio::time::sleep(Duration::seconds(5).to_std().unwrap()).await;

        let fiat_value = kucoin_api_service::crypto_to_fiat(
            &crypto.symbol,
            payment.crypto_amount.unwrap(),
            &fiat.symbol,
        )
        .await
        .unwrap();

        // make payment status as finished
        let mut payment = payment::ActiveModel::from(payment);
        payment.status = Set(PaymentStatus::Finished);

        let payment = update(&db, payment).await.unwrap();

        log::info!("Payment with id {} is finished!", payment.id);

        // create user transaction
        let user_payment_transaction = user_transaction::ActiveModel {
            user_id: Set(payment.user_id),
            typ: Set(UserTransactionType::Deposit),
            amount: Set(fiat_value),
            fiat_currency_id: Set(payment.fiat_currency_id),
            created_at: Set(Utc::now().naive_utc()),
            deposit_payment_id: Set(Some(payment.id)),
            ..Default::default()
        };

        let user_payment_transaction =
            user_transaction_service::create(&db, user_payment_transaction)
                .await
                .unwrap();

        log::info!(
            "New user payment transaction: {:#?}",
            user_payment_transaction
        );
    });
}
