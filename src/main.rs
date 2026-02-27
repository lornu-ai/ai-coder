use clap::Parser;
use futures_util::StreamExt;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

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
    #[arg(short, long)]
    model: Option<String>,

    /// Ollama host (can also be set via OLLAMA_HOST env var)
    #[arg(short = 'H', long)]
    host: Option<String>,

    /// Optional config file path (default: ./.ai-coder.toml)
    #[arg(long)]
    config: Option<PathBuf>,
}

#[derive(Deserialize, Debug)]
struct OllamaResponse {
    response: String,
    done: bool,
}

#[derive(Deserialize, Debug, Default)]
struct FileConfig {
    model: Option<String>,
    host: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EffectiveConfig {
    model: String,
    host: String,
}

fn load_file_config(path: &Path) -> Result<FileConfig, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: FileConfig = toml::from_str(&content)?;
    Ok(config)
}

fn resolve_config(
    args_model: Option<String>,
    args_host: Option<String>,
    env_host: Option<String>,
    file_config: Option<FileConfig>,
) -> EffectiveConfig {
    let file_model = file_config.as_ref().and_then(|config| config.model.clone());
    let file_host = file_config.and_then(|config| config.host);

    let model = args_model
        .or(file_model)
        .unwrap_or_else(|| "qwen2.5-coder".to_string());

    let host = args_host
        .or(env_host)
        .or(file_host)
        .unwrap_or_else(|| "http://localhost:11434".to_string());

    EffectiveConfig { model, host }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::new();

    let config_path = args
        .config
        .clone()
        .unwrap_or_else(|| PathBuf::from(".ai-coder.toml"));

    let file_config = if config_path.exists() {
        Some(load_file_config(&config_path)?)
    } else {
        None
    };

    let config = resolve_config(
        args.model,
        args.host,
        env::var("OLLAMA_HOST").ok(),
        file_config,
    );

    // 2. Construct the full API URL
    let api_url = format!("{}/api/generate", config.host);

    eprintln!("[ai-coder] Using model: {}", config.model);
    eprintln!("[ai-coder] Connecting to: {}", config.host);
    eprintln!("[ai-coder] ---\n");

    let request_body = json!({
        "model": config.model,
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

#[cfg(test)]
mod tests {
    use super::{resolve_config, FileConfig};

    #[test]
    fn cli_overrides_everything() {
        let resolved = resolve_config(
            Some("cli-model".to_string()),
            Some("http://cli-host:11434".to_string()),
            Some("http://env-host:11434".to_string()),
            Some(FileConfig {
                model: Some("file-model".to_string()),
                host: Some("http://file-host:11434".to_string()),
            }),
        );

        assert_eq!(resolved.model, "cli-model");
        assert_eq!(resolved.host, "http://cli-host:11434");
    }

    #[test]
    fn env_host_overrides_file_host() {
        let resolved = resolve_config(
            None,
            None,
            Some("http://env-host:11434".to_string()),
            Some(FileConfig {
                model: Some("file-model".to_string()),
                host: Some("http://file-host:11434".to_string()),
            }),
        );

        assert_eq!(resolved.model, "file-model");
        assert_eq!(resolved.host, "http://env-host:11434");
    }

    #[test]
    fn falls_back_to_defaults_without_overrides() {
        let resolved = resolve_config(None, None, None, None);

        assert_eq!(resolved.model, "qwen2.5-coder");
        assert_eq!(resolved.host, "http://localhost:11434");
    }
}
