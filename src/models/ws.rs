use crate::entities::payment;
use derive_more::Display;
use ethers::types::Transaction;

#[derive(Debug)]
pub enum WsInputMessage {
    ChooseCrypto(i32),
}

#[derive(Debug, Display)]
pub enum WsInputMessageParseError {
    CommandNotFound,
    NoCommandArgument,
    BadCommandArgument,
}

impl TryFrom<&str> for WsInputMessage {
    type Error = WsInputMessageParseError;

    fn try_from(msg: &str) -> Result<Self, Self::Error> {
        if msg.starts_with('/') {
            let mut cmd_args = msg.splitn(2, ' ');

            // unwrap: we have guaranteed non-zero string length already
            match cmd_args.next().unwrap() {
                "/CHOOSE_CRYPTO" => {
                    if let Some(crypto_currency_id) = cmd_args.next() {
                        Ok(WsInputMessage::ChooseCrypto(
                            crypto_currency_id
                                .parse()
                                .map_err(|_| WsInputMessageParseError::BadCommandArgument)?,
                        ))
                    } else {
                        Err(WsInputMessageParseError::NoCommandArgument)
                    }
                }

                _ => Err(WsInputMessageParseError::CommandNotFound),
            }
        } else {
            Err(WsInputMessageParseError::CommandNotFound)
        }
    }
}

#[derive(Debug, Display)]
pub enum WsOutputMessage {
    #[display(fmt = "ERROR")]
    Error(anyhow::Error),

    #[display(fmt = "PAYMENT_UPDATED")]
    PaymentUpdated(payment::Model),

    #[display(fmt = "PAYMENT_DONE")]
    PaymentDone(payment::Model),

    #[display(fmt = "PAYMENT_EXPIRED")]
    PaymentExpired(payment::Model),

    #[display(fmt = "TRANSACTION_RECEIVED")]
    TransactionReceived(Transaction),
}

impl WsOutputMessage {
    pub fn into_str(self) -> String {
        let param = match self {
            WsOutputMessage::Error(ref err) => {
                let err = serde_error::Error::new(err.root_cause());
                serde_json::to_value(err).unwrap()
            }

            WsOutputMessage::PaymentUpdated(ref payment)
            | WsOutputMessage::PaymentDone(ref payment)
            | WsOutputMessage::PaymentExpired(ref payment) => {
                serde_json::to_value(payment).unwrap()
            }

            WsOutputMessage::TransactionReceived(ref transaction) => {
                serde_json::to_value(transaction).unwrap()
            }
        };
        format!("{} {}", self.to_string(), param)
    }
}
