use crate::{AliError, AliErrorKind};
use reqwest::Error;

impl From<Error> for AliError {
    fn from(value: Error) -> Self {
        let kind = AliErrorKind::Network { message: value.to_string() };
        Self { kind: Box::new(kind) }
    }
}
