use crate::{
    config::AppConfig,
    entities::payment::{self, PaymentStatus},
    errors::{NotFoundError, PaymentError},
    models::dtos::{CreatePayment, VerifyPayment},
    security::jwt::Claims,
    services::{fiat_currency_service, payment_service, user_service},
};
use actix_web::web::ReqData;
use actix_web::{
    get, post,
    web::{Data, ServiceConfig},
    Error, HttpResponse, Responder,
};
use actix_web_grants::proc_macro::has_any_role;
use actix_web_validator::Json;
use chrono::{Duration, Utc};
use sea_orm::{DbConn, Set};
use serde_json::json;

#[get("/payments")]
#[has_any_role("ADMIN")]
async fn get_all_payments(db: Data<DbConn>) -> Result<impl Responder, Error> {
    let payments = payment_service::find_all(&db).await?;

    Ok(HttpResponse::Ok().json(payments))
}

#[post("/payments")]
async fn create_payment(
    payment: Json<CreatePayment>,
    req_user: ReqData<Claims>,
    config: Data<AppConfig>,
    db: Data<DbConn>,
) -> Result<impl Responder, Error> {
    let user_id = req_user.sub.parse::<i32>().unwrap();

    let user = user_service::find_by_id(&db, user_id)
        .await?
        .ok_or(NotFoundError::UserNotFoundWithGivenId)?;

    fiat_currency_service::find_by_id(&db, payment.fiat_currency_id.clone())
        .await?
        .ok_or(NotFoundError::FiatCurrencyNotFoundWithGivenId)?;

    let payment_waiting_duration = Duration::minutes(10); //TODO: use config
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
        expired_at: Set(Utc::now().naive_utc() + payment_waiting_duration),
        ..Default::default()
    };
    let payment = payment_service::create(&db, payment).await?;

    payment_service::spawn_payment_exp_scheduler(payment_waiting_duration, payment.id, db);

    let payment_response = json!({
        "id": payment.id,
        "link": format!("{}/{}", config.payment_gateway_base_url, payment.id),
    });
    Ok(HttpResponse::Created().json(payment_response))
}

#[post("/payments/verify")]
async fn verify_payment(
    payment: Json<VerifyPayment>,
    req_user: ReqData<Claims>,
    db: Data<DbConn>,
) -> Result<impl Responder, Error> {
    let user_id = req_user.sub.parse::<i32>().unwrap();

    let payment = payment_service::find_by_id(&db, payment.id)
        .await?
        .ok_or(NotFoundError::PaymentNotFoundWithGivenId)?;

    let user = user_service::find_by_id(&db, user_id)
        .await?
        .ok_or(NotFoundError::UserNotFoundWithGivenId)?;

    if payment.user_id != user.id {
        return Err(PaymentError::PaymentIsNotBelongsToYou)?;
    }

    if payment.status != PaymentStatus::Done {
        return Err(PaymentError::PaymentShouldBeDone(payment.status))?;
    }

    let mut payment = payment::ActiveModel::from(payment);
    payment.status = Set(PaymentStatus::Verified);
    payment.verified_at = Set(Some(Utc::now().naive_utc()));

    let payment = payment_service::update(&db, payment).await?;
    log::info!("Payment with id {} is verified", payment.id);

    payment_service::spawn_crypto_seller(payment.clone(), db);

    Ok(HttpResponse::Ok().json(payment))
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(create_payment)
        .service(verify_payment)
        .service(get_all_payments);
}
