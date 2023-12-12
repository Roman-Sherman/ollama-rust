use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct GenerateRequest {
    pub model: String,
    pub prompt: String,
    pub foramt: Option<String>,
    pub options: Option<String>,
    pub system: Option<String>,
    pub template: Option<String>,
    pub context: Option<String>,
    pub stream: Option<bool>,
    pub raw: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct GenerateResponse {
    pub model: String,
    pub created_at: String,
    pub response: String,
    pub done: bool,
}
