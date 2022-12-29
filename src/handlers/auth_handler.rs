use crate::{
    entities::user,
    errors::{AuthError, NotFoundError},
    models::dtos::{CreateUser, LoginUser},
    security::{hash, jwt},
    services::user_service,
};
use actix_web::{
    http::header,
    post,
    web::{Data, ServiceConfig},
    Error, HttpResponse, Responder,
};
use actix_web_validator::Json;
use chrono::Utc;
use sea_orm::{DbConn, Set};

#[post("/signup")]
async fn signup(new_user: Json<CreateUser>, db: Data<DbConn>) -> Result<impl Responder, Error> {
    user_service::find_by_username(&db, &new_user.username)
        .await?
        .map_or(Ok(()), |_| Err(AuthError::UsernameAlreadyFound))?;

    let password_hash = hash::hash_password(&new_user.password);

    let user = user::ActiveModel {
        username: Set(new_user.username.clone()),
        password_hash: Set(password_hash),
        role: Set(user::UserRole::User),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    let user = user_service::create(&db, user).await?;

    Ok(HttpResponse::Created()
        .insert_header((header::AUTHORIZATION, jwt::generate_jwt(&user)))
        .json(user))
}

#[post("/login")]
async fn login(login_user: Json<LoginUser>, db: Data<DbConn>) -> Result<impl Responder, Error> {
    let user = user_service::find_by_username(&db, &login_user.username)
        .await?
        .ok_or(NotFoundError::UserNotFoundWithGivenId)?;

    if !hash::verify_password(&user.password_hash, &login_user.password) {
        return Err(AuthError::WrongPassword)?;
    }

    Ok(HttpResponse::Ok()
        .insert_header((header::AUTHORIZATION, jwt::generate_jwt(&user)))
        .finish())
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(signup).service(login);
}
