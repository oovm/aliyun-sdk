#![doc = include_str!("../readme.md")]
mod helpers;
pub mod provider;

pub use crate::{helpers::EmailSender, provider::AlibabaSMTP};
pub use aliyun_error::party_3rd::lettre;
