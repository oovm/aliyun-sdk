use crate::{AliError, AliErrorKind};
use lettre::{address::AddressError, error::Error};

impl From<lettre::transport::smtp::Error> for AliError {
    fn from(value: lettre::transport::smtp::Error) -> Self {
        let kind = AliErrorKind::NetworkError { message: value.to_string() };
        Self { kind: Box::new(kind) }
    }
}

impl From<Error> for AliError {
    fn from(value: Error) -> Self {
        let kind = AliErrorKind::ServiceError { message: value.to_string() };
        Self { kind: Box::new(kind) }
    }
}
impl From<AddressError> for AliError {
    fn from(value: AddressError) -> Self {
        let kind = AliErrorKind::DecoderError { format: "email".to_string(), message: value.to_string() };
        Self { kind: Box::new(kind) }
    }
}
