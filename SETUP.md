# ai-coder Setup Guide

Complete step-by-step instructions to get ai-coder working on your machine.

## 📋 Prerequisites Checklist

Before you start, you need:

- [ ] **Ollama** - For running local AI models
- [ ] **Rust 1.70+** - For building the CLI
- [ ] **Git** - For cloning the repository
- [ ] **~5GB free disk space** - For the coding model

---

## ✅ Step 1: Install Ollama

### macOS
```bash
# Download and install Ollama
# Visit: https://ollama.ai/download
# Or use Homebrew if available:
brew install ollama
```

### Linux
```bash
curl -fsSL https://ollama.ai/install.sh | sh
```

### Windows
```bash
# Download installer from: https://ollama.ai/download
```

### Verify Installation
```bash
ollama --version
```

---

## ✅ Step 2: Install Rust (if not already installed)

Check if you have Rust:
```bash
rustc --version
cargo --version
```

If you don't have it, install:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify installation:
```bash
rustc --version
cargo --version
```

---

## ✅ Step 3: Clone the ai-coder Repository

```bash
# Clone the repository
git clone https://github.com/lornu-ai/ai-coder.git

# Navigate to the directory
cd ai-coder

# Verify you're in the right place
ls -la
# You should see: Cargo.toml, README.md, src/, .local-ci.toml
```

---

## ✅ Step 4: Build ai-coder

```bash
# Build in release mode (optimized for speed)
cargo build --release

# This may take 1-2 minutes on first build
# Watch for: "Finished `release` profile [optimized] target(s)"
```

### Where's the Binary?
```bash
# After building, the binary is at:
./target/release/ai-coder

# Verify it was built:
./target/release/ai-coder --version
```

---

## ✅ Step 5: Download a Coding Model

**Start Ollama service** (if not already running):
```bash
# Terminal 1: Start Ollama in the background
ollama serve
```

**In a new terminal, pull a model**:
```bash
# Terminal 2: Download the model (one-time, ~5GB)
ollama pull qwen2.5-coder

# Wait for completion. You'll see:
# "success" message at the end
```

### Alternative Models

If you want different models:
```bash
# Larger, more capable (14B - requires more VRAM)
ollama pull deepseek-coder-v2

# Smaller, lightweight (1.5B)
ollama pull starcoder2:1b
```

**Check available models:**
```bash
ollama list
```

---

## ✅ Step 6: Test Your Setup

```bash
# Make sure Ollama is still running (from Step 5)
# Then run:
./target/release/ai-coder "Write a hello world in Rust"

# You should see:
# [ai-coder] Using model: qwen2.5-coder
# [ai-coder] Connecting to: http://localhost:11434
# [ai-coder] ---
# (followed by generated code)
```

---

## 🎉 Success!

If you saw code generated, you're all set!

---

## 📍 Quick Command Reference

Once everything is installed:

```bash
# Basic usage
./target/release/ai-coder "Write a function that..."

# With custom model
./target/release/ai-coder -m deepseek-coder-v2 "Your prompt"

# With custom Ollama host
./target/release/ai-coder -H http://192.168.1.50:11434 "Your prompt"

# With config file in current directory (.ai-coder.toml)
./target/release/ai-coder "Your prompt"

# With custom config file path
./target/release/ai-coder --config ./configs/dev.toml "Your prompt"

# Get help
./target/release/ai-coder --help
```

---

## 💡 Optional: Add to PATH

To use `ai-coder` from anywhere without the full path:

```bash
# Copy binary to a directory in your PATH
sudo cp target/release/ai-coder /usr/local/bin/

# Now you can use it from anywhere:
ai-coder "Your prompt here"
```

---

## 🆘 Troubleshooting

### ❌ Error: "Connection refused"
**Problem**: Ollama service isn't running
**Solution**:
```bash
# Make sure Ollama is running
ollama serve
```

### ❌ Error: "Model not found"
**Problem**: You didn't download the model yet
**Solution**:
```bash
# Download the default model
ollama pull qwen2.5-coder

# Or download a different model
ollama pull deepseek-coder-v2
```

### ❌ Error: "Out of memory" or "CUDA out of memory"
**Problem**: Your GPU doesn't have enough VRAM
**Solution**: Use a smaller model
```bash
ollama pull starcoder2:1b
ai-coder -m starcoder2:1b "Your prompt"
```

### ❌ Build fails with "error: linker" or similar
**Problem**: Missing build dependencies
**Solution**:
```bash
# macOS
xcode-select --install

# Ubuntu/Debian
sudo apt-get install build-essential

# Then try building again
cargo build --release
```

---

## 🚀 Common Usage Patterns

### Pattern 0: Persistent Defaults via Config File
```toml
# .ai-coder.toml
model = "qwen2.5-coder"
host = "http://localhost:11434"
```

Run without passing model/host every time:
```bash
ai-coder "Generate a Rust parser for CSV with tests"
```

### Pattern 1: Quick Code Generation
```bash
ai-coder "Write a function that..."
```

### Pattern 2: Generate with Tests
```bash
ai-coder "Write a Rust function that validates email with comprehensive tests"
```

### Pattern 3: Debugging Help
```bash
ai-coder "Explain this Rust compiler error: [paste error message]"
```

### Pattern 4: Project Setup
```bash
ai-coder "Create a new Rust project structure for a CLI tool with these features: ..."
```

---

## 📚 Next Steps

1. **Review the main README.md** - Full feature overview
2. **Try examples** - Run some test prompts
3. **Read the roadmap** - See what's coming next
4. **Contribute** - Found a bug? Have an idea? Open an issue!

---

## 💬 Questions?

- **Ollama help**: https://github.com/ollama/ollama
- **Rust help**: https://www.rust-lang.org/
- **ai-coder issues**: https://github.com/lornu-ai/ai-coder/issues
