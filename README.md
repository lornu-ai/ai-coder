# ai-coder

A blazingly fast CLI tool for AI-assisted coding using local Ollama models on your GPU.

> **New to ai-coder?** 👉 **[Start with the SETUP.md guide](./SETUP.md)** - Complete step-by-step instructions

## Features

- 🚀 **Fast Local Inference**: Run large language models directly on your GPU with Ollama
- 🔐 **Privacy-First**: All processing happens locally—no data sent to external APIs
- ⚡ **Streaming Output**: Real-time streaming responses as they're generated
- 🔧 **Configurable**: Choose your model and Ollama instance with ease
- 💰 **Free Forever**: No API costs, subscriptions, or limits

## Prerequisites

1. **Ollama** installed and running locally
   ```bash
   ollama serve
   ```

2. **A coding model** pulled (e.g., qwen2.5-coder, deepseek-coder-v2)
   ```bash
   ollama pull qwen2.5-coder
   ```

3. **Rust 1.70+** (for building from source)

## Installation

Clone and build:

```bash
git clone https://github.com/lornu-ai/ai-coder.git
cd ai-coder
cargo build --release
```

The binary will be available at `./target/release/ai-coder`.

## Usage

### Basic Example

```bash
./target/release/ai-coder "Write a fast Fibonacci sequence generator in Rust"
```

### Specify a Different Model

```bash
./target/release/ai-coder --model deepseek-coder-v2 "Implement a binary search algorithm"
```

### Custom Ollama Host

```bash
# Via command-line flag
./target/release/ai-coder -H http://192.168.1.50:11434 "Your prompt here"

# Via environment variable
OLLAMA_HOST="http://192.168.1.50:11434" ./target/release/ai-coder "Your prompt here"
```

### Config File (`.ai-coder.toml`)

Create a config file in your current directory:

```toml
model = "deepseek-coder-v2"
host = "http://localhost:11434"
```

Then run normally:

```bash
./target/release/ai-coder "Refactor this Rust module"
```

Or specify a custom config path:

```bash
./target/release/ai-coder --config ./configs/dev.toml "Your prompt here"
```

### Full Options

```bash
./target/release/ai-coder --help
```

## Architecture

- **Language**: Rust (async/await with Tokio)
- **HTTP Client**: Reqwest with streaming support
- **CLI Framework**: Clap for command-line argument parsing
- **Ollama Integration**: Local REST API calls to localhost:11434

## How It Works

1. Takes your prompt as a CLI argument
2. Connects to your local Ollama instance (default: http://localhost:11434)
3. Sends a streaming request to the model
4. Streams the output directly to your terminal in real-time
5. Exits when generation is complete

## Configuration

Precedence is:

1. Command-line flags
2. Environment variables (`OLLAMA_HOST`)
3. Config file (`.ai-coder.toml` or `--config` path)
4. Built-in defaults

### Environment Variables

- `OLLAMA_HOST`: Default Ollama instance URL (e.g., `http://localhost:11434`)

### Command-Line Flags

- `-m, --model <MODEL>`: Model name (default: `qwen2.5-coder`)
- `-H, --host <HOST>`: Ollama host URL (overrides `OLLAMA_HOST` env var)
- `--config <PATH>`: Optional config file path (default lookup: `./.ai-coder.toml`)

## Performance Tips

1. **GPU VRAM**: Models typically require 6-14GB VRAM. Check your GPU capacity.
2. **Model Selection**: Start with smaller models (7B) for faster iterations.
3. **Temperature**: For coding, lower temperature values produce more deterministic output.
4. **Context Length**: Larger context windows allow for more complex prompts.

## Roadmap

- [ ] Agentic loop support (auto-execute generated code)
- [ ] Project file context integration
- [ ] Bash command execution
- [x] Configuration file support
- [ ] Multi-turn conversation mode
- [ ] Code formatting and syntax highlighting

## License

MIT

## Contributing

Contributions welcome! Please open an issue or PR.

## Support

For Ollama issues: https://github.com/ollama/ollama
For ai-coder issues: https://github.com/lornu-ai/ai-coder/issues
