mod config;
mod entities;
mod errors;
mod handlers;
mod macros;
mod models;
mod security;
mod services;

use crate::config::AppConfig;
use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig::from_env().expect("Failed to load all server configurations");
    env_logger::init();

    let db = config
        .setup_db()
        .await
        .expect("Failed to setup the database");

    let jwt_encoding_key = config.create_jwt_encoding_key().await;
    let jwt_decoding_key = config.create_jwt_decoding_key().await;

    let db_data = web::Data::new(db);
    let jwt_encoding_key_data = web::Data::new(jwt_encoding_key);
    let jwt_decoding_key_data = web::Data::new(jwt_decoding_key);
    let config_data = web::Data::new(config.clone());

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .app_data(config_data.clone())
            .app_data(jwt_encoding_key_data.clone())
            .app_data(jwt_decoding_key_data.clone())
            .app_data(db_data.clone())
            .configure(handlers::auth_handler::config)
            .configure(handlers::ws_handler::config)
            .service(
                web::scope("/api")
                    .wrap(HttpAuthentication::with_fn(security::jwt::validator))
                    .configure(handlers::user_handler::config)
                    .configure(handlers::payment_handler::config)
                    .configure(handlers::asset_handler::config),
            )
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
