use crate::impl_crud;
use crate::{
    entities::{prelude::*, user_transaction},
    errors::AppError,
};
use sea_orm::{DbConn, DeleteResult};

impl_crud!(UserTransaction, user_transaction, AppError, i32);
