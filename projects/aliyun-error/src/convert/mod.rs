use super::*;

#[cfg(feature = "reqwest")]
mod for_reqwest;

impl From<AliErrorKind> for AliError {
    fn from(value: AliErrorKind) -> Self {
        Self { kind: Box::new(value) }
    }
}
