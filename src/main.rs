mod cli;
mod ollama;
mod agent;

use clap::Parser;
use futures_util::StreamExt;
use reqwest::Client;
use serde_json::json;
use std::env;
use std::io::{self, Write};

use crate::cli::Args;
use crate::ollama::OllamaResponse;
use crate::agent::extract_and_execute_commands;

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

    let mode = if args.agent { "AGENT" } else { "CHAT" };
    eprintln!("[ai-coder] Mode: {}", mode);
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
    let mut full_response = String::new();

    // 4. Stream the output word-by-word to the terminal
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;

        if let Ok(parsed) = serde_json::from_slice::<OllamaResponse>(&chunk) {
            print!("{}", parsed.response);
            full_response.push_str(&parsed.response);
            io::stdout().flush()?; // Ensure immediate rendering

            if parsed.done {
                break;
            }
        }
    }

    println!("\n");

    // 5. If agent mode, extract and execute bash commands
    if args.agent {
        extract_and_execute_commands(&full_response, args.yes)?;
    }

    eprintln!("[ai-coder] Complete");
    Ok(())
}
