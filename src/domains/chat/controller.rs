use axum::{extract::State, http::{HeaderMap, StatusCode}, response::IntoResponse, Json};
use std::sync::Arc;
use crate::{
    domains::chat::{model::ChatRequest, validator::validate_chat_request, service},
    shared::{auth::validator::is_api_key_valid, ollama::service::OllamaService},
};

pub async fn handle_chat_request(
    State(service): State<Arc<OllamaService>>,
    headers: HeaderMap,
    Json(chat_request): Json<ChatRequest>,
) -> impl IntoResponse {
    if !is_api_key_valid(&headers) {
        return (StatusCode::UNAUTHORIZED, "Invalid API key").into_response();
    }

    let validated_chat_request = match validate_chat_request(&chat_request) {
        Ok(validated) => validated,
        Err(error_message) => return (StatusCode::BAD_REQUEST, error_message).into_response(),
    };

    match service::send_prompt_to_ollama(validated_chat_request, service).await {
        Ok(chat_response) => (StatusCode::OK, Json(chat_response)).into_response(),
        Err(error_message) => (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response(),
    }
}
