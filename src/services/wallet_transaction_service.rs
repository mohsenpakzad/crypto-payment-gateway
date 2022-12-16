use crate::impl_crud;
use crate::{
    entities::{prelude::*, wallet_transaction},
    errors::AppError,
};
use sea_orm::{DbConn, DeleteResult};

impl_crud!(WalletTransaction, wallet_transaction, AppError, i32);
