use axum::http::HeaderMap;

pub fn is_api_key_valid(headers: &HeaderMap) -> bool {
    let required_key = std::env::var("API_KEY").unwrap_or_else(|_| "dev-key".to_string());

    headers
        .get("X-API-KEY")
        .and_then(|v| v.to_str().ok())
        .map(|v| v == required_key)
        .unwrap_or(false)
}
