use crate::{AlibabaSDK, Result};
#[cfg(feature = "aliyun-oss")]
use aliyun_oss::oss::AlibabaOSS;
#[cfg(feature = "aliyun-sms")]
use aliyun_sms::aliyun::AlibabaSMS;
#[cfg(feature = "aliyun-smtp")]
use aliyun_smtp::AlibabaSMTP;

impl AlibabaSDK {
    /// 创建 OSS SMS
    #[cfg(feature = "aliyun-sms")]
    pub fn sms(&self) -> AlibabaSMS {
        AlibabaSMS { access_key_id: &self.access_key_id, access_secret: &self.access_secret }
    }
    /// 创建 OSS 实例
    #[cfg(feature = "aliyun-oss")]
    pub fn oss(&self, endpoint: impl Into<String>, bucket: impl Into<String>) -> AlibabaOSS {
        AlibabaOSS {
            access_key_id: self.access_key_id.clone(),
            access_secret: self.access_secret.clone(),
            endpoint: endpoint.into(),
            bucket: bucket.into(),
        }
    }
    /// 创建 SMTP 实例
    #[cfg(feature = "aliyun-smtp")]
    pub fn smtp(&self, endpoint: impl Into<String>, bucket: impl Into<String>) -> Result<AlibabaSMTP> {
        AlibabaSMTP::login(endpoint, bucket)
    }
}
