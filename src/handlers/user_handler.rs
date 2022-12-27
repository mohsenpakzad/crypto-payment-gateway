use crate::entities::user_transaction;
use crate::entities::user_transaction::UserTransactionType;
use crate::errors::AppError;
use crate::models::dtos::BalanceWithdrawal;
use crate::security::jwt::Claims;
use crate::services::{
    fiat_currency_service, payment_service, user_service, user_transaction_service,
};
use actix_web::web::ReqData;
use actix_web::{
    get, post,
    web::{Data, Path, ServiceConfig},
    HttpResponse, Responder,
};
use actix_web_validator::Json;
use chrono::Utc;
use sea_orm::prelude::Decimal;
use sea_orm::{DbConn, Set};

#[get("/users/payments")]
async fn get_all_user_payments(
    req_user: ReqData<Claims>,
    db: Data<DbConn>,
) -> Result<impl Responder, AppError> {
    let user = user_service::find_by_id(&db, req_user.sub.parse().unwrap())
        .await?
        .ok_or(AppError::UserNotFoundWithGivenId)?;

    let user_payments = payment_service::find_all_by_user_id(&db, user.id).await?;

    Ok(HttpResponse::Ok().json(user_payments))
}

#[get("/users/payments/{id}")]
async fn get_user_payment(
    path: Path<i32>,
    req_user: ReqData<Claims>,
    db: Data<DbConn>,
) -> Result<impl Responder, AppError> {
    let payment_id = path.into_inner();

    let user = user_service::find_by_id(&db, req_user.sub.parse().unwrap())
        .await?
        .ok_or(AppError::UserNotFoundWithGivenId)?;

    let payment = payment_service::find_by_id(&db, payment_id)
        .await?
        .ok_or(AppError::PaymentNotFoundWithGivenId)?;

    if payment.user_id != user.id {
        return Err(AppError::PaymentIsNotBelongsToYou);
    }

    Ok(HttpResponse::Ok().json(payment))
}

#[get("/users/transactions")]
async fn get_all_user_transactions(
    req_user: ReqData<Claims>,
    db: Data<DbConn>,
) -> Result<impl Responder, AppError> {
    let user = user_service::find_by_id(&db, req_user.sub.parse().unwrap())
        .await?
        .ok_or(AppError::UserNotFoundWithGivenId)?;

    let user_payments = user_transaction_service::find_all_by_user_id(&db, user.id).await?;

    Ok(HttpResponse::Ok().json(user_payments))
}

#[get("/users/transactions/{id}")]
async fn get_user_transaction(
    path: Path<i32>,
    req_user: ReqData<Claims>,
    db: Data<DbConn>,
) -> Result<impl Responder, AppError> {
    let transaction_id = path.into_inner();

    let user = user_service::find_by_id(&db, req_user.sub.parse().unwrap())
        .await?
        .ok_or(AppError::UserNotFoundWithGivenId)?;

    let user_transaction = user_transaction_service::find_by_id(&db, transaction_id)
        .await?
        .ok_or(AppError::UserTransactionNotFoundWithGivenId)?;

    if user_transaction.user_id != user.id {
        return Err(AppError::UserTransactionIsNotBelongsToYou);
    }

    Ok(HttpResponse::Ok().json(user_transaction))
}

#[get("/users/balance")]
async fn get_user_balance(
    req_user: ReqData<Claims>,
    db: Data<DbConn>,
) -> Result<impl Responder, AppError> {
    let user = user_service::find_by_id(&db, req_user.sub.parse().unwrap())
        .await?
        .ok_or(AppError::UserNotFoundWithGivenId)?;

    let user_balance = user_transaction_service::get_user_balance(&db, user.id).await?;

    Ok(HttpResponse::Ok().json(user_balance))
}

#[post("/users/withdraw")]
async fn withdraw_balance(
    withdrawal: Json<BalanceWithdrawal>,
    req_user: ReqData<Claims>,
    db: Data<DbConn>,
) -> Result<impl Responder, AppError> {
    let user = user_service::find_by_id(&db, req_user.sub.parse().unwrap())
        .await?
        .ok_or(AppError::UserNotFoundWithGivenId)?;

    fiat_currency_service::find_by_id(&db, withdrawal.fiat_currency_id)
        .await?
        .ok_or(AppError::FiatCurrencyNotFoundWithGivenId)?;

    let user_balance = user_transaction_service::get_user_balance(&db, user.id).await?;

    let fiat_currency_balance = user_balance.get(&withdrawal.fiat_currency_id);

    if fiat_currency_balance.is_none() {
        return Err(AppError::NotEnoughBalance(
            Decimal::from_str_exact("0").unwrap(),
        ));
    }
    let fiat_currency_balance = fiat_currency_balance.unwrap();

    if withdrawal.amount > *fiat_currency_balance {
        return Err(AppError::NotEnoughBalance(fiat_currency_balance.clone()));
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
