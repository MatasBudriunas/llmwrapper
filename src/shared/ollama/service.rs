use crate::{
    domains::chat::model::{ValidatedChatRequest, ChatResponse},
    domains::agent::model::AgentResponse,
    shared::{memory::service::MemoryService, ollama::client::OllamaClient},
};

pub struct OllamaService {
    memory: MemoryService,
    client: OllamaClient,
}

impl OllamaService {
    pub fn new(memory: MemoryService) -> Result<Self, String> {
        let client = OllamaClient::new()?;
        Ok(Self { memory, client })
    }

    pub async fn generate_chat(&self, req: &ValidatedChatRequest) -> Result<ChatResponse, String> {
        self.memory.append(&req.session_id, &req.agent, "user", &req.prompt);

        let full_prompt = &req.prompt;

        let (reply, _) = self
            .client
            .send(
                &req.model,
                full_prompt,
                req.stream,
                req.temperature,
                req.max_tokens,
                Some(&req.session_id),
            )
            .await?;

        self.memory.append(&req.session_id, &req.agent, "assistant", &reply);

        Ok(ChatResponse {
            reply,
            model: req.model.clone(),
        })
    }

    pub async fn generate_personality(
        &self,
        model: &str,
        prompt: &str,
        temperature: Option<f32>,
        max_tokens: Option<u16>,
    ) -> Result<AgentResponse, String> {
        let (reply, _) = self
            .client
            .send(
                model,
                prompt,
                false,
                temperature.unwrap_or(0.7),
                max_tokens.unwrap_or(300),
                None,
            )
            .await?;

        Ok(AgentResponse {
            personality: reply,
            model: model.to_string(),
        })
    }
}
