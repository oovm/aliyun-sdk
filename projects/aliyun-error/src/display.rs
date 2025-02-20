use crate::{AliError, AliErrorKind};
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

impl Error for AliError {}

impl Debug for AliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.kind, f)
    }
}

impl Display for AliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

impl Display for AliErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AliErrorKind::ServiceError { message } => {
                write!(f, "ServiceError: {message}")
            }
            AliErrorKind::NetworkError { message } => {
                write!(f, "NetworkError: {message}")
            }
            AliErrorKind::CustomError { message } => {
                write!(f, "CustomError: {message}")
            }
            AliErrorKind::UnknownError => {
                write!(f, "UnknownError")
            }
        }
    }
}

impl AliError {
    pub fn network(message: impl Into<String>) -> Self {
        AliErrorKind::NetworkError { message: message.into() }.into()
    }
}
