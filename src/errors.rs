use crate::entities::payment::PaymentStatus;
use actix_web::{http::StatusCode, Error, HttpResponse, ResponseError};
use derive_more::Display;
use migration::DbErr;
use sea_orm::prelude::Decimal;
use serde::Serialize;

#[derive(Debug, Display, Serialize)]
pub enum AppError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "Database Error")]
    DataBaseError,

    #[display(fmt = "User with given username already exists")]
    UsernameAlreadyFound,

    #[display(fmt = "Wrong password")]
    WrongPassword,

    #[display(fmt = "This payment isn't belongs to you")]
    PaymentIsNotBelongsToYou,

    #[display(fmt = "This transaction isn't belongs to you")]
    UserTransactionIsNotBelongsToYou,

    #[display(fmt = "Payment is done or expired, payment status: {}", _0)]
    PaymentIsDoneOrExpired(PaymentStatus),

    #[display(fmt = "Payment should be done to be verified, current status: {}", _0)]
    PaymentShouldBeDone(PaymentStatus),

    #[display(fmt = "There is no free wallet for your selected network, please try again later")]
    NotFreeWallet,

    #[display(
        fmt = "There is not enough balance to withdrawal, withdrawable amount for this currency is: {}",
        _0
    )]
    NotEnoughBalance(Decimal),

    // 404s
    #[display(fmt = "User with given id doesn't exists")]
    UserNotFoundWithGivenId,

    #[display(fmt = "Network with given id doesn't exists")]
    NetworkNotFoundWithGivenId,

    #[display(fmt = "Fiat Currency with given id doesn't exists")]
    FiatCurrencyNotFoundWithGivenId,

    #[display(fmt = "Crypto Currency with given id doesn't exists")]
    CryptoCurrencyNotFoundWithGivenId,

    #[display(fmt = "Payment with given id doesn't exists")]
    PaymentNotFoundWithGivenId,

    #[display(fmt = "Transaction with given id doesn't exists")]
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
