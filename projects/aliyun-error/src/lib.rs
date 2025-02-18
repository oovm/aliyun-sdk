#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod display;
mod convert;

/// The result type of this crate.
pub type Result<T> = std::result::Result<T, AliError>;

/// A boxed error kind, wrapping an [AliErrorKind].
#[derive(Clone)]
pub struct AliError {
    kind: Box<AliErrorKind>,
}

/// The kind of [AliError].
#[derive(Debug, Copy, Clone)]
pub enum AliErrorKind {
    /// An unknown error.
    UnknownError
}


