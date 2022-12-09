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
