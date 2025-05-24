use super::model::{ChatRequest, ValidatedChatRequest};
use crate::shared::abstract_validator::*;
const DEFAULT_MAX_TOKENS: u16 = 300;
const DEFAULT_TEMPERATURE: f32 = 0.7;

pub fn validate_chat_request(chat_request: &ChatRequest) -> Result<ValidatedChatRequest, String> {
    validate_required_str(&chat_request.prompt, "Prompt")?;
    validate_required_str(&chat_request.model, "Model")?;
    validate_required_str(&chat_request.session_id, "Session ID")?;

    Ok(ValidatedChatRequest {
        session_id: chat_request.session_id.clone(),
        model: chat_request.model.clone(),
        prompt: chat_request.prompt.clone(),
        max_tokens: chat_request.max_tokens.unwrap_or(DEFAULT_MAX_TOKENS),
        temperature: chat_request.temperature.unwrap_or(DEFAULT_TEMPERATURE),
        stream: chat_request.stream.unwrap_or(false),
    })
}
