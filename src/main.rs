use crate::config::AppConfig;
use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;

mod config;
mod entities;
mod errors;
mod handlers;
mod models;
mod security;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig::from_env().expect("Failed to load all server configurations");
    env_logger::init();

    let db = config
        .setup_db()
        .await
        .expect("Failed to setup the database");

    let db_data = web::Data::new(db);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(db_data.clone())
            .configure(handlers::auth_handler::config)
            .service(
                web::scope("/api")
                    .wrap(HttpAuthentication::bearer(security::jwt::validator))
                    /* Add your apis here */
            )
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
