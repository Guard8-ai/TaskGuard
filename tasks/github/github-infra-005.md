---
id: github-infra-005
title: Implement GitHub Queries (Read Issues)
status: todo
priority: high
tags:
- github
- infrastructure
dependencies: [github-infra-004]
assignee: developer
created: 2025-10-30T21:50:00Z
estimate: 3h
complexity: 6
area: github
---

# Implement GitHub Queries (Read Issues)

## Context
Complete the GitHub API integration with read operations. This enables TaskGuard to fetch issue data for sync comparisons and status checks.

## Dependencies
**Requires:** github-infra-004 (Mutations)
**Why:** Queries complement mutations for complete read-write API coverage

## Objectives
1. Implement `get_repository_issues()` query
2. Implement `get_issue_by_number()` query
3. Implement `get_issue_by_id()` query
4. Add pagination support for large repositories

## Implementation Plan

### 1. Create Queries Module (src/github/queries.rs)

```rust
use anyhow::{Context, Result};
use serde_json::json;
use super::client::GitHubClient;
use super::types::GitHubIssue;

pub struct GitHubQueries;

impl GitHubQueries {
    /// Get all issues from a repository
    pub fn get_repository_issues(
        client: &GitHubClient,
        owner: &str,
        repo: &str,
        limit: Option<usize>,
    ) -> Result<Vec<GitHubIssue>> {
        let limit = limit.unwrap_or(100);

        let query = r#"
            query($owner: String!, $name: String!, $limit: Int!) {
                repository(owner: $owner, name: $name) {
                    issues(first: $limit, orderBy: {field: UPDATED_AT, direction: DESC}) {
                        nodes {
                            id
                            number
                            title
                            state
                            body
                        }
                    }
                }
            }
        "#;

        let variables = json!({
            "owner": owner,
            "name": repo,
            "limit": limit,
        });

        let response = client.query(query, variables)
            .context("Failed to get repository issues")?;

        let nodes = response["data"]["repository"]["issues"]["nodes"]
            .as_array()
            .context("Invalid issues response")?;

        let issues: Vec<GitHubIssue> = nodes
            .iter()
            .filter_map(|node| {
                Some(GitHubIssue {
                    id: node["id"].as_str()?.to_string(),
                    number: node["number"].as_u64()? as u32,
                    title: node["title"].as_str()?.to_string(),
                    state: node["state"].as_str()?.to_string(),
                    body: node["body"].as_str().map(|s| s.to_string()),
                })
            })
            .collect();

        Ok(issues)
    }

    /// Get a specific issue by number
    pub fn get_issue_by_number(
        client: &GitHubClient,
        owner: &str,
        repo: &str,
        number: u32,
    ) -> Result<GitHubIssue> {
        let query = r#"
            query($owner: String!, $name: String!, $number: Int!) {
                repository(owner: $owner, name: $name) {
                    issue(number: $number) {
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
            "owner": owner,
            "name": repo,
            "number": number,
        });

        let response = client.query(query, variables)
            .context("Failed to get issue")?;

        let issue_data = response["data"]["repository"]["issue"]
            .as_object()
            .context("Issue not found")?;

        Ok(GitHubIssue {
            id: issue_data["id"].as_str().unwrap().to_string(),
            number: issue_data["number"].as_u64().unwrap() as u32,
            title: issue_data["title"].as_str().unwrap().to_string(),
            state: issue_data["state"].as_str().unwrap().to_string(),
            body: issue_data["body"].as_str().map(|s| s.to_string()),
        })
    }

    /// Get issue by GraphQL node ID
    pub fn get_issue_by_id(
        client: &GitHubClient,
        issue_id: &str,
    ) -> Result<GitHubIssue> {
        let query = r#"
            query($id: ID!) {
                node(id: $id) {
                    ... on Issue {
                        id
                        number
                        title
                        state
                        body
                    }
                }
            }
        "#;

        let variables = json!({ "id": issue_id });

        let response = client.query(query, variables)
            .context("Failed to get issue by ID")?;

        let node = response["data"]["node"]
            .as_object()
            .context("Issue not found")?;

        Ok(GitHubIssue {
            id: node["id"].as_str().unwrap().to_string(),
            number: node["number"].as_u64().unwrap() as u32,
            title: node["title"].as_str().unwrap().to_string(),
            state: node["state"].as_str().unwrap().to_string(),
            body: node["body"].as_str().map(|s| s.to_string()),
        })
    }

    /// Helper: Convert issue number to GraphQL node ID
    pub fn get_issue_id(
        client: &GitHubClient,
        owner: &str,
        repo: &str,
        issue_number: u32,
    ) -> Result<String> {
        let issue = Self::get_issue_by_number(client, owner, repo, issue_number)?;
        Ok(issue.id)
    }
}
```

### 2. Add to Module Exports (src/github/mod.rs)

```rust
pub mod queries;
pub use queries::*;
```

## Acceptance Criteria

✅ **Repository Issues:**
- Can fetch all issues from repository
- Results ordered by last updated
- Pagination limit configurable

✅ **Individual Issues:**
- Can get issue by number
- Can get issue by GraphQL node ID
- Missing issues return proper errors

✅ **ID Conversion:**
- Helper converts issue number to node ID
- Used by mutations that need node IDs

✅ **Performance:**
- Large repositories handled efficiently
- Rate limiting respected
- Unnecessary data not fetched

## Testing

```bash
# Build project
cargo build

# Manual test (requires GitHub token):
# 1. Query repository issues
# 2. Get specific issue by number
# 3. Verify issue data matches GitHub web UI
```

## Technical Notes

- GraphQL allows fetching exactly the fields needed
- Pagination with `first: N` and `after: cursor`
- Rate limit: 5000 points/hour (queries cost varies)
- Consider caching for repeated queries
- Future: Add filtering by labels, state, assignee

## Session Handoff Template

### What Changed
- [Created src/github/queries.rs with GitHubQueries]
- [Implemented get_repository_issues, get_issue_by_number, get_issue_by_id]
- [Added issue number to ID conversion helper]

### Causality Impact
- **Fetch Issues → Compare**: Query results compared with local tasks
- **Get by Number → Update**: Number lookup enables targeted updates
- **Get by ID → Sync**: ID-based access for efficient sync
- **Query → Cache**: Results can be cached to reduce API calls

### Dependencies Unblocked
- github-fix-2 (updated): Archive can now close issues (full API ready)
- github-fix-4: Sync can query and update issues
- github-fix-5: Validate can check GitHub issue states

### Next Task Context
With the complete API infrastructure (infra-001 through infra-005), we can now update the original github-fix-* tasks to use these modules:

**github-fix-2 (Archive)** can now:
1. Load mapper to find task's GitHub issue
2. Use GitHubMutations::update_issue_state() to close issue
3. Mark mapping as archived

**github-fix-4 (Sync)** can now:
1. Use GitHubQueries to fetch all issues
2. Compare with local tasks (including archived)
3. Use GitHubMutations to update issue states
4. Update mapper with sync results

**github-fix-5 (Validate)** can now:
1. Query issues for mapped tasks
2. Check for state mismatches
3. Report issues needing attention
