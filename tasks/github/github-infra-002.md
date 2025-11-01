---
id: github-infra-002
title: Create GitHub API Module Foundation
status: todo
priority: high
tags:
- github
- infrastructure
dependencies: [github-infra-001]
assignee: developer
created: 2025-10-30T21:50:00Z
estimate: 4h
complexity: 7
area: github
---

# Create GitHub API Module Foundation

## Context
**PRIORITY UPDATE**: Community users requested **GitHub Projects v2 Dashboard** visibility (not just Issues). This task creates the foundational GitHub API infrastructure with **Projects v2 as the primary focus**. Issues are created to populate the Projects board.

## Objectives
1. Create `src/github/` module structure
2. Implement GraphQL client for GitHub API
3. Add basic authentication and configuration
4. Create type definitions for **GitHub Projects v2** entities (Issues, ProjectItem, FieldValue, ProjectV2StatusUpdate)

## Dependencies
**Requires:** github-infra-001 (Git commit tracking)
**Why:** Archive commits provide the foundation for tracking task lifecycle

## Implementation Plan

### 1. Add Dependencies to Cargo.toml

```toml
[dependencies]
# Add these to existing dependencies
reqwest = { version = "0.11", features = ["json", "blocking"] }
serde_json = "1.0"
```

### 2. Create Module Structure

```
src/github/
├── mod.rs           # Module exports
├── client.rs        # GraphQL client
├── types.rs         # GitHub-specific types
└── config.rs        # Configuration helpers
```

### 3. Implement GitHub Types (src/github/types.rs)

**CRITICAL**: Include Projects v2 types as PRIMARY structures

```rust
use serde::{Deserialize, Serialize};

// Issues (used to populate Projects board)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubIssue {
    pub id: String,           // GraphQL node ID
    pub number: i64,          // Issue number (changed from u32 to i64 for GraphQL compatibility)
    pub title: String,
    pub state: String,        // "OPEN", "CLOSED"
    pub body: Option<String>,
    pub labels: Vec<String>,
    pub assignees: Vec<String>,
}

// Projects v2 - PRIMARY FOCUS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectItem {
    pub id: String,              // Project item ID
    pub issue_id: String,        // Associated issue ID
    pub project_id: String,      // Parent project ID
    pub status: String,          // Status field value
    pub field_values: Vec<FieldValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldValue {
    pub field_id: String,
    pub value: String,
}

// NEW: 2025 API Update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectV2StatusUpdate {
    pub id: String,
    pub status: String,
    pub body: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubConfig {
    // NO TOKEN FIELD - uses `gh` CLI authentication instead
    pub owner: String,
    pub repo: String,
    pub project_number: i64,     // Required for Projects v2
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskMapping {
    pub task_id: String,
    pub issue_number: i64,
    pub project_item_id: String,  // Track project board item
    pub last_synced: String,      // ISO 8601 timestamp
}
```

### 4. Implement GraphQL Client (src/github/client.rs)

**AUTHENTICATION STRATEGY**: Use `gh` CLI for token management (better UX than manual tokens)

```rust
use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde_json::Value;
use std::process::Command;

pub struct GitHubClient {
    client: Client,
    token: String,
    api_url: String,
}

impl GitHubClient {
    /// Create client using `gh` CLI authentication (ONLY supported method for quick delivery)
    pub fn new() -> Result<Self> {
        // Get token from `gh auth token` command
        let output = Command::new("gh")
            .args(&["auth", "token"])
            .output()
            .context("Failed to run 'gh auth token'. Is gh CLI installed?")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!(
                "GitHub authentication failed.\n\n\
                Please run: gh auth login\n\n\
                Error: {}", stderr
            );
        }

        let token = String::from_utf8(output.stdout)
            .context("Invalid UTF-8 in gh token")?
            .trim()
            .to_string();

        if token.is_empty() {
            anyhow::bail!(
                "No GitHub token found.\n\n\
                Please run: gh auth login"
            );
        }

        let client = Client::builder()
            .user_agent("TaskGuard/0.2.2")
            .build()
            .context("Failed to create HTTP client")?;

        Ok(GitHubClient {
            client,
            token,
            api_url: "https://api.github.com/graphql".to_string(),
        })
    }

    pub fn query(&self, query: &str, variables: Value) -> Result<Value> {
        let body = serde_json::json!({
            "query": query,
            "variables": variables,
        });

        let response = self.client
            .post(&self.api_url)
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&body)
            .send()
            .context("Failed to send GraphQL request")?;

        let json: Value = response.json()
            .context("Failed to parse GraphQL response")?;

        // Check for GraphQL errors
        if let Some(errors) = json.get("errors") {
            anyhow::bail!("GitHub API error: {}", errors);
        }

        Ok(json)
    }
}
```

### 5. Implement Configuration (src/github/config.rs)

