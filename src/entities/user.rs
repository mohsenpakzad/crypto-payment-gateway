//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.5

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(None)")]
pub enum UserRole {
    #[sea_orm(string_value = "USER")]
    User,
    #[sea_orm(string_value = "ADMIN")]
    Admin,
}

impl UserRole {
    pub fn to_role_str(&self) -> String {
        match self {
            UserRole::User => "ROLE_USER".to_owned(),
            UserRole::Admin => "ROLE_ADMIN".to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: UserRole,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::payment::Entity")]
    Payment,
    #[sea_orm(has_many = "super::user_transaction::Entity")]
    UserTransaction,
}

impl Related<super::payment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Payment.def()
    }
}

impl Related<super::user_transaction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserTransaction.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
