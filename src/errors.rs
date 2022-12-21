use crate::entities::payment::PaymentStatus;
use actix_web::{http::StatusCode, Error, HttpResponse, ResponseError};
use derive_more::Display;
use migration::DbErr;

#[derive(Debug, Display)]
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

    #[display(fmt = "Payment is done or expired, payment status: {}", _0)]
    PaymentIsDoneOrExpired(PaymentStatus),

    #[display(fmt = "Payment should be done to be verified, current status: {}", _0)]
    PaymentShouldBeDone(PaymentStatus),

    #[display(fmt = "There is no free wallet for your selected network, please try again later")]
    NotFreeWallet,

    // 404s
    #[display(fmt = "User with given id doesn't exists")]
    UserNotFoundWithGivenId,

    #[display(fmt = "Network with given id doesn't exists")]
    NetworkNotFoundWithGivenId,

    #[display(fmt = "Fiat Currency with given id doesn't exists")]
    FiatCurrencyNotFoundWithGivenId,

    #[display(fmt = "Payment with given id doesn't exists")]
    PaymentNotFoundWithGivenId,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::DataBaseError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::UsernameAlreadyFound => StatusCode::CONFLICT,
            AppError::WrongPassword => StatusCode::UNAUTHORIZED,
            AppError::PaymentIsNotBelongsToYou => StatusCode::UNAUTHORIZED,
            AppError::PaymentIsDoneOrExpired(_) => StatusCode::BAD_REQUEST,
            AppError::PaymentShouldBeDone(_) => StatusCode::BAD_REQUEST,
            AppError::NotFreeWallet => StatusCode::IM_USED,
            // 404s
            AppError::UserNotFoundWithGivenId => StatusCode::NOT_FOUND,
            AppError::NetworkNotFoundWithGivenId => StatusCode::NOT_FOUND,
            AppError::FiatCurrencyNotFoundWithGivenId => StatusCode::NOT_FOUND,
            AppError::PaymentNotFoundWithGivenId => StatusCode::NOT_FOUND,
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
