use axum::{routing::post, Router};
use std::sync::Arc;

use crate::domains::chat::controller::handle_chat_request;
use crate::shared::memory::service::MemoryService;
use crate::shared::ollama::service::OllamaService;

pub fn routes() -> Router {
    let memory = MemoryService::new();
    let ollama = OllamaService::new(memory.clone()).unwrap();
    let shared_state = Arc::new(ollama);

    Router::new()
        .route("/chat", post(handle_chat_request))
        .with_state(shared_state)
}
