use super::model::{ChatRequest, ValidatedChatRequest};

const DEFAULT_MAX_TOKENS: u16 = 300;
const DEFAULT_TEMPERATURE: f32 = 0.7;

pub fn validate_chat_request(chat_request: &ChatRequest) -> Result<ValidatedChatRequest, String> {
    if chat_request.prompt.trim().is_empty() {
        return Err("Prompt cannot be empty.".to_string());
    }

    if chat_request.model.trim().is_empty() {
        return Err("Model cannot be empty.".to_string());
    }

    Ok(ValidatedChatRequest {
        model: chat_request.model.clone(),
        prompt: chat_request.prompt.clone(),
        max_tokens: chat_request.max_tokens.unwrap_or(DEFAULT_MAX_TOKENS),
        temperature: chat_request.temperature.unwrap_or(DEFAULT_TEMPERATURE),
        stream: chat_request.stream.unwrap_or(false),
    })
}
