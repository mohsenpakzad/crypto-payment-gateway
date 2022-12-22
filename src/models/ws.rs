use crate::{entities::payment, errors::AppError};
use derive_more::Display;

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
    Error(AppError),
    #[display(fmt = "PAYMENT_UPDATED")]
    PaymentUpdated(payment::Model),
}

impl WsOutputMessage {
    pub fn into_str(self) -> String {
        match self {
            WsOutputMessage::Error(ref err) => {
                format!(
                    "{} {}",
                    self.to_string(),
                    serde_json::to_value(err).unwrap()
                )
            }
            WsOutputMessage::PaymentUpdated(ref payment) => {
                format!(
                    "{} {}",
                    self.to_string(),
                    serde_json::to_value(payment).unwrap()
                )
            }
        }
    }
}
