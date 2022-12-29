use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("User with given username already exists")]
    UsernameAlreadyFound,

    #[error("Wrong password")]
    WrongPassword,
}

impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AuthError::UsernameAlreadyFound => StatusCode::CONFLICT,
            AuthError::WrongPassword => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}
