//! GitHub GraphQL queries for reading issues and Projects v2 data
//!
//! This module provides functions to:
//! - Fetch issues from repositories
//! - Query Projects v2 custom fields and status options
//! - Retrieve project items and their field values
//! - Find project items by issue ID
//!
//! # Query Categories
//!
//! ## Issue Queries
//! - [`GitHubQueries::get_repository_issues`] - Fetch all repository issues
//! - [`GitHubQueries::get_issue_by_number`] - Get specific issue by number
//! - [`GitHubQueries::get_issue_by_id`] - Get issue by GraphQL node ID
//! - [`GitHubQueries::get_issue_id`] - Convert issue number to node ID
//!
//! ## Projects v2 Queries
//! - [`GitHubQueries::get_project_fields`] - Discover all custom fields
//! - [`GitHubQueries::get_project_item`] - Get item with field values
//! - [`GitHubQueries::get_project_items_by_issue`] - Find items by issue
//!
//! # Example
//!
//! ```no_run
//! use taskguard::github::{GitHubClient, GitHubQueries};
//!
//! let client = GitHubClient::new()?;
//!
//! // Get repository issues
//! let issues = GitHubQueries::get_repository_issues(
//!     &client,
//!     "owner",
//!     "repo",
//!     Some(50)
//! )?;
//!
//! // Get specific issue
//! let issue = GitHubQueries::get_issue_by_number(&client, "owner", "repo", 42)?;
//!
//! // Get project fields
//! let fields = GitHubQueries::get_project_fields(&client, "project_id")?;
//! # Ok::<(), anyhow::Error>(())
//! ```

use anyhow::{Context, Result};
use serde_json::json;

use super::client::GitHubClient;
use super::types::{FieldValue, GitHubIssue, ProjectItem};

/// Project field definition
#[derive(Debug, Clone)]
pub struct ProjectField {
    /// Field ID
    pub id: String,
    /// Field name (e.g., "Status", "Priority")
    pub name: String,
    /// Field data type
    pub data_type: String,
    /// Available options (for single-select fields)
    pub options: Vec<(String, String)>, // (id, name) pairs
}

/// GitHub queries for issues and Projects v2
pub struct GitHubQueries;

impl GitHubQueries {
    // ========================================
    // ISSUE QUERIES
    // ========================================

    /// Get all issues from a repository
    ///
    /// # Arguments
    ///
    /// * `client` - Authenticated GitHub client
    /// * `owner` - Repository owner (username or organization)
    /// * `repo` - Repository name
    /// * `limit` - Maximum number of issues to fetch (default: 100)
    ///
    /// # Returns
    ///
    /// Vector of issues ordered by last updated (most recent first)
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Repository not found
    /// - Network request fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// use taskguard::github::{GitHubClient, GitHubQueries};
    ///
    /// let client = GitHubClient::new()?;
    /// let issues = GitHubQueries::get_repository_issues(&client, "owner", "repo", Some(50))?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
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
                            labels(first: 10) {
                                nodes {
                                    name
                                }
                            }
                            assignees(first: 10) {
                                nodes {
                                    login
                                }
                            }
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

        let response = client
            .query(query, variables)
            .context("Failed to get repository issues")?;

        let nodes = response["data"]["repository"]["issues"]["nodes"]
            .as_array()
            .context("Invalid issues response")?;

