use crate::impl_crud;
use crate::{
    entities::{fiat_currency, prelude::*},
    errors::InternalError,
};
use sea_orm::{DbConn, DeleteResult};

impl_crud!(FiatCurrency, fiat_currency, InternalError, i32);
