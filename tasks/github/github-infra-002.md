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
Before we can integrate GitHub Issues with archive/sync commands, we need the foundational GitHub API infrastructure. This task creates the core module structure and GraphQL client.

## Objectives
1. Create `src/github/` module structure
2. Implement GraphQL client for GitHub API
3. Add basic authentication and configuration
4. Create type definitions for GitHub entities

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

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubIssue {
    pub id: String,           // GraphQL node ID
    pub number: u32,          // Issue number
    pub title: String,
    pub state: String,        // "OPEN", "CLOSED"
    pub body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubConfig {
    pub token: String,
    pub owner: String,
    pub repo: String,
}
```

### 4. Implement GraphQL Client (src/github/client.rs)

```rust
use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde_json::Value;

pub struct GitHubClient {
    client: Client,
    token: String,
    api_url: String,
}

impl GitHubClient {
    pub fn new(token: String) -> Result<Self> {
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

        Ok(json)
    }
}
```

### 5. Implement Configuration (src/github/config.rs)

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
```

## Acceptance Criteria

✅ **Module Structure:**
- `src/github/` directory exists with all module files
- Module properly exported in `src/lib.rs`

✅ **GraphQL Client:**
- Can create GitHubClient with authentication token
- Can send GraphQL queries to GitHub API
- Proper error handling for network failures

✅ **Configuration:**
- Can check if GitHub sync is enabled
- Can load GitHub configuration from `.taskguard/github.toml`
- Configuration includes token, owner, and repo

✅ **Type Safety:**
- All GitHub entities have proper type definitions
- Serde serialization/deserialization works correctly

## Testing

```bash
# Build with new dependencies
cargo build

# Test module imports
# (Will add unit tests in next task)
```

## Technical Notes

- Use `reqwest` blocking client for simplicity (sync commands only)
- Store GitHub token in `.taskguard/github.toml` (gitignored)
- Follow GitHub GraphQL API v4 schema
- Handle rate limiting gracefully (future enhancement)

## Session Handoff Template

### What Changed
- [List new files created and their purposes]
- [Dependencies added to Cargo.toml]
- [Module exports in src/lib.rs]

### Causality Impact
- [How GitHub client creation flows]
- [Configuration loading sequence]
- [Error handling patterns]

### Dependencies Unblocked
- github-infra-003: Task-Issue mapper (needs client)
- github-infra-004: GitHub mutations (needs client and types)

### Next Task Context
The mapper (github-infra-003) will use this client to query/update GitHub. It needs:
- GitHubClient for API calls
- GitHubConfig for repository info
- GitHubIssue type for data exchange
