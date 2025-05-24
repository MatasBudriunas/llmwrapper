use super::model::{ValidatedChatRequest, ChatResponse};
use serde_json::json;
use std::time::Instant;

pub async fn send_prompt_to_ollama(validated_chat_request: ValidatedChatRequest) -> Result<ChatResponse, String> {
    let ollama_url = std::env::var("OLLAMA_URL")
        .map_err(|_| "OLLAMA_URL env variable is not set".to_string())?;

    let client = reqwest::Client::new();

    let payload = json!({
        "model": validated_chat_request.model,
        "prompt": validated_chat_request.prompt,
        "stream": validated_chat_request.stream,
        "temperature": validated_chat_request.temperature,
        "max_tokens": validated_chat_request.max_tokens
    });

    let request_time = Instant::now();
    let response = client
        .post(ollama_url)
        .json(&payload)
        .send()
        .await
        .map_err(|error| format!("Failed to send request to Ollama: {}", error))?;

    let duration = request_time.elapsed().as_millis();

    let response_json: serde_json::Value = response
        .json()
        .await
        .map_err(|error| format!("Invalid JSON from Ollama: {}", error))?;

    let generated_text = response_json
        .get("response")
        .and_then(|value| value.as_str())
        .unwrap_or("No response from model.")
        .to_string();

    Ok(ChatResponse {
        reply: format!("{} (responded in {} ms)", generated_text, duration),
        model: validated_chat_request.model,
    })
}