        let issues: Vec<GitHubIssue> = nodes
            .iter()
            .filter_map(|node| {
                let labels = node["labels"]["nodes"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|label| label["name"].as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default();

                let assignees = node["assignees"]["nodes"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|user| user["login"].as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default();

                Some(GitHubIssue {
                    id: node["id"].as_str()?.to_string(),
                    number: node["number"].as_i64()?,
                    title: node["title"].as_str()?.to_string(),
                    state: node["state"].as_str()?.to_string(),
                    body: node["body"].as_str().map(|s| s.to_string()),
                    labels,
                    assignees,
                })
            })
            .collect();

        Ok(issues)
    }

    /// Get a specific issue by number
    ///
    /// # Arguments
    ///
    /// * `client` - Authenticated GitHub client
    /// * `owner` - Repository owner (username or organization)
    /// * `repo` - Repository name
    /// * `number` - Issue number
    ///
    /// # Returns
    ///
    /// The requested issue
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Repository not found
    /// - Issue not found
    /// - Network request fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// use taskguard::github::{GitHubClient, GitHubQueries};
    ///
    /// let client = GitHubClient::new()?;
    /// let issue = GitHubQueries::get_issue_by_number(&client, "owner", "repo", 42)?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn get_issue_by_number(
        client: &GitHubClient,
        owner: &str,
        repo: &str,
        number: i64,
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
                        labels(first: 10) {
                            nodes {
                                name
                            }
                        }
                        assignees(first: 10) {
                            nodes {
                                login
                            }
                        }
                    }
                }
            }
        "#;

        let variables = json!({
            "owner": owner,
            "name": repo,
            "number": number,
        });

        let response = client
            .query(query, variables)
            .context("Failed to get issue")?;

        let issue_data = response["data"]["repository"]["issue"]
            .as_object()
            .context("Issue not found")?;

        let labels = issue_data["labels"]["nodes"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|label| label["name"].as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let assignees = issue_data["assignees"]["nodes"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|user| user["login"].as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        Ok(GitHubIssue {
            id: issue_data["id"]
                .as_str()
                .context("Missing issue ID")?
                .to_string(),
            number: issue_data["number"]
                .as_i64()
                .context("Missing issue number")?,
            title: issue_data["title"]
                .as_str()
                .context("Missing issue title")?
                .to_string(),
            state: issue_data["state"]
                .as_str()
                .context("Missing issue state")?
                .to_string(),
            body: issue_data["body"].as_str().map(|s| s.to_string()),
            labels,
            assignees,
        })
    }

    /// Get issue by GraphQL node ID
    ///
    /// # Arguments
    ///
    /// * `client` - Authenticated GitHub client
    /// * `issue_id` - GraphQL node ID
    ///
    /// # Returns
    ///
    /// The requested issue
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Issue not found
    /// - Invalid ID format
    /// - Network request fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// use taskguard::github::{GitHubClient, GitHubQueries};
    ///
    /// let client = GitHubClient::new()?;
    /// let issue = GitHubQueries::get_issue_by_id(&client, "I_node_id")?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn get_issue_by_id(client: &GitHubClient, issue_id: &str) -> Result<GitHubIssue> {
        let query = r#"
            query($id: ID!) {
                node(id: $id) {
                    ... on Issue {
                        id
                        number
                        title
                        state
                        body
                        labels(first: 10) {
                            nodes {
                                name
                            }
                        }
                        assignees(first: 10) {
                            nodes {
                                login
                            }
                        }
                    }
                }
            }
        "#;

        let variables = json!({ "id": issue_id });

        let response = client
            .query(query, variables)
            .context("Failed to get issue by ID")?;

        let node = response["data"]["node"]
            .as_object()
            .context("Issue not found")?;

        let labels = node["labels"]["nodes"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|label| label["name"].as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let assignees = node["assignees"]["nodes"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|user| user["login"].as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        Ok(GitHubIssue {
            id: node["id"].as_str().context("Missing issue ID")?.to_string(),
            number: node["number"].as_i64().context("Missing issue number")?,
            title: node["title"]
                .as_str()
                .context("Missing issue title")?
                .to_string(),
            state: node["state"]
                .as_str()
                .context("Missing issue state")?
                .to_string(),
            body: node["body"].as_str().map(|s| s.to_string()),
            labels,
            assignees,
        })
    }

    /// Helper: Convert issue number to GraphQL node ID
    ///
    /// # Arguments
    ///
    /// * `client` - Authenticated GitHub client
    /// * `owner` - Repository owner
    /// * `repo` - Repository name
    /// * `issue_number` - Issue number
    ///
    /// # Returns
    ///
    /// The GraphQL node ID for the issue
    ///
    /// # Example
    ///
    /// ```no_run
    /// use taskguard::github::{GitHubClient, GitHubQueries};
    ///
    /// let client = GitHubClient::new()?;
    /// let node_id = GitHubQueries::get_issue_id(&client, "owner", "repo", 42)?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn get_issue_id(
        client: &GitHubClient,
        owner: &str,
        repo: &str,
        issue_number: i64,
    ) -> Result<String> {
        let issue = Self::get_issue_by_number(client, owner, repo, issue_number)?;
        Ok(issue.id)
    }

    /// Search for issues containing a specific TaskGuard ID in their body
    ///
    /// # Arguments
    ///
    /// * `client` - Authenticated GitHub client
    /// * `owner` - Repository owner
    /// * `repo` - Repository name
    /// * `taskguard_id` - The TaskGuard task ID to search for
    ///
    /// # Returns
    ///
    /// Vector of matching issues
    ///
    /// # Example
    ///
    /// ```no_run
    /// use taskguard::github::{GitHubClient, GitHubQueries};
    ///
    /// let client = GitHubClient::new()?;
    /// let issues = GitHubQueries::search_issues_by_taskguard_id(
    ///     &client, "owner", "repo", "backend-001"
    /// )?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn search_issues_by_taskguard_id(
        client: &GitHubClient,
        owner: &str,
        repo: &str,
        taskguard_id: &str,
    ) -> Result<Vec<GitHubIssue>> {
        // Use GitHub's search query to find issues with the TaskGuard ID in body
        let query = r#"
            query($searchQuery: String!) {
                search(query: $searchQuery, type: ISSUE, first: 5) {
                    nodes {
                        ... on Issue {
                            id
                            number
                            title
                            state
                            body
                            labels(first: 10) {
                                nodes {
                                    name
                                }
                            }
                            assignees(first: 10) {
                                nodes {
                                    login
                                }
                            }
                        }
                    }
                }
            }
        "#;

        let search_query = format!(
            "repo:{}/{} in:body \"**TaskGuard ID:** {}\"",
            owner, repo, taskguard_id
        );

        let variables = json!({ "searchQuery": search_query });

        let response = client
            .query(query, variables)
            .context("Failed to search for TaskGuard issues")?;

        let nodes = response["data"]["search"]["nodes"]
            .as_array()
            .unwrap_or(&Vec::new())
            .clone();

        let issues: Vec<GitHubIssue> = nodes
            .iter()
            .filter_map(|node| {
                // Skip if not a valid issue node
                if node["id"].is_null() {
                    return None;
                }

                let labels = node["labels"]["nodes"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|label| label["name"].as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default();

                let assignees = node["assignees"]["nodes"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|user| user["login"].as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default();

                Some(GitHubIssue {
                    id: node["id"].as_str()?.to_string(),
                    number: node["number"].as_i64()?,
                    title: node["title"].as_str()?.to_string(),
                    state: node["state"].as_str()?.to_string(),
                    body: node["body"].as_str().map(|s| s.to_string()),
                    labels,
                    assignees,
                })
            })
            .collect();

        Ok(issues)
    }

    // ========================================
    // PROJECTS V2 QUERIES
    // ========================================

    /// Get all custom fields from a GitHub Projects v2 board
    ///
    /// # Arguments
    ///
    /// * `client` - Authenticated GitHub client
    /// * `project_node_id` - GraphQL node ID of the project
    ///
    /// # Returns
    ///
    /// Vector of project fields with their IDs, names, types, and options
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Project not found
    /// - Network request fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// use taskguard::github::{GitHubClient, GitHubQueries};
    ///
    /// let client = GitHubClient::new()?;
    /// let fields = GitHubQueries::get_project_fields(&client, "project_id")?;
    ///
    /// for field in fields {
    ///     println!("{}: {} ({:?})", field.name, field.id, field.data_type);
    /// }
    /// # Ok::<(), anyhow::Error>(())
    /// ```
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
        let response = client
            .query(query, variables)
            .context("Failed to get project fields")?;

        let nodes = response["data"]["node"]["fields"]["nodes"]
            .as_array()
            .context("Invalid fields response")?;

        let fields: Vec<ProjectField> = nodes
            .iter()
            .filter_map(|node| {
                let id = node["id"].as_str()?.to_string();
                let name = node["name"].as_str()?.to_string();
                let data_type = node["dataType"].as_str()?.to_string();

                // Extract options if this is a single-select field
                let options = if let Some(opts) = node["options"].as_array() {
                    opts.iter()
                        .filter_map(|opt| {
                            let opt_id = opt["id"].as_str()?.to_string();
                            let opt_name = opt["name"].as_str()?.to_string();
                            Some((opt_id, opt_name))
                        })
                        .collect()
                } else {
                    Vec::new()
                };

                Some(ProjectField {
                    id,
                    name,
                    data_type,
                    options,
                })
            })
            .collect();

        Ok(fields)
    }

    /// Get a project item with its field values
    ///
    /// # Arguments
    ///
    /// * `client` - Authenticated GitHub client
    /// * `project_item_id` - GraphQL node ID of the project item
    ///
    /// # Returns
    ///
    /// Project item with all field values
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Item not found
    /// - Network request fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// use taskguard::github::{GitHubClient, GitHubQueries};
    ///
    /// let client = GitHubClient::new()?;
    /// let item = GitHubQueries::get_project_item(&client, "item_id")?;
    /// println!("Status: {}", item.status);
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn get_project_item(client: &GitHubClient, project_item_id: &str) -> Result<ProjectItem> {
        let query = r#"
            query($itemId: ID!) {
                node(id: $itemId) {
                    ... on ProjectV2Item {
                        id
                        project {
                            id
                        }
                        content {
                            ... on Issue {
                                id
                            }
                        }
                        fieldValues(first: 20) {
                            nodes {
                                ... on ProjectV2ItemFieldSingleSelectValue {
                                    name
                                    field {
                                        ... on ProjectV2SingleSelectField {
                                            id
                                            name
                                        }
                                    }
                                }
                                ... on ProjectV2ItemFieldTextValue {
                                    text
                                    field {
                                        ... on ProjectV2Field {
                                            id
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
        let response = client
            .query(query, variables)
            .context("Failed to get project item")?;

        let item_data = response["data"]["node"]
            .as_object()
            .context("Project item not found")?;

        let project_id = item_data["project"]["id"]
            .as_str()
            .context("Missing project ID")?
            .to_string();

        let issue_id = item_data["content"]["id"]
            .as_str()
            .context("Missing issue ID")?
            .to_string();

        // Parse field values
        let field_nodes = item_data["fieldValues"]["nodes"]
            .as_array()
            .context("Missing field values")?;

        let mut status = String::new();
        let mut field_values = Vec::new();

        for node in field_nodes {
            if let Some(field) = node["field"].as_object() {
                let field_id = field["id"].as_str().unwrap_or("").to_string();
                let field_name = field["name"].as_str().unwrap_or("");

                // Get the value (either name for single-select or text for text fields)
                let value = if let Some(name) = node["name"].as_str() {
                    name.to_string()
                } else if let Some(text) = node["text"].as_str() {
                    text.to_string()
                } else {
                    continue;
                };

                // If this is the Status field, save it separately
                if field_name.eq_ignore_ascii_case("status") {
                    status = value.clone();
                }

                field_values.push(FieldValue { field_id, value });
            }
        }

        Ok(ProjectItem {
            id: project_item_id.to_string(),
            issue_id,
            project_id,
            status,
            field_values,
        })
    }

    /// Find project items by issue ID
    ///
    /// # Arguments
    ///
    /// * `client` - Authenticated GitHub client
    /// * `project_node_id` - GraphQL node ID of the project
    /// * `issue_node_id` - GraphQL node ID of the issue
    ///
    /// # Returns
    ///
    /// Vector of project items that match the issue
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Project not found
    /// - Network request fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// use taskguard::github::{GitHubClient, GitHubQueries};
    ///
    /// let client = GitHubClient::new()?;
    /// let items = GitHubQueries::get_project_items_by_issue(
    ///     &client,
    ///     "project_id",
    ///     "issue_id"
    /// )?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn get_project_items_by_issue(
        client: &GitHubClient,
        project_node_id: &str,
        issue_node_id: &str,
    ) -> Result<Vec<ProjectItem>> {
        let query = r#"
            query($projectId: ID!) {
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

        let variables = json!({ "projectId": project_node_id });
        let response = client
            .query(query, variables)
            .context("Failed to get project items")?;

        let nodes = response["data"]["node"]["items"]["nodes"]
            .as_array()
            .context("Invalid items response")?;

        // Filter items that match the issue_node_id and fetch full details
        let mut matching_items = Vec::new();

        for node in nodes {
            if let Some(content_id) = node["content"]["id"].as_str()
                && content_id == issue_node_id
                && let Some(item_id) = node["id"].as_str()
            {
                // Fetch full item details
                let item = Self::get_project_item(client, item_id)?;
                matching_items.push(item);
            }
        }

        Ok(matching_items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests require a real GitHub token and repository
    // They are marked as #[ignore] by default

    #[test]
    #[ignore]
    fn test_get_repository_issues() {
        let client = GitHubClient::new().expect("Failed to create client");
        let result = GitHubQueries::get_repository_issues(&client, "octocat", "Hello-World", None);
        assert!(result.is_ok());
    }

    #[test]
    #[ignore]
    fn test_get_issue_by_number() {
        let client = GitHubClient::new().expect("Failed to create client");
        // Use Guard8-ai/TaskGuard issue #1 (or any existing issue)
        let result = GitHubQueries::get_issue_by_number(&client, "Guard8-ai", "TaskGuard", 95);
        assert!(result.is_ok(), "Failed to get issue: {:?}", result);
    }
}
