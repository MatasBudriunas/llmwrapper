use axum::{routing::post, Router};
use super::controller;

pub fn routes() -> Router {
    Router::new().route(
        "/chat",
        post(controller::handle_chat_request)
    )
}
