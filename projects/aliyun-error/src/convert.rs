use super::*;

impl From<AliErrorKind> for AliError {
    fn from(value: AliErrorKind) -> Self {
        Self {
            kind: Box::new(value),
        }
    }
}