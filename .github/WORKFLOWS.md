# GitHub Actions Workflows

This document describes the automated CI/CD workflows for ai-coder.

## 🔄 CI Workflow (`ci.yml`)

Runs on every push to `main` and `develop` branches, and on all pull requests.

### Jobs:

1. **Check** - `cargo check` on multiple platforms
   - Runs on: Ubuntu, macOS, Windows
   - Verifies the code compiles without building

2. **Fmt** - `cargo fmt --check`
   - Ensures code follows Rust formatting standards
   - Runs on: Ubuntu

3. **Clippy** - `cargo clippy`
   - Static analysis and linting
   - Runs on: Ubuntu
   - Rejects any warnings with `-D warnings`

4. **Test** - `cargo test`
   - Runs all unit and integration tests
   - Runs on: Ubuntu, macOS, Windows

5. **Build** - `cargo build --release`
   - Creates optimized release binaries
   - Uploads artifacts for each platform
   - Runs on: Ubuntu, macOS, Windows

6. **Nix Build** - `nix build`
   - Builds using Nix for reproducibility
   - Runs on: Ubuntu (with Nix)
   - Verifies binary works with `--version`

### Artifacts

Build artifacts are available as GitHub Actions artifacts for download:
- `ai-coder-ubuntu-latest`
- `ai-coder-macos-latest`
- `ai-coder-windows-latest`

## 🚀 Release Workflow (`release.yml`)

Triggered when a tag matching `v*` is pushed (e.g., `v0.1.0`).

### Jobs:

1. **Create Release** - Creates a GitHub release
   - Automatically marks as pre-release if tag contains "alpha" or "beta"

2. **Build and Upload** - Builds and uploads binaries
   - Targets:
     - Linux x86_64
     - macOS x86_64 (Intel)
     - macOS ARM64 (Apple Silicon)
     - Windows x86_64
   - Each binary uploaded as release asset

3. **Nix Release** - Builds with Nix and uploads
   - Creates reproducible build
   - Uploads as `ai-coder-nix.tar.gz`

### Usage

```bash
# Create a release by tagging
git tag v0.2.0
git push origin v0.2.0

# Or trigger manually via GitHub Actions UI
```

## 🔒 Security Workflow (`security.yml`)

Runs on push to main/develop, PRs, and weekly on Sunday.

### Jobs:

1. **Audit** - `cargo audit` via RustSec
   - Checks for known security vulnerabilities in dependencies
   - Fails if vulnerabilities found
   - Uses [rustsec/audit-check-action](https://github.com/rustsec/audit-check-action)

2. **Dependency Check** - Checks for outdated dependencies
   - Lists outdated packages
   - Does not fail (informational only)

3. **Security Audit** - Weekly vulnerability scanning
   - Same as Audit job but scheduled weekly
   - Continues on error (informational for trending)
   - Helps track vulnerability patterns over time

## 📊 Status Badges

Add these to your README.md to show workflow status:

```markdown
[![CI](https://github.com/lornu-ai/ai-coder/actions/workflows/ci.yml/badge.svg)](https://github.com/lornu-ai/ai-coder/actions/workflows/ci.yml)
[![Security](https://github.com/lornu-ai/ai-coder/actions/workflows/security.yml/badge.svg)](https://github.com/lornu-ai/ai-coder/actions/workflows/security.yml)
```

## 🔧 Local CI Equivalent

The `.local-ci.toml` file provides a local equivalent of the CI pipeline:

```bash
local-ci  # Runs: fmt, clippy, test, build (sequentially)
```

This allows developers to test locally before pushing.

## 📋 Dependencies

### Required:
- Rust toolchain (dtolnay/rust-toolchain action)
- Cargo

### Optional:
- Nix (for Nix builds)
- cargo-outdated (for dependency checks)

## ⚙️ Customization

### Adding More Platforms

Edit `.github/workflows/release.yml` to add more targets:

```yaml
- os: ubuntu-latest
  target: aarch64-unknown-linux-gnu
  artifact_name: ai-coder
  asset_name: ai-coder-linux-arm64
```

### Disabling Jobs

To skip a job, add `if: false` to the job definition:

```yaml
nix-build:
  if: false
  # ... rest of job
```

### Custom Environment Variables

Add to the `env:` section at workflow level or job level:

```yaml
env:
  CUSTOM_VAR: value
```

## 🚨 Troubleshooting

### Workflow not running

1. Check branch protection rules
2. Verify `.github/workflows/*.yml` files exist
3. Check workflow syntax: https://github.com/lornu-ai/ai-coder/actions

### Build failures

- Check runner logs in GitHub Actions UI
- Reproduce locally with: `cargo build --release`
- For Nix builds: ensure `flake.nix` is correct

### Artifact upload failures

- Ensure `target/release/ai-coder` exists after build
- Check permissions in workflow file

## 📚 Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Rust GitHub Actions](https://docs.github.com/en/actions/guides/building-and-testing-rust)
- [Creating Releases](https://docs.github.com/en/repositories/releasing-projects-on-github/creating-releases)
