use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug)]
pub struct CreateRequest {
    pub name: String,
    pub modelfile: String,
    pub stream: Option<bool>,
    pub path: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CreateResponse {
    pub status: String
}
