use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FiatCurrency::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FiatCurrency::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(FiatCurrency::Name).string().not_null())
                    .col(ColumnDef::new(FiatCurrency::Symbol).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FiatCurrency::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum FiatCurrency {
    Table,
    Id,
    Name,
    Symbol,
}
