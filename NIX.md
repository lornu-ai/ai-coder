# Nix/NixOS Support for ai-coder

ai-coder includes full support for Nix and NixOS with Attic cache integration.

## Quick Start

### Using Nix Flakes

```bash
# Enter development environment
nix flake update
nix develop

# Build the project
nix build

# Run directly
nix run

# Run with custom prompt
nix run -- "Your prompt here"
```

### With direnv (Recommended)

If you have `direnv` installed:

```bash
# Automatic environment loading
direnv allow

# From now on, nix environment loads automatically when entering directory
cargo build --release
```

---

## Attic Cache Configuration

ai-coder uses the Attic cache at `https://nix-cache.stevedores.org/` for faster builds.

### Setup Attic Cache (Optional)

1. **Install attic CLI** (optional but recommended):
   ```bash
   nix profile install github:zhaofengli/attic/main
   ```

2. **Configure cache in `/etc/nix/nix.conf`** (requires sudo):
   ```ini
   [substituters](substituters)
   https://nix-cache.stevedores.org
   https://cache.nixos.org

   [trusted-public-keys](trusted-public-keys)
   nix-cache.stevedores.org:your-public-key-here
   cache.nixos.org-1:6NCHdD59X431o0gWypSTEBLYNZ8ZWag1HOAth-GOAD8=
   ```

3. **Or use environment variables** (simpler for development):
   ```bash
   export NIX_CONFIG="substituters = https://nix-cache.stevedores.org https://cache.nixos.org"
   ```

---

## Flake Structure

The `flake.nix` includes:

### Development Shell
- Latest stable Rust toolchain
- Cargo, rustfmt, clippy
- pkg-config and OpenSSL
- rust-analyzer (IDE support)
- LLDB (debugger)

### Package Definition
- Proper Rust/Cargo integration
- All dependencies managed by Nix
- Automatic caching with Attic

### Application
- Runnable via `nix run`
- Binary symlinked to `result/bin/ai-coder`

---

## Building

### Build Development Binary
```bash
nix develop
cargo build
```

### Build Release Binary
```bash
nix develop
cargo build --release
```

### Build with Nix (Direct)
```bash
nix build
```

Output: `./result/bin/ai-coder`

---

## Cache Management

### Push to Attic Cache

To push compiled artifacts to the shared cache:

```bash
# Build locally first
nix build

# Push to attic (if you have credentials)
attic push ai-coder ./result
```

### Pull from Cache

With the cache configured, Nix will automatically pull pre-built artifacts:

```bash
nix develop  # Uses cache automatically
```

---

## Troubleshooting

### Cache Not Found
If you see cache warnings:
```bash
# Just use default cache
nix develop --override-input nixpkgs github:nixos/nixpkgs/nixos-unstable
```

### Flake Lock Issues
```bash
# Update flake.lock
nix flake update

# Force fresh build
nix build --no-update-lock-file --refresh
```

### Out of Space
```bash
# Clean Nix store
nix store gc

# Or use nix-direnv for faster reloads
nix flake update
```

---

## Integration with Local-CI

The local-ci configuration works within the Nix environment:

```bash
nix develop
local-ci  # Runs all stages
```

---

## Resources

- **Nix Flakes**: https://wiki.nixos.org/wiki/Flakes
- **Attic**: https://github.com/zhaofengli/attic
- **direnv**: https://direnv.net/
- **Cache**: https://nix-cache.stevedores.org/

---

## Contributing

When contributing, Nix builds ensure reproducibility:

```bash
# Development
nix develop
git checkout -b feature/your-feature
# Make changes...
nix build  # Verify it builds

# Submit PR against develop
```
