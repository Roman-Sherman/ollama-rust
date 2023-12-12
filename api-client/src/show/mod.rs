use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug)]
pub struct ShowRequest {
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct ShowResponse {
    pub license: String,
    pub modelfile: String,
    pub parameters: String,
    pub template: String,
}