**NOTE**: No token in config file - uses `gh` CLI authentication instead

```rust
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use crate::config::find_taskguard_root;
use super::types::GitHubConfig;

pub fn is_github_sync_enabled() -> Result<bool> {
    let root = find_taskguard_root()
        .context("Not in a TaskGuard project")?;
    let config_path = root.join(".taskguard/github.toml");
    Ok(config_path.exists())
}

pub fn load_github_config() -> Result<GitHubConfig> {
    let root = find_taskguard_root()
        .context("Not in a TaskGuard project")?;
    let config_path = root.join(".taskguard/github.toml");

    let content = fs::read_to_string(&config_path)
        .context("Failed to read GitHub config")?;

    let config: GitHubConfig = toml::from_str(&content)
        .context("Failed to parse GitHub config")?;

    Ok(config)
}

/// Example .taskguard/github.toml file (NO TOKEN NEEDED):
/// ```toml
/// owner = "Eliran79"
/// repo = "TaskGuard"
/// project_number = 1
/// ```
```

## Acceptance Criteria

✅ **Module Structure:**
- `src/github/` directory exists with all module files
- Module properly exported in `src/lib.rs`

✅ **GraphQL Client:**
- Can create GitHubClient using `gh` CLI authentication (ONLY method - simple!)
- Can send GraphQL queries to GitHub API
- Proper error handling for network failures and GraphQL errors
- Clear error messages if `gh` CLI not installed or not authenticated

✅ **Configuration:**
- Can check if GitHub sync is enabled
- Can load GitHub configuration from `.taskguard/github.toml`
- Configuration includes owner, repo, **and project_number** (NO token - uses `gh` CLI)
- Clear example configuration provided

✅ **Type Safety - Projects v2 Focus:**
- ✅ GitHubIssue type (with labels, assignees)
- ✅ ProjectItem type (PRIMARY)
- ✅ FieldValue type (for custom fields)
- ✅ ProjectV2StatusUpdate type (2025 API)
- ✅ TaskMapping includes project_item_id
- All types use Serde serialization/deserialization

## Testing

```bash
# Build with new dependencies
cargo build

# Test module imports
# (Will add unit tests in next task)
```

## Technical Notes

### Authentication Strategy (Simplified for Quick Delivery)
- **ONLY method**: `gh` CLI authentication via `gh auth token`
- **Why `gh` CLI only**:
  - Users already have `gh` installed for GitHub operations
  - Secure keyring storage (system-managed)
  - Supports multiple accounts (users can switch with `gh auth switch`)
  - No manual token management required
  - Tokens automatically refreshed by `gh` CLI
  - **Simpler code** - no fallback paths, no complexity
- **User experience**: Just run `gh auth login` once, TaskGuard handles the rest
- **Future**: Can add explicit token support later if needed for CI/CD

### API Client
- Use `reqwest` blocking client for simplicity (sync commands only)
- Configuration in `.taskguard/github.toml` (NO token - just owner/repo/project_number)
- Follow GitHub GraphQL API v4 schema
- Handle rate limiting gracefully (future enhancement)

### Multi-Account Support (from your `gh auth status`)
Users with multiple GitHub accounts can use:
```bash
gh auth switch             # Switch between accounts
gh auth status             # Check active account
taskguard sync --github    # Uses active account automatically
```

### Required Token Scopes
For Projects v2 Dashboard integration, the GitHub token needs:
- `repo` - Read/write access to repository and issues ✅ (you have this)
- `read:org` - Read organization projects ✅ (you have this)
- `project` - Read/write access to projects (may need to add this)

Check scopes with: `gh auth status`

If missing project scope, re-authenticate:
```bash
gh auth refresh -s project
```

## Session Handoff Template

### What Changed
- [Created src/github/client.rs with simplified `gh` CLI-only authentication]
- [Created src/github/types.rs with Projects v2 types]
- [Created src/github/config.rs (no token field)]
- [Added reqwest and serde_json dependencies to Cargo.toml]
- [Module exports in src/lib.rs]

### Causality Impact
- **Simple auth flow**: `GitHubClient::new()` → `gh auth token` → authenticated client
- **No token storage**: Config file only has owner/repo/project_number
- **Clear errors**: If `gh` not installed or not logged in, user gets actionable message

### Dependencies Unblocked
- github-infra-003: Task-Issue-Project mapper (needs client + Projects v2 types)
- github-infra-004: GitHub mutations for Projects v2 (needs client and types)

### Next Task Context
The mapper (github-infra-003) will use this client to sync with **GitHub Projects v2 boards**. It needs:
- GitHubClient for API calls
- GitHubConfig for repository and project info
- ProjectItem, FieldValue types for project board operations
- TaskMapping to track task_id ↔ issue_number ↔ project_item_id

**Key Decision**: Projects v2 Dashboard is the PRIMARY goal, not Issues alone.
