use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use migration::DbErr;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum InternalError {
    #[error("Database Error")]
    DatabaseError,
}

impl ResponseError for InternalError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

impl From<DbErr> for InternalError {
    fn from(value: DbErr) -> Self {
        log::error!("Database error: {value}");

        InternalError::DatabaseError
    }
}