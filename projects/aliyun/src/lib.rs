#![doc = include_str!("../readme.md")]
mod sdk;

pub use aliyun_error::{AliError, AliErrorKind, Result};
pub use aliyun_oss as oss;
pub use aliyun_sms as sms;
pub use aliyun_smtp as smtp;

pub struct AlibabaSDK {
    pub access_key_id: String,
    pub access_secret: String,
}
