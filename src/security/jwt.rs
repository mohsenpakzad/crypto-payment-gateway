use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::{
    bearer::{BearerAuth, Config},
    AuthenticationError,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
}

pub fn generate_jwt(user_id: i32) -> String {
    let claims = Claims {
        sub: user_id.to_string(),
        iat: Utc::now().timestamp(),
        exp: (Utc::now() + Duration::weeks(1)).timestamp(),
    };

    let token = encode(
        &Header::new(Algorithm::HS512),
        &claims,
        &EncodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_ref()),
    )
    .unwrap();

    format!("Bearer {token}")
}

fn verify_jwt(token: &str) -> Option<TokenData<Claims>> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_ref()),
        &Validation::new(Algorithm::HS512),
    )
    .ok()
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();
    let verify_res = verify_jwt(&token);

    if verify_res.is_some() {
        req.extensions_mut().insert(verify_res.unwrap().claims);
        return Ok(req);
    }

    let config = req
        .app_data::<Config>()
        .cloned()
        .unwrap_or_default()
        .scope("");
    Err((AuthenticationError::from(config).into(), req))
}
