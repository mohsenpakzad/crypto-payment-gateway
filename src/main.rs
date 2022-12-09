use crate::config::AppConfig;
use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use migration::DbErr;
use sea_orm::{ConnectOptions, Database, DbConn};

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

    let db = establish_db_connection(&config)
        .await
        .expect("Failed to setup the database");

    let db_data = web::Data::new(db);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(db_data.clone())
            .configure(handlers::auth_handler::config)
            .wrap(HttpAuthentication::bearer(security::jwt::validator))
            .service(web::scope("/api") /* Add your apis here */)
    })
    .bind((config.host, config.port))?
    .run()
    .await
}

pub async fn establish_db_connection(config: &AppConfig) -> Result<DbConn, DbErr> {
    let mut opt = ConnectOptions::new(config.database_url.clone());
    opt.sqlx_logging(false);

    Ok(Database::connect(opt).await?)
}
