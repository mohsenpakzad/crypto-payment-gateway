use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum NotFoundError {
    #[error("User with given id doesn't exists")]
    UserNotFoundWithGivenId,

    #[error("Network with given id doesn't exists")]
    NetworkNotFoundWithGivenId,

    #[error("Fiat Currency with given id doesn't exists")]
    FiatCurrencyNotFoundWithGivenId,

    #[error("Crypto Currency with given id doesn't exists")]
    CryptoCurrencyNotFoundWithGivenId,

    #[error("Payment with given id doesn't exists")]
    PaymentNotFoundWithGivenId,

    #[error("Transaction with given id doesn't exists")]
    UserTransactionNotFoundWithGivenId,
}

impl ResponseError for NotFoundError {
    fn status_code(&self) -> StatusCode {
        StatusCode::NOT_FOUND
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}
