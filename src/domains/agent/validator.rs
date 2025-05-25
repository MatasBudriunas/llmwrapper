use crate::shared::abstract_validator::*;
use super::model::{AgentCreateRequest, ValidatedAgentRequest};

const DEFAULT_MAX_TOKENS: u16 = 300;
const DEFAULT_TEMPERATURE: f32 = 0.7;

pub fn validate_agent_create_request(req: &AgentCreateRequest) -> Result<ValidatedAgentRequest, String> {
    validate_required_str(&req.model, "Model")?;
    validate_required_str(&req.prompt, "Prompt")?;

    Ok(ValidatedAgentRequest {
        model: req.model.clone(),
        prompt: req.prompt.clone(),
        max_tokens: req.max_tokens.unwrap_or(DEFAULT_MAX_TOKENS),
        temperature: req.temperature.unwrap_or(DEFAULT_TEMPERATURE),
    })
}
