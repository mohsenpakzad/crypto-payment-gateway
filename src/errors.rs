use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum AppError {
    #[display(fmt = "Internal Server Error")]
    DataBaseError,

    #[display(fmt = "User with given username already exists")]
    UsernameAlreadyFound,

    #[display(fmt = "Wrong password")]
    WrongPassword,

    #[display(fmt = "User with given id doesn't exists")]
    UserNotFoundWithGivenId,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::DataBaseError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::UsernameAlreadyFound => StatusCode::CONFLICT,
            AppError::WrongPassword => StatusCode::UNAUTHORIZED,
            AppError::UserNotFoundWithGivenId => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}
