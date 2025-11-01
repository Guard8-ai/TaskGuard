---
id: github-infra-005
title: Implement GitHub Queries (Read Issues & Projects v2)
status: todo
priority: high
tags:
- github
- infrastructure
- projects-v2
dependencies:
- github-infra-004
assignee: developer
created: 2025-10-30T21:50:00Z
estimate: 4h
complexity: 7
area: github
---

# Implement GitHub Queries (Read Issues & Projects v2)

## Context
Complete the GitHub API integration with read operations for both Issues and Projects v2. This enables TaskGuard to:
- Fetch issue data for sync comparisons
- Discover project custom fields (especially Status fields)
- Query project items and their status values
- Support flexible status mapping for different project configurations

## Dependencies
**Requires:** github-infra-004 (Mutations)
**Why:** Queries complement mutations for complete read-write API coverage

## Objectives

### Issue Queries
1. Implement `get_repository_issues()` query
2. Implement `get_issue_by_number()` query
3. Implement `get_issue_by_id()` query
4. Add pagination support for large repositories

### Projects v2 Queries (NEW - HIGH PRIORITY)
5. Implement `get_project_fields()` - Discover custom fields
6. Implement `get_status_field_options()` - Get available status columns
7. Implement `get_project_item()` - Get item with field values
8. Implement `get_project_items_by_issue()` - Find items by issue ID

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

    /// Get project custom fields (especially Status field)
    pub fn get_project_fields(
        client: &GitHubClient,
        project_node_id: &str,
    ) -> Result<Vec<ProjectField>> {
        let query = r#"
            query($projectId: ID!) {
                node(id: $projectId) {
                    ... on ProjectV2 {
                        fields(first: 20) {
                            nodes {
                                ... on ProjectV2Field {
                                    id
                                    name
                                    dataType
                                }
                                ... on ProjectV2SingleSelectField {
                                    id
                                    name
                                    dataType
                                    options {
                                        id
                                        name
                                    }
                                }
                            }
                        }
                    }
                }
            }
        "#;

        let variables = json!({ "projectId": project_node_id });
        let response = client.query(query, variables)
            .context("Failed to get project fields")?;

        // Parse fields from response
        let nodes = response["data"]["node"]["fields"]["nodes"]
            .as_array()
            .context("Invalid fields response")?;

        // Convert to ProjectField structs (defined in types.rs)
        // Implementation depends on ProjectField struct design
        Ok(Vec::new()) // Placeholder
    }

    /// Get available status options for a project's Status field
    pub fn get_status_field_options(
        client: &GitHubClient,
        project_node_id: &str,
        status_field_name: &str,
    ) -> Result<Vec<StatusOption>> {
        let query = r#"
            query($projectId: ID!) {
                node(id: $projectId) {
                    ... on ProjectV2 {
                        fields(first: 20) {
                            nodes {
                                ... on ProjectV2SingleSelectField {
                                    id
                                    name
                                    options {
                                        id
                                        name
                                    }
                                }
                            }
                        }
                    }
                }
            }
        "#;

        let variables = json!({ "projectId": project_node_id });
        let response = client.query(query, variables)
            .context("Failed to get status field options")?;

        // Find the Status field and extract options
        // Implementation depends on StatusOption struct design
        Ok(Vec::new()) // Placeholder
    }

    /// Get a project item with its field values
    pub fn get_project_item(
        client: &GitHubClient,
        project_item_id: &str,
    ) -> Result<ProjectItem> {
        let query = r#"
            query($itemId: ID!) {
                node(id: $itemId) {
                    ... on ProjectV2Item {
                        id
                        content {
                            ... on Issue {
                                id
                                number
                                title
                            }
                        }
                        fieldValues(first: 20) {
                            nodes {
                                ... on ProjectV2ItemFieldSingleSelectValue {
                                    name
                                    field {
                                        ... on ProjectV2SingleSelectField {
                                            name
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        "#;

        let variables = json!({ "itemId": project_item_id });
        let response = client.query(query, variables)
            .context("Failed to get project item")?;

        // Parse into ProjectItem struct (defined in types.rs)
        Ok(ProjectItem {
            id: project_item_id.to_string(),
            issue_id: String::new(), // Placeholder
            status: String::new(),   // Placeholder
        })
    }

    /// Find project items by issue ID
    pub fn get_project_items_by_issue(
        client: &GitHubClient,
        project_node_id: &str,
        issue_node_id: &str,
    ) -> Result<Vec<ProjectItem>> {
        let query = r#"
            query($projectId: ID!, $issueId: ID!) {
                node(id: $projectId) {
                    ... on ProjectV2 {
                        items(first: 100) {
                            nodes {
                                id
                                content {
                                    ... on Issue {
                                        id
                                    }
                                }
                            }
                        }
                    }
                }
            }
        "#;

        let variables = json!({
            "projectId": project_node_id,
            "issueId": issue_node_id,
        });

        let response = client.query(query, variables)
            .context("Failed to get project items by issue")?;

        // Filter items that match the issue_node_id
        // Implementation depends on ProjectItem struct design
        Ok(Vec::new()) // Placeholder
    }
}
```

### 2. Add to Module Exports (src/github/mod.rs)

```rust
pub mod queries;
pub use queries::*;
```

## Acceptance Criteria

### Issue Queries
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

### Projects v2 Queries (NEW)
✅ **Field Discovery:**
- Can query all custom fields in a project
- Identifies Status/SingleSelect fields
- Returns field IDs and available options

✅ **Status Field Mapping:**
- Can get available status columns (Backlog, In Progress, etc.)
- Supports custom column names
- Returns option IDs needed for updates

✅ **Project Item Queries:**
- Can get project item by ID with field values
- Can find project items by issue ID
- Returns current status value

✅ **Flexible Status Support:**
- Works with different project configurations
- Handles custom status field names
- Supports varied column naming conventions

### Performance
✅ **Efficiency:**
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

### Issue Queries
- GraphQL allows fetching exactly the fields needed
- Pagination with `first: N` and `after: cursor`
- Rate limit: 5000 points/hour (queries cost varies)
- Consider caching for repeated queries
- Future: Add filtering by labels, state, assignee

### Projects v2 Queries (Critical for Dashboard Integration)
- **Field Discovery Pattern**: Query project → get fields → find Status field
- **Status Options**: SingleSelectField has `options` array with id/name pairs
- **Flexible Mapping**: Must support custom column names:
  - "Backlog", "Ready", "Todo" → TaskGuard `todo`
  - "In progress", "In Progress", "Working" → TaskGuard `doing`
  - "In review", "Review" → TaskGuard `review`
  - "Done", "Completed" → TaskGuard `done`
- **Project Item Structure**: ProjectV2Item contains:
  - `id`: Item node ID (needed for updates)
  - `content`: The linked issue
  - `fieldValues`: Array of field values including status
- **User Experience**: Users see tasks on GitHub Projects Dashboard (see WhatsApp screenshot)
- **2025 API Changes**: ProjectV2StatusUpdate is the recommended pattern

## Session Handoff Template

### What Changed
- [Created src/github/queries.rs with GitHubQueries]
- [Implemented issue queries: get_repository_issues, get_issue_by_number, get_issue_by_id]
- [Added issue number to ID conversion helper]
- **[NEW]** [Implemented Projects v2 queries: get_project_fields, get_status_field_options]
- **[NEW]** [Implemented project item queries: get_project_item, get_project_items_by_issue]

### Causality Impact

#### Issue Query Chain
- **Fetch Issues → Compare**: Query results compared with local tasks
- **Get by Number → Update**: Number lookup enables targeted updates
- **Get by ID → Sync**: ID-based access for efficient sync
- **Query → Cache**: Results can be cached to reduce API calls

#### Projects v2 Query Chain (NEW - CRITICAL PATH)
- **Get Fields → Discover Status**: Find available status columns in user's project
- **Status Options → Map**: Map TaskGuard statuses to project columns
- **Get Item by Issue → Current State**: Check current status on dashboard
- **Field IDs → Update Mutations**: Query results feed into status update mutations

### Dependencies Unblocked
- **github-infra-004**: Can now query field IDs needed for mutations
- **github-sync-001**: Full read-write cycle now possible (query + update)
- **github-fix-2**: Archive can close issues AND update project status
- **github-fix-4**: Sync can query issues, project items, and update both

### Next Task Context

#### For github-sync-001 (Sync Command)
The complete workflow is now possible:
1. Query project fields to discover Status field
2. Get status options and build mapping
3. Fetch all issues from repository
4. For each issue, get project item and current status
5. Compare with local task status
6. Use mutations to update mismatches

#### For User Experience
When user runs `taskguard sync --github`:
```
✅ Synced task: backend-001
   Issue #123: Open
   Project status: In progress ✅
   Dashboard: https://github.com/users/me/projects/1
```

User sees their tasks on GitHub Projects Dashboard with proper status columns (Backlog, In Progress, Done, etc.), not just as a list of issues.