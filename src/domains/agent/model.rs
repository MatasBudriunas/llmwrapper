use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AgentCreateRequest {
    pub api_key: String,
    pub model: String,
    pub prompt: String,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u16>,
}

#[derive(Debug)]
pub struct ValidatedAgentRequest {
    pub model: String,
    pub prompt: String,
    pub max_tokens: u16,
    pub temperature: f32,
}

#[derive(Debug, Serialize)]
pub struct AgentResponse {
    pub personality: String,
    pub model: String,
}
