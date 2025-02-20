use super::*;

#[cfg(feature = "reqwest")]
mod for_reqwest;

impl From<AliErrorKind> for AliError {
    fn from(error: AliErrorKind) -> Self {
        Self { kind: Box::new(error) }
    }
}

impl From<std::io::Error> for AliError {
    fn from(error: std::io::Error) -> Self {
        let kind = AliErrorKind::ServiceError { message: error.to_string() };
        Self { kind: Box::new(kind) }
    }
}
