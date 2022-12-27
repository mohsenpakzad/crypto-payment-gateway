use crate::errors::AppError;
use crate::security::jwt::Claims;
use crate::services::{payment_service, user_service, user_transaction_service};
use actix_web::web::ReqData;
use actix_web::{
    get,
    web::{Data, Path, ServiceConfig},
    HttpResponse, Responder,
};
use sea_orm::DbConn;

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

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_all_user_payments)
        .service(get_user_payment)
        .service(get_all_user_transactions)
        .service(get_user_transaction)
        .service(get_user_balance);
}
