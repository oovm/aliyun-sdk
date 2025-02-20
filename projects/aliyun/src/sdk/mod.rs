use crate::AlibabaSDK;
use aliyun_oss::oss::AlibabaOSS;
use aliyun_sms::aliyun::AlibabaSMS;
use aliyun_smtp::AliyunMailer;
use aliyun_smtp::lettre::error::Error;

impl AlibabaSDK {
    /// 创建 OSS SMS
    pub fn sms(&self) -> AlibabaSMS {
        AlibabaSMS { access_key_id: &self.access_key_id, access_secret: &self.access_secret }
    }
    /// 创建 OSS 实例
    pub fn oss(&self, endpoint: impl Into<String>, bucket: impl Into<String>) -> AlibabaOSS {
        AlibabaOSS {
            access_key_id: self.access_key_id.clone(),
            access_secret: self.access_secret.clone(),
            endpoint: endpoint.into(),
            bucket: bucket.into(),
        }
    }
    /// 创建 SMTP 实例
    pub fn smtp(&self, endpoint: impl Into<String>, bucket: impl Into<String>) -> Result<AliyunMailer, Error> {
        AliyunMailer::login(endpoint, bucket)
    }
}
