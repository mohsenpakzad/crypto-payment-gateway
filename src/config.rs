use config::{Config, ConfigError};
use migration::DbErr;
use sea_orm::{ConnectOptions, Database, DbConn};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub base_payment_gateway_address: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();

        log::info!("Loading configurations");

        let config = Config::builder()
            .add_source(config::Environment::default())
            .build()?;

        let app_config: AppConfig = config.try_deserialize()?;
        Ok(app_config)
    }

    pub async fn setup_db(&self) -> Result<DbConn, DbErr> {
        log::info!("Setup database");

        let mut opt = ConnectOptions::new(self.database_url.clone());
        opt.sqlx_logging(false);

        Ok(Database::connect(opt).await?)
    }
}
