use sea_orm_migration::prelude::*;

use crate::m20221212_153934_create_wallet_table::Wallet;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WalletTransaction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WalletTransaction::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(WalletTransaction::Hash).string().not_null())
                    .col(
                        ColumnDef::new(WalletTransaction::WalletId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WalletTransaction::CreatedAt)
                            .date_time()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(WalletTransaction::Table, WalletTransaction::WalletId)
                            .to(Wallet::Table, Wallet::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WalletTransaction::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum WalletTransaction {
    Table,
    Id,
    Hash,
    WalletId,
    CreatedAt,
}
