use crate::impl_crud;
use crate::{
    entities::{fiat_currency, prelude::*},
    errors::AppError,
};
use sea_orm::{DbConn, DeleteResult};

impl_crud!(FiatCurrency, fiat_currency, AppError, i32);
