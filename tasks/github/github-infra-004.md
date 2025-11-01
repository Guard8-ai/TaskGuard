---
id: github-infra-004
title: Implement GitHub Mutations (Create/Update/Close Issues)
status: todo
priority: high
tags:
- github
- infrastructure
dependencies:
- github-infra-003
assignee: developer
created: 2025-10-30T21:50:00Z
estimate: 4h
complexity: 7
area: github
---

# Implement GitHub Mutations (Issues + Projects v2)

## Context
**PROJECTS V2 FOCUS**: Functions to modify GitHub state for **Projects v2 Dashboard**. Creating issues is step 1, adding to project board and setting status are PRIMARY goals. This enables TaskGuard to push tasks to Projects dashboard.

## Dependencies
**Requires:** github-infra-003 (Task-Issue mapper)
**Why:** Mutations need mapper to track created issues and prevent duplicates

## Objectives
1. Implement `create_issue()` mutation
2. **Implement `add_issue_to_project()` mutation (CORE for Projects v2)**
3. **Implement `update_project_item_status()` mutation (CORE for Projects v2)**
4. **Implement `get_status_field_info()` query (get field IDs and options)**
5. Implement `update_issue_state()` mutation (open/close)
6. Add proper error handling and retry logic

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

    // ========================================
    // PROJECTS V2 MUTATIONS (HIGH PRIORITY)
    // ========================================

    /// Add an issue to a GitHub Project v2 board (CORE mutation)
    pub fn add_issue_to_project(
        client: &GitHubClient,
        project_id: &str,
        issue_id: &str,
    ) -> Result<String> {
        let mutation = r#"
            mutation($projectId: ID!, $contentId: ID!) {
                addProjectV2ItemById(input: {
                    projectId: $projectId,
                    contentId: $contentId
                }) {
                    item {
                        id
                    }
                }
            }
        "#;

        let variables = json!({
            "projectId": project_id,
            "contentId": issue_id,
        });

        let response = client.query(mutation, variables)
            .context("Failed to add issue to project")?;

        let item_id = response["data"]["addProjectV2ItemById"]["item"]["id"]
            .as_str()
            .context("Missing project item ID in response")?
            .to_string();

        Ok(item_id)
    }

    /// Update project item status field (CORE mutation for dashboard)
    pub fn update_project_item_status(
        client: &GitHubClient,
        project_id: &str,
        item_id: &str,
        field_id: &str,
        option_id: &str,
    ) -> Result<()> {
        let mutation = r#"
            mutation($projectId: ID!, $itemId: ID!, $fieldId: ID!, $value: ProjectV2FieldValue!) {
                updateProjectV2ItemFieldValue(input: {
                    projectId: $projectId,
                    itemId: $itemId,
                    fieldId: $fieldId,
                    value: $value
                }) {
                    projectV2Item {
                        id
                    }
                }
            }
        "#;

        let variables = json!({
            "projectId": project_id,
            "itemId": item_id,
            "fieldId": field_id,
            "value": {
                "singleSelectOptionId": option_id
            }
        });

        client.query(mutation, variables)
            .context("Failed to update project item status")?;

        Ok(())
    }

    /// Get status field info (field ID and available options)
    pub fn get_status_field_info(
        client: &GitHubClient,
        project_id: &str,
    ) -> Result<(String, Vec<(String, String)>)> {
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

        let variables = json!({ "projectId": project_id });
        let response = client.query(query, variables)
            .context("Failed to get status field info")?;

        // Parse fields to find Status field
        let fields = response["data"]["node"]["fields"]["nodes"]
            .as_array()
            .context("Invalid fields response")?;

        // Find a field named "Status" (or similar)
        for field in fields {
            if let Some(name) = field["name"].as_str() {
                if name.eq_ignore_ascii_case("status") {
                    let field_id = field["id"].as_str()
                        .context("Missing field ID")?
                        .to_string();

                    let options = field["options"]
                        .as_array()
                        .context("Missing options")?
                        .iter()
                        .filter_map(|opt| {
                            let id = opt["id"].as_str()?.to_string();
                            let name = opt["name"].as_str()?.to_string();
                            Some((id, name))
                        })
                        .collect();

                    return Ok((field_id, options));
                }
            }
        }

        Err(anyhow::anyhow!("Status field not found in project"))
    }
}
```

### 2. Add to Module Exports (src/github/mod.rs)

```rust
pub mod mutations;
pub use mutations::*;
```

## Acceptance Criteria

### Issue Mutations
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

### Projects v2 Mutations (HIGH PRIORITY)
✅ **Add to Project Board:**
- Can add issue to Projects v2 dashboard
- Returns project item ID for tracking
- Handles duplicate additions gracefully

✅ **Update Status Field:**
- Can update project item status column
- Supports any SingleSelect field
- Maps TaskGuard status → project column

✅ **Field Discovery:**
- Can query project fields and options
- Identifies Status field automatically
- Returns field ID and option IDs for updates

✅ **Dashboard Integration:**
- User sees task on Projects dashboard after sync
- Status column reflects TaskGuard status
- Changes appear in real-time on GitHub

### Error Handling
✅ **Robustness:**
- Network errors handled gracefully
- Invalid states rejected
- Missing permissions reported clearly
- Project not found errors handled

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

### Issue Mutations
- Use GraphQL mutations for all write operations
- GitHub GraphQL API requires node IDs (not numbers)
- Rate limiting: 5000 points/hour (check headers)
- Mutations require write permissions on repository
- Consider adding dry-run mode for testing

### Projects v2 Mutations (Critical for Dashboard)
- **Two-step process**: Create issue → Add to project → Update status
- **Node IDs everywhere**: project_id, item_id, field_id, option_id all use GraphQL node IDs
- **Status field discovery**: Must query project fields first to get field_id and option_ids
- **Flexible field names**: Status field might be named "Status", "State", etc.
- **Option ID mapping**: Must map TaskGuard status names to project option IDs:
  - Query project → Get "Status" field → Get options → Find "In Progress" → Use its ID
- **User experience**: User sees `https://github.com/users/USERNAME/projects/NUMBER`
- **2025 API pattern**: Use `updateProjectV2ItemFieldValue` with `singleSelectOptionId`

