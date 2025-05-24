use crate::{
    domains::chat::model::{ValidatedChatRequest, ChatResponse},
    shared::ollama::service::OllamaService,
};
use std::sync::Arc;

pub async fn send_prompt_to_ollama(
    validated: ValidatedChatRequest,
    service: Arc<OllamaService>,
) -> Result<ChatResponse, String> {
    service.generate(&validated).await
}
