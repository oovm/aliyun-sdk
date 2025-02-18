use crate::{
    application::{CompletionDebug, CompletionInput, CompletionParameters},
    Application,
};
use std::fmt::{Debug, Display, Formatter};

pub struct DashScope {
    pub(crate) client: reqwest::Client,
    pub(crate) api_key: String,
}

impl Debug for DashScope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("DashScope");
        s.field("api_key", &self.api_key);
        s.finish()
    }
}
impl Display for DashScope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("DashScope");
        s.field("api_key", &"<redacted>");
        s.finish()
    }
}
impl DashScope {
    pub fn new(key: impl Into<String>) -> Self {
        let client = reqwest::Client::new();
        DashScope { client, api_key: key.into() }
    }

    pub fn application(&self, id: impl Into<String>) -> Application {
        Application {
            host: self,
            id: id.into(),
            input: CompletionInput { prompt: "".to_string() },
            parameters: CompletionParameters::default(),
            debug: CompletionDebug {},
        }
    }
}
