use crate::impl_crud;
use crate::{
    entities::{payment, prelude::*},
    errors::AppError,
};
use sea_orm::{DbConn, DeleteResult};

impl_crud!(Payment, payment, AppError, i32);
