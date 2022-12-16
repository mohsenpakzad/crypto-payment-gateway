//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.5

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "crypto_currency")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub symbol: String,
    pub network_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::network::Entity",
        from = "Column::NetworkId",
        to = "super::network::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Network,
    #[sea_orm(has_many = "super::payment::Entity")]
    Payment,
}

impl Related<super::network::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Network.def()
    }
}

impl Related<super::payment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Payment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}