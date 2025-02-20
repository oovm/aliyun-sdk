use crate::EmailSender;
use aliyun_error::party_3rd::lettre::{
    transport::smtp::{authentication::Credentials, response::Response}, Message,
    SmtpTransport,
    Transport,
};
use poem::{error::GetDataError, FromRequest, Request, RequestBody};

mod aliyun;

pub use self::aliyun::AlibabaSMTP;
