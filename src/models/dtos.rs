use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Clone, Debug, Validate)]
pub struct CreateUser {
    #[validate(length(min = 3))]
    pub username: String,

    #[validate(length(min = 3))]
    pub password: String,
}

#[derive(Deserialize, Clone, Debug, Validate)]
pub struct LoginUser {
    #[validate(length(min = 3))]
    pub username: String,

    #[validate(length(min = 3))]
    pub password: String,
}

#[derive(Deserialize, Clone, Debug, Validate)]
pub struct CreateNetwork {
    pub name: String,

    #[validate(url)]
    pub http_address_url: String,

    #[validate(url)]
    pub websocket_address_url: String,
}

#[derive(Deserialize, Clone, Debug, Validate)]
pub struct CreateCryptoCurrency {
    pub name: String,
    pub symbol: String,
    pub network_id: i32,
}

#[derive(Deserialize, Clone, Debug, Validate)]
pub struct CreateWallet {
    pub address: String,
    pub network_id: i32,
}

#[derive(Deserialize, Clone, Debug, Validate)]
pub struct CreateFiatCurrency {
    pub name: String,
    pub symbol: String,
}

