use crate::impl_crud;
use crate::{
    entities::{crypto_currency, prelude::*},
    errors::InternalError,
};
use sea_orm::{DbConn, DeleteResult};

impl_crud!(CryptoCurrency, crypto_currency, InternalError, i32);
