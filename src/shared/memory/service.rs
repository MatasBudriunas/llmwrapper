use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct MemoryService {
    store: Arc<Mutex<HashMap<String, Vec<(String, String)>>>>,
}

impl MemoryService {
    pub fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn append(&self, session_id: &str, role: &str, content: &str) {
        let mut guard = self.store.lock().unwrap();
        let history = guard.entry(session_id.to_string()).or_default();
        history.push((role.to_string(), content.to_string()));
    }

    pub fn get_history(&self, session_id: &str, system_prompt: &str) -> String {
        let guard = self.store.lock().unwrap();
        let history = guard.get(session_id);

        let mut prompt = format!("{system_prompt}\n");

        if let Some(entries) = history {
            for (role, message) in entries {
                prompt.push_str(&format!("{role}: {message}\n"));
            }
        }

        prompt
    }
}
