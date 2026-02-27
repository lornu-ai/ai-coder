# GitHub Integration Tools Plan

This document outlines the plan for adding GitHub integration capabilities to ai-coder.

## Overview

Phase 2 will add GitHub API integration to enable ai-coder to:
- Post code reviews on pull requests
- Read and analyze repository files
- Fetch PR/issue details
- Create commits and push changes
- Manage issues and discussions

## Architecture

### Option A: Lightweight HTTP Client (Recommended)
- Use `reqwest` (already in dependencies) for GitHub API calls
- Add GitHub token authentication via `GITHUB_TOKEN` env var
- Create lean structs for GitHub API responses
- Minimal dependencies, maximum control

**Pros:** Simple, minimal overhead, tight integration with ai-coder
**Cons:** Need to manually handle API pagination, error handling

### Option B: `octokit` Rust Library
- Use established GitHub API client library
- Automatic pagination, better error handling
- Well-tested and documented

**Pros:** Battle-tested, comprehensive API coverage
**Cons:** Another dependency, less control

### Option C: CLI Wrapper (`gh` command)
- Shell out to GitHub CLI (if available)
- Users need `gh` installed locally
- Leverage user's existing auth

**Pros:** No new code needed, reuses existing auth
**Cons:** Not portable, requires external tool, harder to integrate with Rust streams

## Proposed Implementation

### 1. GitHub Module Structure

```rust
// src/github/mod.rs
pub mod client;
pub mod models;
pub mod errors;

// src/github/client.rs
pub struct GitHubClient {
    http_client: Client,
    token: String,
    base_url: String,
}

// src/github/models.rs
#[derive(Deserialize)]
pub struct PullRequest {
    pub number: u32,
    pub title: String,
    pub body: String,
    pub state: String,
}

// src/github/errors.rs
pub enum GitHubError {
    ApiError(String),
    AuthenticationError,
    NotFound,
    RateLimited,
}
```

### 2. Core Operations

#### 2.1 Read Repository Content
```rust
impl GitHubClient {
    pub async fn get_file_content(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        branch: &str,
    ) -> Result<String, GitHubError>
}
```

#### 2.2 Post Code Review
```rust
#[derive(Serialize)]
pub struct ReviewComment {
    pub body: String,
    pub event: String,  // APPROVE, REQUEST_CHANGES, COMMENT
}

impl GitHubClient {
    pub async fn post_pr_review(
        &self,
        owner: &str,
        repo: &str,
        pr_number: u32,
        review: ReviewComment,
    ) -> Result<(), GitHubError>
}
```

#### 2.3 Get PR Details
```rust
impl GitHubClient {
    pub async fn get_pull_request(
        &self,
        owner: &str,
        repo: &str,
        pr_number: u32,
    ) -> Result<PullRequest, GitHubError>
}
```

#### 2.4 Create Commit
```rust
#[derive(Serialize)]
pub struct CreateCommitRequest {
    pub message: String,
    pub tree: String,
    pub parents: Vec<String>,
}

impl GitHubClient {
    pub async fn create_commit(
        &self,
        owner: &str,
        repo: &str,
        request: CreateCommitRequest,
    ) -> Result<String, GitHubError>  // Returns commit SHA
}
```

### 3. Authentication

```bash
# Set token via environment variable
export GITHUB_TOKEN="ghp_xxxxxxxxxxxx"

# ai-coder automatically uses it
./target/release/ai-coder --github "analyze PR #1 and post a review"
```

### 4. New CLI Flags

```rust
#[arg(long)]
github: bool,  // Enable GitHub operations

#[arg(long)]
github_token: Option<String>,  // Override GITHUB_TOKEN env var

#[arg(long)]
repo: Option<String>,  // org/repo format
```

### 5. Integration with Agent Mode

Extend agent mode to support GitHub operations:

```bash
# AI generates both bash AND GitHub operations
./target/release/ai-coder --agent --github \
  "Review PR #1 and suggest improvements"

# ai-coder executes:
# 1. Bash commands if present
# 2. GitHub operations (posting reviews, creating issues, etc.)
```

## Implementation Roadmap

### Phase 2a: Foundation (Week 1)
- [ ] Add `reqwest::Client` GitHub wrapper
- [ ] Implement `GitHubClient` struct
- [ ] Add authentication via `GITHUB_TOKEN`
- [ ] Implement `get_pull_request()`
- [ ] Implement `get_file_content()`
- [ ] Add error handling

### Phase 2b: Review Operations (Week 2)
- [ ] Implement `post_pr_review()`
- [ ] Add inline comments support
- [ ] Add event support (APPROVE, REQUEST_CHANGES, COMMENT)
- [ ] Add tests with mock GitHub API

### Phase 2c: Write Operations (Week 3)
- [ ] Implement `create_commit()`
- [ ] Implement `push_branch()`
- [ ] Add PR creation support
- [ ] Handle merge conflicts

### Phase 2d: Integration (Week 4)
- [ ] Integrate with agent mode
- [ ] Create example prompts for GitHub workflows
- [ ] Add comprehensive documentation
- [ ] Create PR with Phase 2 changes

## Example Usage

### Example 1: Analyze PR and Post Review
```bash
./target/release/ai-coder --github \
  "analyze PR #54 in lornu-ai/bond for security issues and post a review"
```

Output:
```
[ai-coder] Mode: CHAT
[ai-coder] GitHub mode: ENABLED
[ai-coder] Repo: lornu-ai/bond

[Fetches PR #54 details]
[AI analyzes the changes]
[Posts review as comment on PR]

✓ Review posted to PR #54
```

### Example 2: Code Review with Suggestions
```bash
./target/release/ai-coder --agent --github \
  "Review PR #1 and suggest code improvements, create a commit with the fixes"
```

Output:
```
[ai-coder] Mode: AGENT
[ai-coder] GitHub mode: ENABLED

[Fetches PR #1]
[AI generates suggestions]
[ai-coder-agent] Found bash command(s):
git clone ...
git checkout ...
[ai-coder-agent] Execute? (y/n): y

[Creates commits]
[Posts PR review with suggestions]
✓ Changes committed and review posted
```

## Dependencies to Add

```toml
[dependencies]
# No new dependencies needed!
# Uses existing: reqwest, serde, serde_json, tokio
```

## Testing Strategy

### Unit Tests
- Mock GitHub API responses
- Test error handling
- Test authentication

### Integration Tests (Optional)
- Create test repository
- Test actual API calls (requires token)
- Clean up test data

### Example Test
```rust
#[tokio::test]
async fn test_get_pull_request() {
    let client = GitHubClient::new("ghp_test_token");
    // Mock the API response
    let pr = client.get_pull_request("owner", "repo", 1).await;
    assert!(pr.is_ok());
}
```

## Future Enhancements (Phase 3+)

- MCP Server: Expose GitHub operations via MCP protocol
- Workflow Automation: Automated PR reviews on every push
- Issue Tracking: Create issues from code analysis
- Status Checks: Fail CI if security issues found
- Project Management: Update GitHub Projects board

## Security Considerations

- **Token Management:** Never log or expose `GITHUB_TOKEN`
- **Rate Limiting:** Handle GitHub API rate limits gracefully
- **Scope:** Limit token to necessary permissions
- **Validation:** Validate owner/repo names before API calls

## Resources

- [GitHub REST API Docs](https://docs.github.com/en/rest)
- [GitHub Authentication](https://docs.github.com/en/authentication)
- [Creating Reviews on PRs](https://docs.github.com/en/rest/pulls/reviews)
- [File Contents API](https://docs.github.com/en/rest/repos/contents)
