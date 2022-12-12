use crate::{
    entities::{prelude::*, user},
    errors::AppError,
};
use sea_orm::DbConn;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};

pub async fn find_by_username(
    db: &DbConn,
    username: &String,
) -> Result<Option<user::Model>, AppError> {
    Ok(User::find()
        .filter(user::Column::Username.eq(username.clone()))
        .one(db)
        .await
        .map_err(Into::into)?)
}

pub async fn create(db: &DbConn, user: user::ActiveModel) -> Result<user::Model, AppError> {
    Ok(user.insert(db).await.map_err(Into::into)?)
}
