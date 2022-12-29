use crate::entities::payment::PaymentStatus;
use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use sea_orm::prelude::Decimal;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PaymentError {
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
}

impl ResponseError for PaymentError {
    fn status_code(&self) -> StatusCode {
        match *self {
            PaymentError::PaymentIsNotBelongsToYou => StatusCode::UNAUTHORIZED,
            PaymentError::UserTransactionIsNotBelongsToYou => StatusCode::UNAUTHORIZED,
            PaymentError::PaymentIsDoneOrExpired(_) => StatusCode::BAD_REQUEST,
            PaymentError::PaymentShouldBeDone(_) => StatusCode::BAD_REQUEST,
            PaymentError::NotFreeWallet => StatusCode::IM_USED,
            PaymentError::NotEnoughBalance(_) => StatusCode::NOT_ACCEPTABLE,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}
