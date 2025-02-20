#![doc = include_str!("../readme.md")]
mod sdk;

pub use aliyun_error::{AliError, AliErrorKind, Result};
#[cfg(feature = "aliyun-oss")]
pub use aliyun_oss as oss;
#[cfg(feature = "aliyun-sms")]
pub use aliyun_sms as sms;
#[cfg(feature = "aliyun-smtp")]
pub use aliyun_smtp as smtp;

pub struct AlibabaSDK {
    pub access_key: String,
    pub access_secret: String,
}
