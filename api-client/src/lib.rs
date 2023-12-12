pub mod generate;
pub mod chat;
pub mod create;
pub mod show;
pub mod copy;

use chat::{ChatRequest, ChatResponse};
use copy::CopyRequest;
use create::{CreateRequest, CreateResponse};
use generate::{GenerateRequest, GenerateResponse};
use reqwest::Client;
use show::{ShowRequest, ShowResponse};
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

    pub async fn chat(&self, request: ChatRequest) ->  impl Stream<Item = Result<ChatResponse, serde_json::Error>> {
        let url = format!("{}/api/chat", self.base_url);

        let stream = self.client.post(&url)
            .json(&request)
            .send()
            .await
            .expect("Request failed")
            .bytes_stream();

        stream.filter_map(|x| {
            match x {
                Ok(bytes) => Some(serde_json::from_slice::<ChatResponse>(&bytes)),
                Err(_) => None,
            }
        })
    }

    pub async fn create(&self, request: CreateRequest) ->  impl Stream<Item = Result<CreateResponse, serde_json::Error>> {
        let url = format!("{}/api/create", self.base_url);

        let stream = self.client.post(&url)
            .json(&request)
            .send()
            .await
            .expect("Request failed")
            .bytes_stream();

        stream.filter_map(|x| {
            match x {
                Ok(bytes) => Some(serde_json::from_slice::<CreateResponse>(&bytes)),
                Err(_) => None,
            }
        })
    }

    // TODO - this isn't a stream and should be single response
    pub async fn show(&self, request: ShowRequest) ->  impl Stream<Item = Result<ShowResponse, serde_json::Error>> {
        let url = format!("{}/api/show", self.base_url);

        let stream = self.client.post(&url)
            .json(&request)
            .send()
            .await
            .expect("Request failed")
            .bytes_stream();

        stream.filter_map(|x| {
            match x {
                Ok(bytes) => Some(serde_json::from_slice::<ShowResponse>(&bytes)),
                Err(_) => None,
            }
        })
    }

    // TODO - this is just a 200 response. No response object
    pub async fn copy(&self, request: CopyRequest) ->  impl Stream<Item = Result<serde_json::Error>> {
        let url = format!("{}/api/copy", self.base_url);

        let stream = self.client.post(&url)
            .json(&request)
            .send()
            .await
            .expect("Request failed")
            .bytes_stream();

        stream.filter_map(|x| {
            match x {
                Ok(bytes) => Some(serde_json::from_slice::<ShowResponse>(&bytes)),
                Err(_) => None,
            }
        })
    }
}
