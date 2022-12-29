use crate::entities::payment::PaymentStatus;
use actix_web::{http::StatusCode, Error, HttpResponse, ResponseError};
use migration::DbErr;
use sea_orm::prelude::Decimal;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum AppError {
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("Database Error")]
    DataBaseError,

    #[error("User with given username already exists")]
    UsernameAlreadyFound,

    #[error("Wrong password")]
    WrongPassword,

    #[error("This payment isn't belongs to you")]
    PaymentIsNotBelongsToYou,

    #[error("This transaction isn't belongs to you")]
    UserTransactionIsNotBelongsToYou,

    #[error("Payment is done or expired, payment status: {0}")]
    PaymentIsDoneOrExpired(PaymentStatus),

    #[error("Payment should be done to be verified, current status: {0}")]
    PaymentShouldBeDone(PaymentStatus),

    #[error("There is no free wallet for your selected network, please try again later")]
    NotFreeWallet,

    #[error(
        "There is not enough balance to withdrawal, withdrawable amount for this currency is: {0}"
    )]
    NotEnoughBalance(Decimal),

    // 404s
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

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            // 500s
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::DataBaseError => StatusCode::INTERNAL_SERVER_ERROR,
            // custom
            AppError::UsernameAlreadyFound => StatusCode::CONFLICT,
            AppError::WrongPassword => StatusCode::UNAUTHORIZED,
            AppError::PaymentIsNotBelongsToYou => StatusCode::UNAUTHORIZED,
            AppError::UserTransactionIsNotBelongsToYou => StatusCode::UNAUTHORIZED,
            AppError::PaymentIsDoneOrExpired(_) => StatusCode::BAD_REQUEST,
            AppError::PaymentShouldBeDone(_) => StatusCode::BAD_REQUEST,
            AppError::NotFreeWallet => StatusCode::IM_USED,
            AppError::NotEnoughBalance(_) => StatusCode::NOT_ACCEPTABLE,
            // 404s
            AppError::UserNotFoundWithGivenId
            | AppError::NetworkNotFoundWithGivenId
            | AppError::FiatCurrencyNotFoundWithGivenId
            | AppError::CryptoCurrencyNotFoundWithGivenId
            | AppError::PaymentNotFoundWithGivenId
            | AppError::UserTransactionNotFoundWithGivenId => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

impl Into<AppError> for Error {
    fn into(self) -> AppError {
        log::error!("Internal server error: {self}");

        AppError::InternalServerError
    }
}

impl Into<AppError> for DbErr {
    fn into(self) -> AppError {
        log::error!("Database error: {self}");

        AppError::DataBaseError
    }
}
