pub mod generate;

use generate::{GenerateRequest, GenerateResponse};
use reqwest::{Client};
use tokio_stream::{self, StreamExt, Stream};


pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub async fn new(base_url: String) -> Self {
        ApiClient {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn generate(&self, request: GenerateRequest) ->  impl Stream<Item = Result<GenerateResponse, serde_json::Error>> {
        let url = format!("{}/api/generate", self.base_url);

        let stream = self.client.post(&url)
            .json(&request)
            .send()
            .await
            .expect("Request failed")
            .bytes_stream();

        stream.filter_map(|x| {
            match x {
                Ok(bytes) => Some(serde_json::from_slice::<GenerateResponse>(&bytes)),
                Err(_) => None,
            }
        })
    }
}
