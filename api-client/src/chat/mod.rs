use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub format: Option<String>,
    pub options: Option<String>,
    pub stream: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct ChatResponse {
    pub model: String,
    pub created_at: String,
    pub message: ChatMessage,
    pub done: bool,
}
