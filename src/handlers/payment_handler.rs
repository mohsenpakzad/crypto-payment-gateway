use crate::config::AppConfig;
use crate::entities::payment::{self, PaymentStatus};
use crate::errors::AppError;
use crate::models::dtos::{CreatePayment, VerifyPayment};
use crate::security::jwt::Claims;
use crate::services::{fiat_currency_service, payment_service, user_service};
use actix_web::web::ReqData;
use actix_web::{
    post,
    web::{Data, ServiceConfig},
    HttpResponse, Responder,
};
use actix_web_validator::Json;
use chrono::{Duration, Utc};
use sea_orm::{DbConn, Set};
use serde_json::json;

#[post("/payment")]
async fn create_payment(
    payment: Json<CreatePayment>,
    req_user: ReqData<Claims>,
    config: Data<AppConfig>,
    db: Data<DbConn>,
) -> Result<impl Responder, AppError> {
    let user_id = req_user.sub.parse::<i32>().unwrap();

    let user = user_service::find_by_id(&db, user_id)
        .await?
        .ok_or(AppError::UserNotFoundWithGivenId)?;

    fiat_currency_service::find_by_id(&db, payment.fiat_currency_id.clone())
        .await?
        .ok_or(AppError::FiatCurrencyNotFoundWithGivenId)?;

    let payment = payment::ActiveModel {
        user_id: Set(user.id),
        fiat_currency_id: Set(payment.fiat_currency_id.clone()),
        amount: Set(payment.amount.clone()),
        callback_url: Set(payment.callback_url.clone()),
        seller_order_id: Set(payment.seller_order_id.clone()),
        description: Set(payment.description.clone()),
        payer_name: Set(payment.payer_name.clone()),
        payer_phone: Set(payment.payer_phone.clone()),
        payer_mail: Set(payment.payer_mail.clone()),
        status: Set(PaymentStatus::Waiting),
        created_at: Set(Utc::now().naive_utc()),
        expired_at: Set(Utc::now().naive_utc() + Duration::minutes(10)), //TODO: use config
        ..Default::default()
    };
    let payment = payment_service::create(&db, payment).await?;

    let payment_response = json!({
        "id": payment.id,
        "link": format!("{}/{}", config.base_payment_gateway_address, payment.id),
    });
    Ok(HttpResponse::Created().json(payment_response))
}

#[post("/payment/verify")]
async fn verify_payment(
    payment: Json<VerifyPayment>,
    req_user: ReqData<Claims>,
    db: Data<DbConn>,
) -> Result<impl Responder, AppError> {
    let user_id = req_user.sub.parse::<i32>().unwrap();

    let payment = payment_service::find_by_id(&db, payment.id)
        .await?
        .ok_or(AppError::BadPayment)?;

    let user = user_service::find_by_id(&db, user_id)
        .await?
        .ok_or(AppError::UserNotFoundWithGivenId)?;

    if payment.user_id != user.id {
        return Err(AppError::BadPayment);
    }

    if payment.status != PaymentStatus::Done {
        return Err(AppError::PaymentCannotBeVerified(payment.status));
    }

    let mut payment = payment::ActiveModel::from(payment);
    payment.status = Set(PaymentStatus::Verified);
    payment.verified_at = Set(Some(Utc::now().naive_utc()));

    let payment = payment_service::update(&db, payment).await?;
    Ok(HttpResponse::Ok().json(payment))
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(create_payment).service(verify_payment);
}

