use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Network::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Network::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Network::Name).string().not_null())
                    .col(ColumnDef::new(Network::HttpAddressUrl).string().not_null())
                    .col(
                        ColumnDef::new(Network::WebsocketAddressUrl)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Network::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Network {
    Table,
    Id,
    Name,
    HttpAddressUrl,
    WebsocketAddressUrl,
}
