use crate::{
    entities::{user_transaction, user_transaction::UserTransactionType},
    errors::{NotFoundError, PaymentError},
    models::dtos::{BalanceWithdrawal, FiatBalance},
    security::jwt::Claims,
    services::{fiat_currency_service, payment_service, user_service, user_transaction_service},
};
use actix_web::web::ReqData;
use actix_web::{
    get, post,
    web::{Data, Path, ServiceConfig},
    Error, HttpResponse, Responder,
};
use actix_web_validator::Json;
use chrono::Utc;
use sea_orm::prelude::Decimal;
use sea_orm::{DbConn, Set};

#[get("/users/payments")]
async fn get_all_user_payments(
    req_user: ReqData<Claims>,
    db: Data<DbConn>,
) -> Result<impl Responder, Error> {
    let user = user_service::find_by_id(&db, req_user.sub.parse().unwrap())
        .await?
        .ok_or(NotFoundError::UserNotFoundWithGivenId)?;

    let user_payments = payment_service::find_all_by_user_id(&db, user.id).await?;

    Ok(HttpResponse::Ok().json(user_payments))
}

#[get("/users/payments/{id}")]
async fn get_user_payment(
    path: Path<i32>,
    req_user: ReqData<Claims>,
    db: Data<DbConn>,
) -> Result<impl Responder, Error> {
    let payment_id = path.into_inner();

    let user = user_service::find_by_id(&db, req_user.sub.parse().unwrap())
        .await?
        .ok_or(NotFoundError::UserNotFoundWithGivenId)?;

    let payment = payment_service::find_by_id(&db, payment_id)
        .await?
        .ok_or(NotFoundError::PaymentNotFoundWithGivenId)?;

    if payment.user_id != user.id {
        return Err(PaymentError::PaymentIsNotBelongsToYou)?;
    }

    Ok(HttpResponse::Ok().json(payment))
}

#[get("/users/transactions")]
async fn get_all_user_transactions(
    req_user: ReqData<Claims>,
    db: Data<DbConn>,
) -> Result<impl Responder, Error> {
    let user = user_service::find_by_id(&db, req_user.sub.parse().unwrap())
        .await?
        .ok_or(NotFoundError::UserNotFoundWithGivenId)?;

    let user_payments = user_transaction_service::find_all_by_user_id(&db, user.id).await?;

    Ok(HttpResponse::Ok().json(user_payments))
}

#[get("/users/transactions/{id}")]
async fn get_user_transaction(
    path: Path<i32>,
    req_user: ReqData<Claims>,
    db: Data<DbConn>,
) -> Result<impl Responder, Error> {
    let transaction_id = path.into_inner();

    let user = user_service::find_by_id(&db, req_user.sub.parse().unwrap())
        .await?
        .ok_or(NotFoundError::UserNotFoundWithGivenId)?;

    let user_transaction = user_transaction_service::find_by_id(&db, transaction_id)
        .await?
        .ok_or(NotFoundError::UserTransactionNotFoundWithGivenId)?;

    if user_transaction.user_id != user.id {
        return Err(PaymentError::UserTransactionIsNotBelongsToYou)?;
    }

    Ok(HttpResponse::Ok().json(user_transaction))
}

#[get("/users/balance")]
async fn get_user_balance(
    req_user: ReqData<Claims>,
    db: Data<DbConn>,
) -> Result<impl Responder, Error> {
    let user = user_service::find_by_id(&db, req_user.sub.parse().unwrap())
        .await?
        .ok_or(NotFoundError::UserNotFoundWithGivenId)?;

    let user_balance = user_transaction_service::get_user_balance(&db, user.id)
        .await?
        .into_iter()
        .map(|(fiat_currency_id, balance)| FiatBalance {
            fiat_currency_id,
            balance,
        })
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(user_balance))
}

#[post("/users/withdraw")]
async fn withdraw_balance(
    withdrawal: Json<BalanceWithdrawal>,
    req_user: ReqData<Claims>,
    db: Data<DbConn>,
) -> Result<impl Responder, Error> {
    let user = user_service::find_by_id(&db, req_user.sub.parse().unwrap())
        .await?
        .ok_or(NotFoundError::UserNotFoundWithGivenId)?;

    fiat_currency_service::find_by_id(&db, withdrawal.fiat_currency_id)
        .await?
        .ok_or(NotFoundError::FiatCurrencyNotFoundWithGivenId)?;

    let user_balance = user_transaction_service::get_user_balance(&db, user.id).await?;

    let fiat_currency_balance = user_balance.get(&withdrawal.fiat_currency_id);

    if fiat_currency_balance.is_none() {
        return Err(PaymentError::NotEnoughBalance(
            Decimal::from_str_exact("0").unwrap(),
        ))?;
    }
    let fiat_currency_balance = fiat_currency_balance.unwrap();

    if withdrawal.amount > *fiat_currency_balance {
        return Err(PaymentError::NotEnoughBalance(
            fiat_currency_balance.clone(),
        ))?;
    }

    let withdrawal_transaction = user_transaction::ActiveModel {
        user_id: Set(user.id),
        typ: Set(UserTransactionType::Withdrawal),
        amount: Set(withdrawal.amount),
        fiat_currency_id: Set(withdrawal.fiat_currency_id),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    let withdrawal_transaction =
        user_transaction_service::create(&db, withdrawal_transaction).await?;

    Ok(HttpResponse::Ok().json(withdrawal_transaction))
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_all_user_payments)
        .service(get_user_payment)
        .service(get_all_user_transactions)
        .service(get_user_transaction)
        .service(get_user_balance)
        .service(withdraw_balance);
}
