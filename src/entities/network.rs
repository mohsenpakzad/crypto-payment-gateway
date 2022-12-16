//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.5

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "network")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    #[serde(skip_serializing)]
    pub http_address_url: String,
    #[serde(skip_serializing)]
    pub websocket_address_url: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::crypto_currency::Entity")]
    CryptoCurrency,
    #[sea_orm(has_many = "super::wallet::Entity")]
    Wallet,
}

impl Related<super::crypto_currency::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CryptoCurrency.def()
    }
}

impl Related<super::wallet::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Wallet.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
