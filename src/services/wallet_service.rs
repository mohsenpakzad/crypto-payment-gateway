use crate::impl_crud;
use crate::{
    entities::{prelude::*, wallet},
    errors::AppError,
};
use sea_orm::{DbConn, DeleteResult};

impl_crud!(Wallet, wallet, AppError, i32);
