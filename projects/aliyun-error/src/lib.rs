#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod convert;

mod display;

/// 常用第三方库
pub mod third_party {
    #[cfg(feature = "reqwest")]
    pub use reqwest;
}

/// The result type of this crate.
pub type Result<T> = std::result::Result<T, AliError>;

/// A boxed error kind, wrapping an [AliErrorKind].
#[derive(Clone)]
pub struct AliError {
    kind: Box<AliErrorKind>,
}

/// The kind of [AliError].
#[derive(Debug, Clone)]
pub enum AliErrorKind {
    /// An unknown error.
    UnknownError,
    ServiceError {
        message: String
    },
    NetworkError {
        message: String,
    },
    CustomError {
        message: String,
    }
}


impl AliError {
    pub fn custom_error(message: impl Into<String>) -> Self {
        AliError {
            kind: Box::new(AliErrorKind::CustomError { message: message.into() }),
        }
    }
}