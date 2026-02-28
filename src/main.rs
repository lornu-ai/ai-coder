mod agent;
mod cli;
mod github;
mod ollama;

use clap::Parser;
use futures_util::StreamExt;
use reqwest::Client;
use serde_json::json;
use std::env;
use std::io::{self, Write};

use crate::agent::extract_and_execute_commands;
use crate::cli::Args;
use crate::github::GitHubClient;
use crate::ollama::OllamaResponse;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::new();

    let mut prompt = args.prompt.clone();

    // 0. GitHub context fetching
    if args.github {
        if let Some(repo_full_name) = &args.repo {
            let parts: Vec<&str> = repo_full_name.split('/').collect();
            if parts.len() == 2 {
                let owner = parts[0];
                let repo = parts[1];

                if let Some(pr_number) = detect_pr_number(&prompt) {
                    eprintln!(
                        "[ai-coder] GitHub: Fetching PR #{} from {}...",
                        pr_number, repo_full_name
                    );
                    let github_client = GitHubClient::new(args.github_token.clone())?;
                    match github_client.get_pull_request(owner, repo, pr_number).await {
                        Ok(pr) => {
                            let context = format!(
                                "\n\n--- GitHub Context: PR #{} ---\nTitle: {}\nBody: {}\nState: {}\nBase: {}\nHead: {}\n--- End Context ---\n",
                                pr.number,
                                pr.title,
                                pr.body.unwrap_or_default(),
                                pr.state,
                                pr.base.ref_name,
                                pr.head.ref_name
                            );
                            prompt.push_str(&context);
                            eprintln!("[ai-coder] GitHub: Context added.");
                        }
                        Err(e) => {
                            eprintln!("[ai-coder] GitHub Warning: Could not fetch PR: {}", e);
                        }
                    }
                }
            } else {
                eprintln!("[ai-coder] GitHub Warning: Invalid repo format. Use 'owner/repo'.");
            }
        } else {
            eprintln!(
                "[ai-coder] GitHub Warning: --repo <owner/repo> is required for GitHub operations."
            );
        }
    }

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
        "prompt": prompt,
        "stream": true
    });

    // 3. Send the request to Ollama
    let response = client.post(&api_url).json(&request_body).send().await?;

    let mut stream = response.bytes_stream();
    let mut full_response = String::new();
    let mut buffer = Vec::new();

    // 4. Stream the output word-by-word to the terminal
    'outer: while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        buffer.extend_from_slice(&chunk);

        while let Some(pos) = buffer.iter().position(|&b| b == b'\n') {
            let line: Vec<u8> = buffer.drain(..=pos).collect();

            if let Ok(parsed) = serde_json::from_slice::<OllamaResponse>(&line) {
                if let Some(err) = parsed.error {
                    eprintln!("\n[ai-coder] Ollama Error: {}", err);
                    break 'outer;
                }

                print!("{}", parsed.response);
                full_response.push_str(&parsed.response);
                io::stdout().flush()?; // Ensure immediate rendering

                if parsed.done {
                    break 'outer;
                }
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

fn detect_pr_number(prompt: &str) -> Option<u32> {
    for word in prompt.split_whitespace() {
        if let Some(number_str) = word.strip_prefix('#') {
            if let Ok(number) = number_str.parse::<u32>() {
                return Some(number);
            }
        }
    }
    None
}
