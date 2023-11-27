use std::{io::Write};

use api_client::generate::GenerateRequest;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    if let Err(e) = generate_completion("llama2:7b", "Why is the sky blue?").await {
        eprintln!("Error: {}", e);
    }
}

async fn generate_completion(model: &str, prompt: &str) -> Result<(), reqwest::Error> {
    let client = api_client::ApiClient::new("http://localhost:11434".to_string()).await;

    let request = GenerateRequest {
        model: model.to_string(),
        prompt: prompt.to_string(),
        stream: Some(true),
        raw: None,
    };

    let mut stream = client.generate(request).await;

    print!("Generating ...\n");
    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(response) => {
                print!("{}", response.response);
                std::io::stdout().flush().unwrap();
                if response.done {
                    println!();
                    break;
                }
            },
            Err(e) => eprintln!("Error while receiving chunk: {}", e),
        }
    }

    Ok(())
}

