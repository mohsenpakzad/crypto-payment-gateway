use sea_orm_migration::prelude::*;

use crate::m20221212_153800_create_network_table::Network;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Wallet::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Wallet::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Wallet::Address).string().not_null())
                    .col(ColumnDef::new(Wallet::NetworkId).integer().not_null())
                    .col(ColumnDef::new(Wallet::Status).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Wallet::Table, Wallet::NetworkId)
                            .to(Network::Table, Network::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Wallet::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Wallet {
    Table,
    Id,
    Address,
    NetworkId,
    Status,
}
