use crate::{
    domains::chat::model::{ValidatedChatRequest, ChatResponse},
    shared::memory::service::MemoryService,
};
use reqwest::Client;
use serde_json::json;
use std::{env, time::Instant};

pub struct OllamaService {
    client: Client,
    base_url: String,
    memory: MemoryService,
}

impl OllamaService {
    pub fn new(memory: MemoryService) -> Result<Self, String> {
        let base_url = env::var("OLLAMA_URL")
            .map_err(|_| "OLLAMA_URL env variable is not set".to_string())?;

        Ok(Self {
            client: Client::new(),
            base_url,
            memory,
        })
    }

    pub async fn generate(
        &self,
        request: &ValidatedChatRequest,
    ) -> Result<ChatResponse, String> {
        let system_persona = "You are Bitsy, a friendly blockchain guide from BitDegree.";

        self.memory.append(&request.session_id, "user", &request.prompt);

        let full_prompt = self.memory.get_history(&request.session_id, system_persona);

        let payload = json!({
            "model": request.model,
            "prompt": full_prompt,
            "session_id": request.session_id,
            "stream": request.stream,
            "temperature": request.temperature,
            "max_tokens": request.max_tokens
        });

        let started = Instant::now();
        let res = self
            .client
            .post(&self.base_url)
            .json(&payload)
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

        self.memory.append(&request.session_id, "assistant", &reply);

        Ok(ChatResponse {
            reply: format!("{reply} (responded in {duration} ms)"),
            model: request.model.clone(),
        })
    }
}
