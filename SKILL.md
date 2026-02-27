# ai-coder Skill Definition

## Overview

**ai-coder** is a local, GPU-accelerated AI-assisted coding CLI tool that runs large language models (LLMs) on your machine using Ollama. It enables private, offline AI-powered code generation without external API calls.

**Skill Type**: Code Generation & Assistance
**Language**: Rust
**Runtime**: Tokio async/await
**Model Backend**: Ollama REST API
**Privacy**: 100% local (no data sent to external APIs)

---

## Capabilities

### 1. **Prompt-Based Code Generation**
- Takes a natural language coding prompt as input
- Streams generated code directly to terminal in real-time
- Supports any model available via Ollama (qwen2.5-coder, deepseek-coder-v2, etc.)

**Example Usage:**
```bash
ai-coder "Write a fast Fibonacci sequence generator in Rust"
ai-coder "Implement binary search in Python"
ai-coder --model deepseek-coder-v2 "Create a web scraper for news articles"
```

### 2. **Configurable Model Selection**
- Default model: `qwen2.5-coder`
- Override via `--model` flag
- Override via `OLLAMA_MODEL` environment variable
- Supports any Ollama-compatible model

**Available Model Categories:**
- Coding-specific: qwen2.5-coder, deepseek-coder-v2, codegemma
- General: llama2, mistral, neural-chat
- Specialized: solar, orca-2

### 3. **Remote & Local Ollama Support**
- Default: `http://localhost:11434`
- Override via `--host` flag or `OLLAMA_HOST` environment variable
- Supports remote machine Ollama instances
- Compatible with cloud deployments (AWS, GCP, etc.)

**Example:**
```bash
# Local GPU
OLLAMA_HOST=http://localhost:11434 ai-coder "Your prompt"

# Remote GPU server
ai-coder --host http://192.168.1.50:11434 "Your prompt"

# AWS/GCP cloud instance
ai-coder -H http://gpu-instance.cloud.provider:11434 "Your prompt"
```

### 4. **Real-Time Streaming Output**
- Results stream to stdout character-by-character
- Real-time feedback while model is generating
- Proper terminal handling (no buffering)
- Clean exit markers and status messages

---

## Input/Output Specifications

### Input
- **Type**: String (command-line argument)
- **Required**: Yes (positional argument)
- **Format**: Natural language coding prompt
- **Max Length**: Depends on model context window (typically 4K-32K tokens)

### Output
- **Type**: Text (streamed to stdout)
- **Format**: Generated code, explanations, or both (model-dependent)
- **Timing**: Real-time streaming (not batched)
- **Status Messages**: Sent to stderr (model name, host, completion marker)

### Exit Codes
- `0`: Success
- `1`: Error (connection, parsing, model timeout, etc.)

---

## Integration Points

### 1. **As a CLI Tool**
```bash
./ai-coder "prompt" [--model MODEL] [--host HOST]
```

### 2. **Programmatically (Future)**
- Could be wrapped in scripts
- Could expose HTTP API for remote clients
- Could integrate with VS Code via extension

### 3. **In Agentic Workflows**
- Agent plans coding task
- Calls ai-coder with detailed prompt
- Parses generated code
- Executes/tests the code (future feature)
- Iterates if needed

---

## Configuration

### Environment Variables
```bash
OLLAMA_HOST     # Default Ollama instance (e.g., http://localhost:11434)
OLLAMA_MODEL    # Default model (currently must be set via --model flag)
```

### Command-Line Flags
```bash
-m, --model <MODEL>  # Model to use (default: qwen2.5-coder)
-H, --host <HOST>    # Ollama host URL (overrides OLLAMA_HOST)
--help               # Show help
--version            # Show version
```

---

## Performance Characteristics

### Latency
- **Model Load Time**: 0-30 seconds (depends on model size and GPU)
- **First Token**: 0.5-5 seconds (model warmup)
- **Throughput**: 10-100 tokens/second (varies by GPU and model)

### Resource Requirements
- **GPU VRAM**: 4-14GB (depending on model size)
- **CPU RAM**: 2-8GB
- **Disk Space**: 4-50GB per model
- **Network**: None (fully local)

