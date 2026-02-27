use clap::Parser;
use futures_util::StreamExt;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::env;
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(
    name = "ai-coder",
    version = "0.1.0",
    about = "Local GPU-Accelerated AI Coding CLI",
    long_about = "A blazingly fast CLI tool for AI-assisted coding using local Ollama models on your GPU"
)]
struct Args {
    /// The coding prompt or question
    prompt: String,

    /// The model to use
    #[arg(short, long, default_value = "qwen2.5-coder")]
    model: String,

    /// Ollama host (can also be set via OLLAMA_HOST env var)
    #[arg(short = 'H', long)]
    host: Option<String>,
}

#[derive(Deserialize, Debug)]
struct OllamaResponse {
    response: String,
    done: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::new();

    // 1. Determine the Ollama host (CLI flag > env var > default)
    let host = args
        .host
        .or_else(|| env::var("OLLAMA_HOST").ok())
        .unwrap_or_else(|| "http://localhost:11434".to_string());

    // 2. Construct the full API URL
    let api_url = format!("{}/api/generate", host);

    eprintln!("[ai-coder] Using model: {}", args.model);
    eprintln!("[ai-coder] Connecting to: {}", host);
    eprintln!("[ai-coder] ---\n");

    let request_body = json!({
        "model": args.model,
        "prompt": args.prompt,
        "stream": true
    });

    // 3. Send the request to Ollama
    let response = client
        .post(&api_url)
        .json(&request_body)
        .send()
        .await?;

    let mut stream = response.bytes_stream();

    // 4. Stream the output word-by-word to the terminal
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;

        if let Ok(parsed) = serde_json::from_slice::<OllamaResponse>(&chunk) {
            print!("{}", parsed.response);
            io::stdout().flush()?; // Ensure immediate rendering

            if parsed.done {
                break;
            }
        }
    }

    println!("\n\n[ai-coder] Generation complete");
    Ok(())
}
