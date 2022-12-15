pub use sea_orm_migration::prelude::*;

mod m20221208_222429_create_user_table;
mod m20221212_153800_create_network_table;
mod m20221212_153837_create_crypto_currency_table;
mod m20221212_153934_create_wallet_table;
mod m20221215_153723_create_wallet_transaction_table;
mod m20221215_153841_create_fiat_currency_table;
mod m20221215_153911_create_payment_table;
mod m20221215_153937_create_user_transaction_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221208_222429_create_user_table::Migration),
            Box::new(m20221212_153800_create_network_table::Migration),
            Box::new(m20221212_153837_create_crypto_currency_table::Migration),
            Box::new(m20221212_153934_create_wallet_table::Migration),
            Box::new(m20221215_153723_create_wallet_transaction_table::Migration),
            Box::new(m20221215_153841_create_fiat_currency_table::Migration),
            Box::new(m20221215_153911_create_payment_table::Migration),
            Box::new(m20221215_153937_create_user_transaction_table::Migration),
        ]
    }
}
