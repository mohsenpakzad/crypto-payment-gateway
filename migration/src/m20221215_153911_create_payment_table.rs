use sea_orm_migration::prelude::*;

use crate::{
    m20221208_222429_create_user_table::User,
    m20221212_153837_create_crypto_currency_table::CryptoCurrency,
    m20221212_153934_create_wallet_table::Wallet,
    m20221215_153841_create_fiat_currency_table::FiatCurrency,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Payment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Payment::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Payment::UserId).integer().not_null())
                    .col(ColumnDef::new(Payment::FiatCurrencyId).integer().not_null())
                    .col(ColumnDef::new(Payment::Amount).decimal().not_null())
                    .col(ColumnDef::new(Payment::CallbackUrl).string().not_null())
                    .col(ColumnDef::new(Payment::SellerOrderId).string().not_null())
                    .col(ColumnDef::new(Payment::Description).string())
                    .col(ColumnDef::new(Payment::PayerName).string())
                    .col(ColumnDef::new(Payment::PayerPhone).string())
                    .col(ColumnDef::new(Payment::PayerMail).string())
                    .col(ColumnDef::new(Payment::Status).string().not_null())
                    .col(ColumnDef::new(Payment::CryptoCurrencyId).integer())
                    .col(ColumnDef::new(Payment::DestWalletId).integer())
                    .col(ColumnDef::new(Payment::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Payment::ExpiredAt).date_time().not_null())
                    .col(ColumnDef::new(Payment::DoneAt).date_time())
                    .col(ColumnDef::new(Payment::VerifiedAt).date_time())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Payment::Table, Payment::UserId)
                            .to(User::Table, User::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Payment::Table, Payment::FiatCurrencyId)
                            .to(FiatCurrency::Table, FiatCurrency::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Payment::Table, Payment::CryptoCurrencyId)
                            .to(CryptoCurrency::Table, CryptoCurrency::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Payment::Table, Payment::DestWalletId)
                            .to(Wallet::Table, Wallet::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Payment::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Payment {
    Table,
    Id,
    UserId,
    FiatCurrencyId,
    Amount,
    CallbackUrl,
    SellerOrderId,
    Description,
    PayerName,
    PayerPhone,
    PayerMail,
    Status,
    CryptoCurrencyId,
    DestWalletId,
    CreatedAt,
    ExpiredAt,
    DoneAt,
    VerifiedAt,
}
