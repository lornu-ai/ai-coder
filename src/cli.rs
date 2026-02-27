use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "ai-coder",
    version = "0.1.0",
    about = "Local GPU-Accelerated AI Coding CLI",
    long_about = "A blazingly fast CLI tool for AI-assisted coding using local Ollama models on your GPU"
)]
pub struct Args {
    /// The coding prompt or question
    pub prompt: String,

    /// The model to use
    #[arg(short, long, default_value = "qwen2.5-coder")]
    pub model: String,

    /// Ollama host (can also be set via OLLAMA_HOST env var)
    #[arg(short = 'H', long)]
    pub host: Option<String>,

    /// Enable agent mode - automatically execute bash commands
    #[arg(short, long)]
    pub agent: bool,

    /// Auto-approve commands without confirmation in agent mode
    #[arg(short = 'y', long)]
    pub yes: bool,
}