## Session Handoff Template

### What Changed
- [Created src/github/mutations.rs with GitHubMutations]
- [Implemented issue mutations: create_issue, update_issue_state, update_issue_title/body]
- [Added repository ID lookup helper]
- **[NEW]** [Implemented Projects v2 mutations: add_issue_to_project, update_project_item_status]
- **[NEW]** [Implemented field discovery: get_status_field_info]

### Causality Impact

#### Issue Mutation Chain
- **Create Issue → Mapper Update**: New issue triggers mapper update
- **Close Issue → Archive**: Closing issue enables archive operations
- **Update Title → Sync**: Title changes sync with local tasks
- **Error → Retry**: Network errors can be retried safely

#### Projects v2 Mutation Chain (NEW - CRITICAL PATH)
- **Create Issue → Add to Project**: Issue must exist before adding to project
- **Add to Project → Returns Item ID**: Item ID needed for status updates
- **Get Field Info → Status Update**: Field/option IDs needed for updateProjectV2ItemFieldValue
- **Update Status → Dashboard Visible**: User sees task in correct column

### Dependencies Unblocked
- **github-infra-005**: Queries (completes read-write API with Projects v2 support)
- **github-sync-001**: Full workflow now possible (create + add + update status)
- **github-fix-2**: Archive can close issues AND update project status
- **github-infra-003**: Mapper now needs to track project_item_id

### Next Task Context

#### For github-infra-005 (Queries)
The queries module will complement these mutations by:
1. Querying project fields to get field_id/option_ids
2. Fetching existing project items and their statuses
3. Comparing local tasks with project item states

#### For github-sync-001 (Sync Command)
The complete sync workflow is now possible:
```
1. Create issue with create_issue() → Get issue_id
2. Add to project with add_issue_to_project() → Get item_id
3. Query field info with get_status_field_info() → Get field_id, option_ids
4. Map TaskGuard status → Find matching option_id
5. Update status with update_project_item_status()
6. User sees task on Projects dashboard in correct column
```

#### User Experience Example
```bash
$ taskguard sync backend-001 --github
✅ Created issue #123: "Implement JWT Auth"
✅ Added to project board
✅ Set status: In Progress
🔗 https://github.com/users/myuser/projects/1
```