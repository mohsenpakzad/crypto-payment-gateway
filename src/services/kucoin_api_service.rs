use sea_orm::prelude::Decimal;
use serde_json::Value;

pub async fn fiat_to_crypto(
    fiat_symbol: &str,
    fiat_amount: Decimal,
    crypto_symbol: &str,
) -> Result<Decimal, reqwest::Error> {
    let res = reqwest::get(format!(
        "https://api.kucoin.com/api/v1/prices?base={fiat_symbol}&currencies={crypto_symbol}"
    ))
    .await?
    .json::<Value>()
    .await?;

    /*
    Response look like this:
    }
        "code": "200000",
        "data": {
            "BTC": "16696.16026711",
        }
    }
    */

    let crypto_fiat_value = res["data"]
        .as_object()
        .unwrap()
        .get(crypto_symbol)
        .unwrap()
        .as_str()
        .unwrap();

    let crypto_fiat_value = Decimal::from_str_exact(crypto_fiat_value).unwrap();

    Ok(fiat_amount / crypto_fiat_value)
}

pub async fn crypto_to_fiat(
    crypto_symbol: &str,
    crypto_amount: Decimal,
    fiat_symbol: &str,
) -> Result<Decimal, reqwest::Error> {
    let res = reqwest::get(format!(
        "https://api.kucoin.com/api/v1/prices?base={fiat_symbol}&currencies={crypto_symbol}"
    ))
    .await?
    .json::<Value>()
    .await?;

    /*
    Response look like this:
    }
        "code": "200000",
        "data": {
            "BTC": "16696.16026711",
        }
    }
    */

    let crypto_fiat_value = res["data"]
        .as_object()
        .unwrap()
        .get(crypto_symbol)
        .unwrap()
        .as_str()
        .unwrap();

    let crypto_fiat_value = Decimal::from_str_exact(crypto_fiat_value).unwrap();

    Ok(crypto_amount * crypto_fiat_value)
}
