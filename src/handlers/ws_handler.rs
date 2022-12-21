use crate::{entities::payment::PaymentStatus, errors::AppError, services::payment_service};
use actix_web::{
    get,
    web::{Data, Path, Payload, ServiceConfig},
    HttpRequest, Responder,
};

use sea_orm::DbConn;

#[get("/ws/payment/{payment_id}")]
async fn payment_ws_handshake(
    path: Path<i32>,
    req: HttpRequest,
    body: Payload,
    db: Data<DbConn>,
) -> Result<impl Responder, AppError> {
    let payment_id = path.into_inner();

    let payment = payment_service::find_by_id(&db, payment_id)
        .await?
        .ok_or(AppError::PaymentNotFoundWithGivenId)?;

    if payment.status != PaymentStatus::Waiting {
        return Err(AppError::PaymentIsDoneOrExpired(payment.status));
    }

    let (response, session, msg_stream) = actix_ws::handle(&req, body).map_err(Into::into)?;

    // TODO: spawn websocket handler (and don't await it) so that the response is returned immediately

    Ok(response)
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(payment_ws_handshake);
}
