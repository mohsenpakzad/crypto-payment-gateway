use crate::entities::payment::PaymentStatus;
use crate::impl_crud;
use crate::{
    entities::{payment, prelude::*},
    errors::AppError,
};
use actix_web::web::Data;
use chrono::Duration;
use sea_orm::{DbConn, DeleteResult, Set};

impl_crud!(Payment, payment, AppError, i32);

pub fn spawn_payment_exp_scheduler(run_after: Duration, payment_id: i32, db: Data<DbConn>) {
    tokio::spawn(async move {
        tokio::time::sleep(run_after.to_std().unwrap()).await;

        let payment = find_by_id(&db, payment_id).await.unwrap().unwrap();

        if payment.status == PaymentStatus::Waiting {
            log::info!("Payment with id {} is expired", payment_id);

            let mut payment = payment::ActiveModel::from(payment);
            payment.status = Set(PaymentStatus::Expired);

            update(&db, payment).await.unwrap();

            // TODO: If some money is paid, return it
        }
    });
}
