use axum::{routing::post, Router};
use std::sync::Arc;

use crate::{
    domains::agent::controller::handle_create_agent,
    shared::ollama::service::OllamaService,
};

pub fn routes(state: Arc<OllamaService>) -> Router {
    Router::new()
        .route("/agent", post(handle_create_agent))
        .with_state(state)
}
