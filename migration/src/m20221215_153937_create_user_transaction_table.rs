use sea_orm_migration::prelude::*;

use crate::{
    m20221208_222429_create_user_table::User,
    m20221215_153841_create_fiat_currency_table::FiatCurrency,
    m20221215_153911_create_payment_table::Payment,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserTransaction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserTransaction::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserTransaction::UserId).integer().not_null())
                    .col(ColumnDef::new(UserTransaction::Typ).string().not_null())
                    .col(ColumnDef::new(UserTransaction::Amount).decimal().not_null())
                    .col(
                        ColumnDef::new(UserTransaction::FiatCurrencyId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserTransaction::CreatedAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserTransaction::DepositPaymentId)
                            .integer()
                            .unique_key(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserTransaction::Table, UserTransaction::UserId)
                            .to(User::Table, User::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserTransaction::Table, UserTransaction::FiatCurrencyId)
                            .to(FiatCurrency::Table, FiatCurrency::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserTransaction::Table, UserTransaction::DepositPaymentId)
                            .to(Payment::Table, Payment::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserTransaction::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum UserTransaction {
    Table,
    Id,
    UserId,
    Typ,
    Amount,
    FiatCurrencyId,
    CreatedAt,
    DepositPaymentId,
}
