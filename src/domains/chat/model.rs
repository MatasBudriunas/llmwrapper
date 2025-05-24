use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub api_key: String,
    pub model: String,
    pub prompt: String,
    pub session_id: String,
    pub max_tokens: Option<u16>,
    pub temperature: Option<f32>,
    pub stream: Option<bool>,
}

#[derive(Debug)]
pub struct ValidatedChatRequest {
    pub session_id: String,
    pub model: String,
    pub prompt: String,
    pub max_tokens: u16,
    pub temperature: f32,
    pub stream: bool,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub reply: String,
    pub model: String,
}
