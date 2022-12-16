use crate::impl_crud;
use crate::{
    entities::{network, prelude::*},
    errors::AppError,
};
use sea_orm::{DbConn, DeleteResult};

impl_crud!(Network, network, AppError, i32);
