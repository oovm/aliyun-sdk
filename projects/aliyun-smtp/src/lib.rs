#![doc = include_str!("../readme.md")]
mod helpers;
pub mod provider;

pub use crate::helpers::EmailSender;
pub use lettre;

pub use crate::provider::AlibabaSMTP;