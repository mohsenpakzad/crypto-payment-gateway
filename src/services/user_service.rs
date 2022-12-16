use crate::impl_crud;
use crate::{
    entities::{prelude::*, user},
    errors::AppError,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use sea_orm::{DbConn, DeleteResult};

impl_crud!(User, user, AppError, i32);

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
