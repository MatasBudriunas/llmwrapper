use axum::{extract::State, http::{HeaderMap, StatusCode}, Json, response::IntoResponse};
use std::sync::Arc;

use crate::{
    shared::{auth::validator::is_api_key_valid, ollama::service::OllamaService},
    domains::agent::{model::AgentCreateRequest, validator::validate_agent_create_request},
};

pub async fn handle_create_agent(
    State(service): State<Arc<OllamaService>>,
    headers: HeaderMap,
    Json(request): Json<AgentCreateRequest>,
) -> impl IntoResponse {
    if !is_api_key_valid(&headers) {
        return (StatusCode::UNAUTHORIZED, "Invalid API key").into_response();
    }

    let validated = match validate_agent_create_request(&request) {
        Ok(validated) => validated,
        Err(error) => return (StatusCode::BAD_REQUEST, error).into_response(),
    };

    let reply = match service
        .generate_personality(&validated.model, &validated.prompt, Some(validated.temperature), Some(validated.max_tokens))
        .await
    {
        Ok(result) => result,
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, err).into_response(),
    };

    (StatusCode::OK, Json(reply)).into_response()
}
