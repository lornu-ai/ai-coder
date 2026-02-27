use clap::Parser;
use futures_util::StreamExt;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::env;
use std::io::{self, Write};
use std::process::Command;

mod github;

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

    /// Enable agent mode - automatically execute bash commands
    #[arg(short, long)]
    agent: bool,

    /// Auto-approve commands without confirmation in agent mode
    #[arg(short = 'y', long)]
    yes: bool,

    /// Enable GitHub App integration for PR reviews and file reading
    #[arg(long)]
    github: bool,

    /// GitHub App ID (default: 2665041 for lornu-ai-bot)
    #[arg(long, default_value = "2665041")]
    github_app_id: u64,

    /// Path to GitHub App private key PEM file
    /// (default: /Users/aivcs/engineering/code/creds/lornu-ai-bot.2026-01-15.private-key.pem)
    #[arg(long)]
    github_app_key: Option<String>,

    /// GitHub App installation ID (from https://github.com/organizations/lornu-ai/settings/installations)
    #[arg(long, default_value = "104427264")]
    github_installation_id: u64,

    /// Repository in format owner/repo (auto-detected from git if not provided)
    #[arg(long)]
    repo: Option<String>,
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

    let mode = if args.agent { "AGENT" } else { "CHAT" };
    eprintln!("[ai-coder] Mode: {}", mode);
    eprintln!("[ai-coder] Using model: {}", args.model);
    eprintln!("[ai-coder] Connecting to: {}", host);

    // Initialize GitHub client if enabled
    let _github_client = if args.github {
        let key_path = args.github_app_key.unwrap_or_else(|| {
            "/Users/aivcs/engineering/code/creds/lornu-ai-bot.2026-01-15.private-key.pem".to_string()
        });

        match github::GitHubAppAuth::from_private_key_file(args.github_app_id, &key_path) {
            Ok(app_auth) => {
                match app_auth.get_installation_token(&client, args.github_installation_id).await {
                    Ok(token) => {
                        match github::GitHubClient::new(Some(token)) {
                            Ok(gh_client) => {
                                eprintln!("[ai-coder] GitHub integration: ENABLED (GitHub App)");
                                eprintln!("[ai-coder] App ID: {}", args.github_app_id);
                                eprintln!("[ai-coder] Installation ID: {}", args.github_installation_id);
                                Some(gh_client)
                            }
                            Err(e) => {
                                eprintln!("[ai-coder] GitHub client error: {}", e);
                                None
                            }
                        }
                    }
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

/// Extract bash code blocks and execute them
fn extract_and_execute_commands(response: &str, auto_approve: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut in_code_block = false;
    let mut code_block = String::new();
    let mut language = String::new();

    for line in response.lines() {
        // Detect code block start
        if line.trim().starts_with("```") {
            if in_code_block {
                // End of code block
                in_code_block = false;

                // Execute if it's a bash block
                if language.is_empty() || language.contains("bash") || language.contains("sh") {
                    eprintln!("\n[ai-coder-agent] Found bash command(s):");
                    eprintln!("{}", "=".repeat(60));
                    eprintln!("{}", code_block);
                    eprintln!("{}", "=".repeat(60));

                    if !auto_approve {
                        eprint!("\n[ai-coder-agent] Execute? (y/n): ");
                        io::stderr().flush()?;
                        let mut input = String::new();
                        io::stdin().read_line(&mut input)?;
                        if !input.trim().eq_ignore_ascii_case("y") {
                            eprintln!("[ai-coder-agent] Skipped.");
                            code_block.clear();
                            language.clear();
                            continue;
                        }
                    }

                    // Execute the command
                    execute_bash(&code_block)?;
                }
                code_block.clear();
                language.clear();
            } else {
                // Start of code block
                in_code_block = true;
                language = line.trim()[3..].to_string(); // Extract language identifier
            }
        } else if in_code_block {
            code_block.push_str(line);
            code_block.push('\n');
        }
    }

    Ok(())
}

/// Execute bash commands safely
fn execute_bash(script: &str) -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("\n[ai-coder-agent] Executing...");
    let output = Command::new("bash")
        .arg("-c")
        .arg(script)
        .output()?;

    // Print output
    if !output.stdout.is_empty() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }

    if !output.stderr.is_empty() {
        eprintln!("[ai-coder-agent] stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    if !output.status.success() {
        eprintln!("[ai-coder-agent] ⚠️  Command failed with status: {}", output.status);
    } else {
        eprintln!("[ai-coder-agent] ✓ Command succeeded");
    }

    Ok(())
}
