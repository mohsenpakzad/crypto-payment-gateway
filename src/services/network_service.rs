use crate::impl_crud;
use crate::{
    entities::{network, prelude::*},
    errors::InternalError,
};
use sea_orm::{DbConn, DeleteResult};

impl_crud!(Network, network, InternalError, i32);
