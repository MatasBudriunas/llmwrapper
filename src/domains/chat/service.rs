use crate::{
    domains::chat::model::{ChatResponse, ValidatedChatRequest},
    shared::ollama::service::OllamaService,
};

pub async fn send_prompt_to_ollama(
    validated: ValidatedChatRequest,
) -> Result<ChatResponse, String> {
    OllamaService::new()?.generate(&validated).await
}
