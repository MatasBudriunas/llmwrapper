use axum::{routing::post, Router};
use std::sync::Arc;

use crate::{
    domains::chat::controller::handle_chat_request,
    shared::ollama::service::OllamaService,
};

pub fn routes(state: Arc<OllamaService>) -> Router {
    Router::new()
        .route("/chat", post(handle_chat_request))
        .with_state(state)
}
