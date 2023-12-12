use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug)]
pub struct CopyRequest {
    source: String,
    destination: String,
}
