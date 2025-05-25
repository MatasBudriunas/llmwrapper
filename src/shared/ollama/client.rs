use reqwest::Client;
use serde_json::json;
use std::{env, time::Instant};

pub struct OllamaClient {
    client: Client,
    base_url: String,
}

impl OllamaClient {
    pub fn new() -> Result<Self, String> {
        let base_url = env::var("OLLAMA_URL")
            .map_err(|_| "OLLAMA_URL env variable is not set".to_string())?;

        Ok(Self {
            client: Client::new(),
            base_url,
        })
    }

    pub async fn send(
        &self,
        model: &str,
        prompt: &str,
        stream: bool,
        temperature: f32,
        max_tokens: u16,
        session_id: Option<&str>,
    ) -> Result<(String, u128), String> {
        let mut body = json!({
            "model": model,
            "prompt": prompt,
            "stream": stream,
            "temperature": temperature,
            "max_tokens": max_tokens
        });

        if let Some(sid) = session_id {
            body["session_id"] = json!(sid);
        }

        let started = Instant::now();
        let res = self
            .client
            .post(&self.base_url)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Failed to reach Ollama: {}", e))?;

        let duration = started.elapsed().as_millis();

        let json = res
            .json::<serde_json::Value>()
            .await
            .map_err(|e| format!("Invalid JSON: {}", e))?;

        let reply = json
            .get("response")
            .and_then(|v| v.as_str())
            .unwrap_or("No response from model.")
            .to_string();

        Ok((reply, duration))
    }
}
