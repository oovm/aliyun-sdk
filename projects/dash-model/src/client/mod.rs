

use poem_openapi::__private::poem::http::header;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

pub enum DashError {
    Network { message: String },
}

impl Display for DashError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DashError::Network { message } => write!(f, "NetworkError: {}", message),
        }
    }
}

impl From<reqwest::Error> for DashError {
    fn from(value: Error) -> Self {
        Self::Network { message: value.to_string() }
    }
}

pub struct DashScope {
    client: reqwest::Client,
    api_key: String,
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

#[derive(Clone, Serialize)]
pub struct Application<'a> {
    #[serde(skip)]
    pub host: &'a DashScope,
    #[serde(skip)]
    pub id: String,
    pub input: CompletionInput,
    pub parameters: CompletionParameters,
    pub debug: CompletionDebug,
}

impl<'a> Debug for Application<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let debug = &mut f.debug_struct("Application");
        debug.field("url", &self.id);
        debug.field("input", &self.input);
        debug.field("parameters", &self.parameters);
        debug.field("debug", &self.debug);
        debug.finish()
    }
}

impl<'a> Application<'a> {
    pub async fn completion(&self) -> Result<ApplicationResponse, DashError> {
        let url = format!("https://dashscope.aliyuncs.com/api/v1/apps/{}/completion", self.id);
        let text: ApplicationResponse = self
            .host
            .client
            .post(url)
            .bearer_auth(&self.host.api_key)
            .header(header::CONTENT_TYPE, "application/json")
            .json(self)
            .send()
            .await?
            .json()
            .await?;
        Ok(text)
    }
    pub fn json_mode(&mut self) {
        self.parameters.response_format.format = "json_object";
    }
    pub fn text_mode(&mut self) {
        self.parameters.response_format.format = "text";
    }
    /// 设置 temperature 参数, temperature 有效值为 `[0, 2)`。
    pub fn set_temperature(&mut self, temperature: f32) {
        self.parameters.temperature = Some(temperature.clamp(0.0, 2.0));
    }
    /// 设置 seed 参数, seed 有效值为 `[0, 2^31-1]`。
    pub fn set_seed(&mut self, seed: u32) {
        self.parameters.seed = Some(seed % 2147483647);
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct CompletionInput {
    pub prompt: String,
}
#[derive(Clone, Debug, Serialize)]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    format: &'static str,
}
#[derive(Clone, Debug, Serialize)]
pub struct CompletionParameters {
    /// 采样温度，控制模型生成文本的多样性。
    pub temperature: Option<f32>,
    /// 设置seed参数会使文本生成过程更具有确定性，通常用于使模型每次运行的结果一致。
    ///
    /// 在每次模型调用时传入相同的 seed 值，并保持其他参数不变，模型将**尽可能**返回相同的结果。
    pub seed: Option<u32>,
    /// 本次请求返回的最大 Token 数, 默认值和最大值都表示模型的最大输出长度。
    ///
    /// `max_tokens` 的设置不会影响大模型的生成过程
    ///
    /// 如果模型生成的 Token 数超过 `max_tokens`，本次请求会返回截断后的内容。
    ///
    /// 关于各模型的最大输出长度，请参见[模型列表](https://help.aliyun.com/zh/model-studio/getting-started/models)。
    pub max_tokens: Option<u32>,
    /// 输出格式, 使用限制见 [格式化输出](https://help.aliyun.com/zh/model-studio/user-guide/json-mode)
    pub response_format: ResponseFormat,
}

impl Default for CompletionParameters {
    fn default() -> Self {
        Self { response_format: ResponseFormat { format: "text" }, temperature: None, seed: None, max_tokens: None }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct CompletionDebug {
    // Add your debug fields here
}

#[derive(Clone, Debug, Deserialize)]
pub struct DashModels {
    pub model_id: String,
    pub input_tokens: u64,
    pub output_tokens: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ApplicationResponse {
    request_id: String,
    #[serde(default)]
    code: String,
    #[serde(default)]
    message: String,
    output: Option<ApplicationOutput>,
    #[serde(default)]
    usage: ApplicationUsage,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ApplicationOutput {
    pub session_id: String,
    pub text: String,
    pub finish_reason: String,
}
#[derive(Clone, Debug, Default, Deserialize)]
pub struct ApplicationUsage {
    pub models: Vec<DashModels>,
}

impl ApplicationResponse {
    pub fn request_id(&self) -> &str {
        &self.request_id
    }
    pub fn usage(&self) -> &ApplicationUsage {
        &self.usage
    }
    pub fn models(&self) -> &[DashModels] {
        &self.usage.models
    }
    pub fn input_tokens(&self) -> u64 {
        self.usage.models.iter().map(|m| m.input_tokens).sum()
    }
    pub fn output_tokens(&self) -> u64 {
        self.usage.models.iter().map(|m| m.output_tokens).sum()
    }
    pub fn as_result(self) -> Result<ApplicationOutput, DashError> {
        match self.output {
            Some(s) => Ok(s),
            None => Err(DashError::Network { message: self.message }),
        }
    }
}
