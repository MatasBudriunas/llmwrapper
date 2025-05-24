use axum::{http::{HeaderMap, StatusCode}, response::IntoResponse, Json};
use super::model::ChatRequest;
use super::service;
use super::validator::validate_chat_request;
use crate::shared::auth::validator::is_api_key_valid;

pub async fn handle_chat_request(
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

    match service::send_prompt_to_ollama(validated_chat_request).await {
        Ok(chat_response) => (StatusCode::OK, Json(chat_response)).into_response(),
        Err(error_message) => (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response(),
    }
}
