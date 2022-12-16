use crate::entities::user;
use crate::errors::AppError;
use crate::models::dtos::{CreateUser, LoginUser};
use crate::security::{hash, jwt};
use crate::services::user_service;
use actix_web::{
    http::header,
    post,
    web::{Data, ServiceConfig},
    HttpResponse, Responder,
};
use actix_web_validator::Json;
use chrono::Utc;
use sea_orm::{DbConn, Set};

#[post("/signup")]
async fn signup(new_user: Json<CreateUser>, db: Data<DbConn>) -> Result<impl Responder, AppError> {
    user_service::find_by_username(&db, &new_user.username)
        .await?
        .map_or(Ok(()), |_| Err(AppError::UsernameAlreadyFound))?;

    let password_hash = hash::hash_password(&new_user.password);

    let user = user::ActiveModel {
        username: Set(new_user.username.clone()),
        password_hash: Set(password_hash),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    let user = user_service::create(&db, user).await?;

    Ok(HttpResponse::Created()
        .insert_header((header::AUTHORIZATION, jwt::generate_jwt(user.id)))
        .json(user))
}

#[post("/login")]
async fn login(login_user: Json<LoginUser>, db: Data<DbConn>) -> Result<impl Responder, AppError> {
    let user = user_service::find_by_username(&db, &login_user.username)
        .await?
        .ok_or(AppError::UserNotFoundWithGivenId)?;

    if !hash::verify_password(&user.password_hash, &login_user.password) {
        return Err(AppError::WrongPassword);
    }

    Ok(HttpResponse::Ok()
        .insert_header((header::AUTHORIZATION, jwt::generate_jwt(user.id)))
        .finish())
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(signup).service(login);
}
