use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use derive_more::Display;
use migration::DbErr;

use crate::entities::payment::PaymentStatus;

#[derive(Debug, Display)]
pub enum AppError {
    #[display(fmt = "Internal Server Error")]
    DataBaseError,

    #[display(fmt = "User with given username already exists")]
    UsernameAlreadyFound,

    #[display(fmt = "Wrong password")]
    WrongPassword,

    #[display(fmt = "User with given id doesn't exists")]
    UserNotFoundWithGivenId,

    #[display(fmt = "Network with given id doesn't exists")]
    NetworkNotFoundWithGivenId,

    #[display(fmt = "Fiat Currency with given id doesn't exists")]
    FiatCurrencyNotFoundWithGivenId,

    #[display(fmt = "There is no payment with such id or it isn't belongs to you")]
    BadPayment,

    #[display(fmt = "Payment should be done to be verified, current state: {}", _0)]
    PaymentCannotBeVerified(PaymentStatus),
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::DataBaseError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::UsernameAlreadyFound => StatusCode::CONFLICT,
            AppError::WrongPassword => StatusCode::UNAUTHORIZED,
            AppError::UserNotFoundWithGivenId => StatusCode::NOT_FOUND,
            AppError::NetworkNotFoundWithGivenId => StatusCode::NOT_FOUND,
            AppError::FiatCurrencyNotFoundWithGivenId => StatusCode::NOT_FOUND,
            AppError::BadPayment => StatusCode::BAD_REQUEST,
            AppError::PaymentCannotBeVerified(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

impl Into<AppError> for DbErr {
    fn into(self) -> AppError {
        log::error!("Database error: {self}");

        AppError::DataBaseError
    }
}
