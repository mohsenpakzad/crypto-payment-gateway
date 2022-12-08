use crate::config::AppConfig;
use actix_web::{middleware::Logger, web, App, HttpServer};

mod config;
mod dtos;
mod entities;
mod errors;
mod handlers;
mod security;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig::from_env().expect("Failed to load all server configurations");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(web::scope("/api") /* Add your apis here */)
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
