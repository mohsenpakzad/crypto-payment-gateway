use crate::entities::user_transaction::UserTransactionType;
use crate::impl_crud;
use crate::{
    entities::{prelude::*, user_transaction},
    errors::AppError,
};
use sea_orm::prelude::Decimal;
use sea_orm::{ColumnTrait, DbConn, DeleteResult, EntityTrait, QueryFilter};
use std::collections::{HashMap, HashSet};

impl_crud!(UserTransaction, user_transaction, AppError, i32);

pub async fn find_all_by_user_id(
    db: &DbConn,
    user_id: i32,
) -> Result<Vec<user_transaction::Model>, AppError> {
    Ok(UserTransaction::find()
        .filter(user_transaction::Column::UserId.eq(user_id))
        .all(db)
        .await
        .map_err(Into::into)?)
}

pub async fn get_user_balance(
    db: &DbConn,
    user_id: i32,
) -> Result<HashMap<i32, Decimal>, AppError> {
    let user_transactions = find_all_by_user_id(db, user_id).await?;
    let user_fiat_currencies_id = user_transactions
        .iter()
        .map(|tx| tx.fiat_currency_id)
        .collect::<HashSet<_>>();

    Ok(user_fiat_currencies_id
        .into_iter()
        .map(|fiat_currency_id| {
            let fiat_amount = user_transactions
                .iter()
                .filter(|&tx| tx.fiat_currency_id == fiat_currency_id)
                .map(|tx| match tx.typ {
                    UserTransactionType::Deposit => tx.amount,
                    UserTransactionType::Withdrawal => -tx.amount,
                })
                .sum::<Decimal>();

            (fiat_currency_id, fiat_amount)
        })
        .collect::<HashMap<_, _>>())
}
