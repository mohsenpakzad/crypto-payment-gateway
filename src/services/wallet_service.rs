use crate::entities::wallet::WalletStatus;
use crate::impl_crud;
use crate::{
    entities::{prelude::*, wallet},
    errors::{InternalError, PaymentError},
};
use anyhow::Result;
use sea_orm::{ColumnTrait, DbConn, DeleteResult, EntityTrait, QueryFilter, Set};

impl_crud!(Wallet, wallet, InternalError, i32);

pub async fn reserve(db: &DbConn, network_id: i32) -> Result<wallet::Model> {
    let wallet = Wallet::find()
        .filter(wallet::Column::Status.eq(WalletStatus::Free))
        .filter(wallet::Column::NetworkId.eq(network_id))
        .one(db)
        .await
        .map_err(Into::<InternalError>::into)?
        .ok_or(PaymentError::NotFreeWallet)?;

    let mut wallet = wallet::ActiveModel::from(wallet);
    wallet.status = Set(WalletStatus::Busy);

    update(db, wallet).await.map_err(Into::into)
}

pub async fn free(db: &DbConn, id: i32) -> Result<wallet::Model> {
    let wallet = find_by_id(db, id)
        .await?
        .ok_or(PaymentError::NotFreeWallet)?;

    let mut wallet = wallet::ActiveModel::from(wallet);
    wallet.status = Set(WalletStatus::Free);

    update(db, wallet).await.map_err(Into::into)
}