### Model Recommendations
| Model | Size | VRAM | Speed | Quality |
|-------|------|------|-------|---------|
| qwen2.5-coder:7b | 7B | 6GB | Fast | Good |
| codegemma:7b | 7B | 6GB | Fast | Very Good |
| deepseek-coder-v2:16b | 16B | 10GB | Moderate | Excellent |
| mistral:7b | 7B | 6GB | Fast | Good |

---

## Current Limitations

### Known Constraints
1. **Single-turn**: No multi-turn conversation (each prompt is independent)
2. **No context**: Doesn't read project files or repository context
3. **No execution**: Generates code but doesn't execute it
4. **No formatting**: Output isn't automatically formatted or linted
5. **Basic streaming**: Simple line-based parsing, no syntax highlighting

### Future Roadmap
- [ ] Multi-turn conversation mode
- [ ] Project file context integration
- [ ] Code execution sandbox
- [ ] Automatic formatting and linting
- [ ] Agentic loop (plan/edit/test)
- [ ] Repository indexing and retrieval
- [ ] Syntax highlighting in output

---

## Use Cases

### 1. **Quick Code Snippets**
- Generate boilerplate code
- Create utility functions
- Build example implementations

**Example:**
```bash
ai-coder "Generate a struct for representing a 3D point in Rust"
```

### 2. **Algorithm Implementation**
- Learn or verify algorithm implementations
- Quick reference implementations
- Test different approaches

**Example:**
```bash
ai-coder "Implement quicksort in Python with comments"
```

### 3. **Debugging Assistance**
- Get suggestions for common errors
- Learn debugging techniques
- Understand edge cases

**Example:**
```bash
ai-coder "Why would this Rust code fail with a borrow checker error?"
```

### 4. **Learning & Reference**
- Understand language features
- Learn library usage
- Explore design patterns

**Example:**
```bash
ai-coder "Show an example of the builder pattern in Rust"
```

### 5. **Code Explanation**
- Understand existing code
- Learn best practices
- Improve code quality

**Example:**
```bash
ai-coder "Explain this async/await pattern in Rust"
```

---

## Error Handling

### Common Errors & Solutions

| Error | Cause | Solution |
|-------|-------|----------|
| "Failed to connect" | Ollama not running | `ollama serve` in another terminal |
| "Model not found" | Model not pulled | `ollama pull qwen2.5-coder` |
| "Timeout" | Model too large for GPU | Try smaller model or increase timeout |
| "Invalid JSON" | Ollama API response issue | Check Ollama version and logs |

---

## Testing & Quality

### Testing Strategy
- Unit tests for CLI argument parsing
- Integration tests with mock Ollama responses
- Manual testing with various models
- Performance benchmarking on different GPUs

### Code Quality
- Format: `cargo fmt`
- Linting: `cargo clippy -- -D warnings`
- Testing: `cargo test --release`
- Build: Multi-platform (Linux, macOS, Windows)

---

## Skill Metadata

```yaml
name: ai-coder
version: "0.1.0"
type: code-generation
language: rust
license: MIT
repository: https://github.com/lornu-ai/ai-coder
keywords:
  - ai
  - coding
  - ollama
  - local-inference
  - privacy-first
  - gpu-acceleration
dependencies:
  required:
    - rust: "1.70+"
    - ollama: "any"
  optional:
    - cuda: "for nvidia gpu"
    - rocm: "for amd gpu"
    - metal: "for apple silicon"
maturity: alpha
status: active-development
```

---

## Getting Started

### Installation
```bash
git clone https://github.com/lornu-ai/ai-coder.git
cd ai-coder
cargo build --release
./target/release/ai-coder "Your prompt here"
```

### First Steps
1. Ensure Ollama is running: `ollama serve`
2. Pull a model: `ollama pull qwen2.5-coder`
3. Run your first prompt: `ai-coder "Hello world in Python"`

### Advanced Setup
- See [SETUP.md](./SETUP.md) for detailed environment configuration
- See [README.md](./README.md) for feature overview
- See [NIX.md](./NIX.md) for reproducible builds

---

## Contributing

Contributions welcome! Areas for contribution:
- New model support and testing
- Performance optimizations
- Error handling and edge cases
- Documentation and examples
- Integration with other tools

See GitHub issues for planned work.

---

## License & Attribution

MIT License - See LICENSE file
Built by Lornu AI team
Uses Ollama for local LLM inference

