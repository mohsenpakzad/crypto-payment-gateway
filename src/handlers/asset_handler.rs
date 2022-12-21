use crate::entities::{
    crypto_currency, fiat_currency, network,
    wallet::{self, WalletStatus},
};
use crate::errors::AppError;
use crate::models::dtos::{CreateCryptoCurrency, CreateFiatCurrency, CreateNetwork, CreateWallet};
use crate::services::{
    crypto_currency_service, fiat_currency_service, network_service, wallet_service,
};
use actix_web::{
    get, post,
    web::{Data, ServiceConfig},
    HttpResponse, Responder,
};
use actix_web_grants::proc_macro::has_any_role;
use actix_web_validator::Json;
use sea_orm::{DbConn, Set};

#[get("/networks")]
async fn get_all_networks(db: Data<DbConn>) -> Result<impl Responder, AppError> {
    let networks = network_service::find_all(&db).await?;

    Ok(HttpResponse::Ok().json(networks))
}

#[post("/networks")]
#[has_any_role("ADMIN")]
async fn create_network(
    network: Json<CreateNetwork>,
    db: Data<DbConn>,
) -> Result<impl Responder, AppError> {
    let network = network::ActiveModel {
        name: Set(network.name.clone()),
        http_address_url: Set(network.http_address_url.clone()),
        websocket_address_url: Set(network.websocket_address_url.clone()),
        ..Default::default()
    };

    let network = network_service::create(&db, network).await?;
    Ok(HttpResponse::Created().json(network))
}

#[get("/crypto-currencies")]
async fn get_all_crypto_currencies(db: Data<DbConn>) -> Result<impl Responder, AppError> {
    let crypto_currencies = crypto_currency_service::find_all(&db).await?;

    Ok(HttpResponse::Ok().json(crypto_currencies))
}

#[post("/crypto-currencies")]
#[has_any_role("ADMIN")]
async fn create_crypto_currency(
    crypto_currency: Json<CreateCryptoCurrency>,
    db: Data<DbConn>,
) -> Result<impl Responder, AppError> {
    network_service::find_by_id(&db, crypto_currency.network_id)
        .await?
        .ok_or(AppError::NetworkNotFoundWithGivenId)?;

    let crypto_currency = crypto_currency::ActiveModel {
        name: Set(crypto_currency.name.clone()),
        symbol: Set(crypto_currency.symbol.clone()),
        network_id: Set(crypto_currency.network_id.clone()),
        ..Default::default()
    };

    let network = crypto_currency_service::create(&db, crypto_currency).await?;
    Ok(HttpResponse::Created().json(network))
}

#[get("/wallets")]
async fn get_all_wallets(db: Data<DbConn>) -> Result<impl Responder, AppError> {
    let wallets = wallet_service::find_all(&db).await?;

    Ok(HttpResponse::Ok().json(wallets))
}

#[post("/wallets")]
#[has_any_role("ADMIN")]
async fn create_wallet(
    wallet: Json<CreateWallet>,
    db: Data<DbConn>,
) -> Result<impl Responder, AppError> {
    network_service::find_by_id(&db, wallet.network_id)
        .await?
        .ok_or(AppError::NetworkNotFoundWithGivenId)?;

    let wallet = wallet::ActiveModel {
        address: Set(wallet.address.clone()),
        network_id: Set(wallet.network_id.clone()),
        status: Set(WalletStatus::Free),
        ..Default::default()
    };

    let network = wallet_service::create(&db, wallet).await?;
    Ok(HttpResponse::Created().json(network))
}

#[get("/fiat-currencies")]
async fn get_all_fiat_currencies(db: Data<DbConn>) -> Result<impl Responder, AppError> {
    let crypto_currencies = fiat_currency_service::find_all(&db).await?;

    Ok(HttpResponse::Ok().json(crypto_currencies))
}

#[post("/fiat-currencies")]
#[has_any_role("ADMIN")]
async fn create_fiat_currency(
    fiat_currency: Json<CreateFiatCurrency>,
    db: Data<DbConn>,
) -> Result<impl Responder, AppError> {
    let fiat_currency = fiat_currency::ActiveModel {
        name: Set(fiat_currency.name.clone()),
        symbol: Set(fiat_currency.symbol.clone()),
        ..Default::default()
    };

    let fiat_currency = fiat_currency_service::create(&db, fiat_currency).await?;
    Ok(HttpResponse::Created().json(fiat_currency))
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_all_networks)
        .service(create_network)
        .service(get_all_crypto_currencies)
        .service(create_crypto_currency)
        .service(get_all_wallets)
        .service(create_wallet)
        .service(get_all_fiat_currencies)
        .service(create_fiat_currency);
}
