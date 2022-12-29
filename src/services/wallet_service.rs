use crate::entities::wallet::WalletStatus;
use crate::impl_crud;
use crate::{
    entities::{prelude::*, wallet},
    errors::AppError,
};
use sea_orm::{ColumnTrait, DbConn, DeleteResult, EntityTrait, QueryFilter, Set};

impl_crud!(Wallet, wallet, AppError, i32);

pub async fn reserve(db: &DbConn, network_id: i32) -> Result<wallet::Model, AppError> {
    let wallet = Wallet::find()
        .filter(wallet::Column::Status.eq(WalletStatus::Free))
        .filter(wallet::Column::NetworkId.eq(network_id))
        .one(db)
        .await
        .map_err(Into::<AppError>::into)?
        .ok_or(AppError::NotFreeWallet)?;

    let mut wallet = wallet::ActiveModel::from(wallet);
    wallet.status = Set(WalletStatus::Busy);

    update(db, wallet).await
}

pub async fn free(db: &DbConn, id: i32) -> Result<wallet::Model, AppError> {
    let wallet = find_by_id(db, id).await?.unwrap();

    let mut wallet = wallet::ActiveModel::from(wallet);
    wallet.status = Set(WalletStatus::Free);

    update(db, wallet).await
}
