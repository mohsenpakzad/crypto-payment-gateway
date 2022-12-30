use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use migration::DbErr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InternalError {
    #[error("Database Error")]
    DatabaseError(DbErr),
}

impl ResponseError for InternalError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body("Internal server error")
    }
}

impl From<DbErr> for InternalError {
    fn from(value: DbErr) -> Self {
        log::error!("Database error: {value}");

        InternalError::DatabaseError(value)
    }
}
