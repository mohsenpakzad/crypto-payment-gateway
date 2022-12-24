use crate::impl_crud;
use crate::{
    entities::{prelude::*, user_transaction},
    errors::AppError,
};
use sea_orm::{ColumnTrait, DbConn, DeleteResult, EntityTrait, QueryFilter};

impl_crud!(UserTransaction, user_transaction, AppError, i32);

pub async fn find_all_by_user_id(
    db: &DbConn,
    user_id: i32,
) -> Result<Vec<user_transaction::Model>, AppError> {
    Ok(UserTransaction::find()
        .filter(user_transaction::Column::UserId.eq(user_id))
        .all(db)
        .await
        .map_err(Into::into)?)
}
