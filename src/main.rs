mod agent;
mod cli;
mod github;
mod ollama;

use futures_util::StreamExt;
use reqwest::Client;
use serde_json::json;
use std::env;
use std::io::{self, Write};

use crate::agent::extract_and_execute_commands;
use crate::cli::Args;
use crate::ollama::OllamaResponse;

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

    // Initialize GitHub client if enabled
    let _github_client = if args.github {
        let key_path = args.github_app_key.unwrap_or_else(|| {
            "/Users/aivcs/engineering/code/creds/lornu-ai-bot.2026-01-15.private-key.pem"
                .to_string()
        });

        match github::GitHubAppAuth::from_private_key_file(args.github_app_id, &key_path) {
            Ok(app_auth) => {
                match app_auth
                    .get_installation_token(&client, args.github_installation_id)
                    .await
                {
                    Ok(token) => match github::GitHubClient::new(Some(token)) {
                        Ok(gh_client) => {
                            eprintln!("[ai-coder] GitHub integration: ENABLED (GitHub App)");
                            eprintln!("[ai-coder] App ID: {}", args.github_app_id);
                            eprintln!(
                                "[ai-coder] Installation ID: {}",
                                args.github_installation_id
                            );
                            Some(gh_client)
                        }
                        Err(e) => {
                            eprintln!("[ai-coder] GitHub client error: {}", e);
                            None
                        }
                    },
                    Err(e) => {
                        eprintln!("[ai-coder] Failed to get installation token: {}", e);
                        None
                    }
                }
            }
            Err(e) => {
                eprintln!("[ai-coder] GitHub App auth failed: {}", e);
                eprintln!("[ai-coder] Private key path: {}", key_path);
                None
            }
        }
    } else {
        None
    };

    eprintln!("[ai-coder] ---\n");

    let request_body = json!({
        "model": args.model,
        "prompt": args.prompt,
        "stream": true
    });

    // 3. Send the request to Ollama
    let response = client.post(&api_url).json(&request_body).send().await?;

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
        extract_and_execute_commands(&full_response, args.yes, args.allow_unsafe_exec)?;
    }

    eprintln!("[ai-coder] Complete");
    Ok(())
}
