---
id: github-infra-004
title: Implement GitHub Mutations (Create/Update/Close Issues)
status: todo
priority: high
tags:
- github
- infrastructure
dependencies: [github-infra-003]
assignee: developer
created: 2025-10-30T21:50:00Z
estimate: 4h
complexity: 7
area: github
---

# Implement GitHub Mutations (Create/Update/Close Issues)

## Context
Once we have the client, types, and mapper, we need functions to modify GitHub state: creating issues, updating status, and closing issues. This enables TaskGuard to push changes to GitHub.

## Dependencies
**Requires:** github-infra-003 (Task-Issue mapper)
**Why:** Mutations need mapper to track created issues and prevent duplicates

## Objectives
1. Implement `create_issue()` mutation
2. Implement `update_issue_state()` mutation (open/close)
3. Implement `update_issue_title()` and `update_issue_body()` mutations
4. Add proper error handling and retry logic

## Implementation Plan

### 1. Create Mutations Module (src/github/mutations.rs)

```rust
use anyhow::{Context, Result};
use serde_json::json;
use super::client::GitHubClient;
use super::types::GitHubIssue;

pub struct GitHubMutations;

impl GitHubMutations {
    /// Create a new GitHub issue
    pub fn create_issue(
        client: &GitHubClient,
        owner: &str,
        repo: &str,
        title: &str,
        body: Option<&str>,
    ) -> Result<GitHubIssue> {
        // Get repository ID first
        let repo_id = Self::get_repository_id(client, owner, repo)?;

        let mutation = r#"
            mutation($repositoryId: ID!, $title: String!, $body: String) {
                createIssue(input: {
                    repositoryId: $repositoryId,
                    title: $title,
                    body: $body
                }) {
                    issue {
                        id
                        number
                        title
                        state
                        body
                    }
                }
            }
        "#;

        let variables = json!({
            "repositoryId": repo_id,
            "title": title,
            "body": body.unwrap_or(""),
        });

        let response = client.query(mutation, variables)
            .context("Failed to create issue")?;

        // Parse response
        let issue_data = response["data"]["createIssue"]["issue"]
            .as_object()
            .context("Invalid issue response")?;

        Ok(GitHubIssue {
            id: issue_data["id"].as_str().unwrap().to_string(),
            number: issue_data["number"].as_u64().unwrap() as u32,
            title: issue_data["title"].as_str().unwrap().to_string(),
            state: issue_data["state"].as_str().unwrap().to_string(),
            body: issue_data["body"].as_str().map(|s| s.to_string()),
        })
    }

    /// Update issue state (OPEN or CLOSED)
    pub fn update_issue_state(
        client: &GitHubClient,
        issue_id: &str,
        state: &str, // "OPEN" or "CLOSED"
    ) -> Result<()> {
        let mutation = match state.to_uppercase().as_str() {
            "CLOSED" => r#"
                mutation($issueId: ID!) {
                    closeIssue(input: { issueId: $issueId }) {
                        issue { id state }
                    }
                }
            "#,
            "OPEN" => r#"
                mutation($issueId: ID!) {
                    reopenIssue(input: { issueId: $issueId }) {
                        issue { id state }
                    }
                }
            "#,
            _ => return Err(anyhow::anyhow!("Invalid state: {}", state)),
        };

        let variables = json!({ "issueId": issue_id });

        client.query(mutation, variables)
            .context("Failed to update issue state")?;

        Ok(())
    }

    /// Update issue title
    pub fn update_issue_title(
        client: &GitHubClient,
        issue_id: &str,
        title: &str,
    ) -> Result<()> {
        let mutation = r#"
            mutation($issueId: ID!, $title: String!) {
                updateIssue(input: {
                    id: $issueId,
                    title: $title
                }) {
                    issue { id title }
                }
            }
        "#;

        let variables = json!({
            "issueId": issue_id,
            "title": title,
        });

        client.query(mutation, variables)
            .context("Failed to update issue title")?;

        Ok(())
    }

    /// Update issue body
    pub fn update_issue_body(
        client: &GitHubClient,
        issue_id: &str,
        body: &str,
    ) -> Result<()> {
        let mutation = r#"
            mutation($issueId: ID!, $body: String!) {
                updateIssue(input: {
                    id: $issueId,
                    body: $body
                }) {
                    issue { id body }
                }
            }
        "#;

        let variables = json!({
            "issueId": issue_id,
            "body": body,
        });

        client.query(mutation, variables)
            .context("Failed to update issue body")?;

        Ok(())
    }

    /// Helper: Get repository ID from owner/name
    fn get_repository_id(
        client: &GitHubClient,
        owner: &str,
        repo: &str,
    ) -> Result<String> {
        let query = r#"
            query($owner: String!, $name: String!) {
                repository(owner: $owner, name: $name) {
                    id
                }
            }
        "#;

        let variables = json!({
            "owner": owner,
            "name": repo,
        });

        let response = client.query(query, variables)
            .context("Failed to get repository ID")?;

        let repo_id = response["data"]["repository"]["id"]
            .as_str()
            .context("Missing repository ID in response")?;

        Ok(repo_id.to_string())
    }
}
```

### 2. Add to Module Exports (src/github/mod.rs)

```rust
pub mod mutations;
pub use mutations::*;
```

## Acceptance Criteria

✅ **Issue Creation:**
- Can create issues with title and body
- Returns complete issue data (ID, number, etc.)
- Properly handles repository not found

✅ **State Updates:**
- Can close issues
- Can reopen issues
- State changes reflected immediately

✅ **Content Updates:**
- Can update issue title
- Can update issue body
- Changes preserved correctly

✅ **Error Handling:**
- Network errors handled gracefully
- Invalid states rejected
- Missing permissions reported clearly

## Testing

```bash
# Build project
cargo build

# Manual test (requires GitHub token):
# 1. Set GITHUB_TOKEN environment variable
# 2. Create test issue
# 3. Update state to closed
# 4. Reopen issue
# 5. Clean up test issue
```

## Technical Notes

- Use GraphQL mutations for all write operations
- GitHub GraphQL API requires node IDs (not numbers)
- Rate limiting: 5000 points/hour (check headers)
- Mutations require write permissions on repository
- Consider adding dry-run mode for testing

## Session Handoff Template

### What Changed
- [Created src/github/mutations.rs with GitHubMutations]
- [Implemented create_issue, update_issue_state, update_issue_title/body]
- [Added repository ID lookup helper]

### Causality Impact
- **Create Issue → Mapper Update**: New issue triggers mapper update
- **Close Issue → Archive**: Closing issue enables archive operations
- **Update Title → Sync**: Title changes sync with local tasks
- **Error → Retry**: Network errors can be retried safely

### Dependencies Unblocked
- github-infra-005: Queries (completes read-write API)
- github-fix-2 (updated): Archive command can now close issues

### Next Task Context
The queries module (github-infra-005) will use the same client to:
1. Fetch existing issues for sync comparison
2. Get issue details by number/ID
3. List all repository issues for bulk sync
4. Check issue state before updates
