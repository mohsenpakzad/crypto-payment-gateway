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
                    .table(CryptoCurrency::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CryptoCurrency::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(CryptoCurrency::Name).string().not_null())
                    .col(ColumnDef::new(CryptoCurrency::Symbol).string().not_null())
                    .col(
                        ColumnDef::new(CryptoCurrency::NetworkId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CryptoCurrency::Table, CryptoCurrency::NetworkId)
                            .to(Network::Table, Network::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CryptoCurrency::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum CryptoCurrency {
    Table,
    Id,
    Name,
    Symbol,
    NetworkId,
}
