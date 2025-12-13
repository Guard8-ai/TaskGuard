//! GitHub GraphQL mutations for creating and updating issues and project items
//!
//! This module provides functions to:
//! - Create and update GitHub issues
//! - Add issues to GitHub Projects v2 boards
//! - Update project item status fields
//! - Query project field information for status updates
//!
//! # Projects v2 Integration Flow
//!
//! The typical flow for syncing a task to GitHub Projects:
//!
//! 1. Create issue with [`GitHubMutations::create_issue`]
//! 2. Add to project with [`GitHubMutations::add_issue_to_project`] → Get item_id
//! 3. Query field info with [`GitHubMutations::get_status_field_info`] → Get field_id and option_ids
//! 4. Use mapper to find best status option
//! 5. Update status with [`GitHubMutations::update_project_item_status`]
//!
//! # Example
//!
//! ```no_run
//! use taskguard::github::{GitHubClient, GitHubMutations, TaskIssueMapper};
//! use taskguard::task::TaskStatus;
//!
//! let client = GitHubClient::new()?;
//!
//! // 1. Create issue
//! let issue = GitHubMutations::create_issue(
//!     &client,
//!     "owner",
//!     "repo",
//!     "Implement feature",
//!     Some("Task description")
//! )?;
//!
//! // 2. Add to project
//! let item_id = GitHubMutations::add_issue_to_project(
//!     &client,
//!     "project_id",
//!     &issue.id
//! )?;
//!
//! // 3. Get status field info
//! let (field_id, options) = GitHubMutations::get_status_field_info(
//!     &client,
//!     "project_id"
//! )?;
//!
//! // 4. Find best status option
//! let option_id = TaskIssueMapper::find_best_status_option(
//!     &TaskStatus::Doing,
//!     &options
//! ).expect("No matching status column");
//!
//! // 5. Update project item status
//! GitHubMutations::update_project_item_status(
//!     &client,
//!     "project_id",
//!     &item_id,
//!     &field_id,
//!     &option_id
//! )?;
//! # Ok::<(), anyhow::Error>(())
//! ```

use anyhow::{Context, Result};
use serde_json::json;

use super::client::GitHubClient;
use super::types::GitHubIssue;

/// GitHub mutations for issues and Projects v2
pub struct GitHubMutations;

impl GitHubMutations {
    // ========================================
    // ISSUE MUTATIONS
    // ========================================

    /// Create a new GitHub issue
    ///
    /// # Arguments
    ///
    /// * `client` - Authenticated GitHub client
    /// * `owner` - Repository owner (username or organization)
    /// * `repo` - Repository name
    /// * `title` - Issue title
    /// * `body` - Optional issue description
    ///
    /// # Returns
    ///
    /// The created issue with all metadata
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Repository not found
    /// - User lacks write permissions
    /// - Network request fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// use taskguard::github::{GitHubClient, GitHubMutations};
    ///
    /// let client = GitHubClient::new()?;
    /// let issue = GitHubMutations::create_issue(
    ///     &client,
    ///     "myuser",
    ///     "myrepo",
    ///     "Implement authentication",
    ///     Some("Add JWT-based authentication")
    /// )?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
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
            "repositoryId": repo_id,
            "title": title,
            "body": body.unwrap_or(""),
        });

        let response = client
            .query(mutation, variables)
            .context("Failed to create issue")?;

        // Parse response
        let issue_data = response["data"]["createIssue"]["issue"]
            .as_object()
            .context("Invalid issue response")?;

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
            number: issue_data["number"].as_i64().context("Missing issue number")?,
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

    /// Update issue state (open or close)
    ///
    /// # Arguments
    ///
    /// * `client` - Authenticated GitHub client
    /// * `issue_id` - GraphQL node ID of the issue
    /// * `state` - Desired state: "OPEN" or "CLOSED"
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Invalid state provided (not "OPEN" or "CLOSED")
    /// - Issue not found
    /// - Network request fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// use taskguard::github::{GitHubClient, GitHubMutations};
    ///
    /// let client = GitHubClient::new()?;
    /// GitHubMutations::update_issue_state(&client, "issue_id", "CLOSED")?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
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

        client
            .query(mutation, variables)
            .context("Failed to update issue state")?;

        Ok(())
    }

    /// Update issue title
    ///
    /// # Arguments
    ///
    /// * `client` - Authenticated GitHub client
    /// * `issue_id` - GraphQL node ID of the issue
    /// * `title` - New issue title
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Issue not found
    /// - Network request fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// use taskguard::github::{GitHubClient, GitHubMutations};
    ///
    /// let client = GitHubClient::new()?;
    /// GitHubMutations::update_issue_title(&client, "issue_id", "Updated title")?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn update_issue_title(client: &GitHubClient, issue_id: &str, title: &str) -> Result<()> {
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

        client
            .query(mutation, variables)
            .context("Failed to update issue title")?;

        Ok(())
    }

    /// Update issue body
    ///
    /// # Arguments
    ///
    /// * `client` - Authenticated GitHub client
    /// * `issue_id` - GraphQL node ID of the issue
    /// * `body` - New issue body/description
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Issue not found
    /// - Network request fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// use taskguard::github::{GitHubClient, GitHubMutations};
    ///
    /// let client = GitHubClient::new()?;
    /// GitHubMutations::update_issue_body(&client, "issue_id", "Updated description")?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn update_issue_body(client: &GitHubClient, issue_id: &str, body: &str) -> Result<()> {
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

        client
            .query(mutation, variables)
            .context("Failed to update issue body")?;

        Ok(())
    }

    // ========================================
    // PROJECTS V2 MUTATIONS (HIGH PRIORITY)
    // ========================================

    /// Add an issue to a GitHub Projects v2 board
    ///
    /// This is a CORE mutation for Projects v2 integration.
    /// After creating an issue, you must add it to the project board
    /// to make it visible on the dashboard.
    ///
    /// # Arguments
    ///
    /// * `client` - Authenticated GitHub client
    /// * `project_id` - GraphQL node ID of the Projects v2 board
    /// * `issue_id` - GraphQL node ID of the issue to add
    ///
    /// # Returns
    ///
    /// The project item ID, which is needed for status updates
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Project not found
    /// - Issue not found
    /// - Issue already in project (handled gracefully by GitHub)
    /// - User lacks project permissions
    /// - Network request fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// use taskguard::github::{GitHubClient, GitHubMutations};
    ///
    /// let client = GitHubClient::new()?;
    /// let item_id = GitHubMutations::add_issue_to_project(
    ///     &client,
    ///     "project_id",
    ///     "issue_id"
    /// )?;
    /// println!("Project item ID: {}", item_id);
    /// # Ok::<(), anyhow::Error>(())
    /// ```
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

        let response = client
            .query(mutation, variables)
            .context("Failed to add issue to project")?;

        let item_id = response["data"]["addProjectV2ItemById"]["item"]["id"]
            .as_str()
            .context("Missing project item ID in response")?
            .to_string();

        Ok(item_id)
    }

    /// Update project item status field
    ///
    /// This is a CORE mutation for Projects v2 dashboard visibility.
    /// Updates the status column value for a project item, making the task
    /// appear in the correct column on the GitHub Projects dashboard.
    ///
    /// # Arguments
    ///
    /// * `client` - Authenticated GitHub client
    /// * `project_id` - GraphQL node ID of the Projects v2 board
    /// * `item_id` - Project item ID (from `add_issue_to_project`)
    /// * `field_id` - Status field ID (from `get_status_field_info`)
    /// * `option_id` - Status option ID (from mapper's `find_best_status_option`)
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Project not found
    /// - Item not found
    /// - Field not found
    /// - Option not found
    /// - Network request fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// use taskguard::github::{GitHubClient, GitHubMutations, TaskIssueMapper};
    /// use taskguard::task::TaskStatus;
    ///
    /// let client = GitHubClient::new()?;
    ///
    /// // Get field info
    /// let (field_id, options) = GitHubMutations::get_status_field_info(&client, "project_id")?;
    ///
    /// // Find best status option
    /// let option_id = TaskIssueMapper::find_best_status_option(&TaskStatus::Doing, &options)
    ///     .expect("No matching status column");
    ///
    /// // Update status
    /// GitHubMutations::update_project_item_status(
    ///     &client,
    ///     "project_id",
    ///     "item_id",
    ///     &field_id,
    ///     &option_id
    /// )?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
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

        client
            .query(mutation, variables)
            .context("Failed to update project item status")?;

        Ok(())
    }

    /// Get status field info from a GitHub Projects v2 board
    ///
    /// This query retrieves the field ID and available options for the
    /// "Status" column, which are needed for updating project item status.
    ///
    /// # Arguments
    ///
    /// * `client` - Authenticated GitHub client
    /// * `project_id` - GraphQL node ID of the Projects v2 board
    ///
    /// # Returns
    ///
    /// A tuple of:
    /// - `field_id`: The GraphQL node ID of the Status field
    /// - `options`: Vector of (option_id, option_name) tuples for available status values
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Project not found
    /// - No "Status" field found (case-insensitive search)
    /// - Network request fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// use taskguard::github::{GitHubClient, GitHubMutations};
    ///
    /// let client = GitHubClient::new()?;
    /// let (field_id, options) = GitHubMutations::get_status_field_info(&client, "project_id")?;
    ///
    /// for (option_id, option_name) in options {
    ///     println!("{}: {}", option_name, option_id);
    /// }
    /// # Ok::<(), anyhow::Error>(())
    /// ```
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
        let response = client
            .query(query, variables)
            .context("Failed to get status field info")?;

        // Parse fields to find Status field
        let fields = response["data"]["node"]["fields"]["nodes"]
            .as_array()
            .context("Invalid fields response")?;

        // Find a field named "Status" (case-insensitive)
        for field in fields {
            if let Some(name) = field["name"].as_str() {
                if name.eq_ignore_ascii_case("status") {
                    let field_id = field["id"]
                        .as_str()
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

    /// Ensure all TaskGuard status columns exist on GitHub Projects v2 board
    ///
    /// This function checks the current status columns and creates any missing ones
    /// required by TaskGuard (todo, doing, review, done, blocked). This provides
    /// zero-configuration GitHub sync by automatically setting up the board.
    ///
    /// # Required Status Columns
    ///
    /// - "Backlog" or "Todo" (for todo status)
    /// - "In Progress" (for doing status)
    /// - "In Review" (for review status) ← **CREATED IF MISSING**
    /// - "Blocked" (for blocked status) ← **CREATED IF MISSING**
    /// - "Done" (for done status)
    ///
    /// # Arguments
    ///
    /// * `client` - Authenticated GitHub client
    /// * `project_id` - GraphQL node ID of the Projects v2 board
    ///
    /// # Returns
    ///
    /// Number of status columns created (0 if all exist)
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Project not found
    /// - Status field not found
    /// - API permission denied
    /// - Network request fails
    ///
    /// Note: This function provides informative warnings but doesn't fail if column
    /// creation is not possible. The sync will continue with existing columns.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use taskguard::github::{GitHubClient, GitHubMutations};
    ///
    /// let client = GitHubClient::new()?;
    /// let created = GitHubMutations::ensure_status_columns(&client, "project_id")?;
    /// println!("Created {} new status columns", created);
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn ensure_status_columns(
        client: &GitHubClient,
        project_id: &str,
    ) -> Result<usize> {
        use super::mapper::TaskIssueMapper;
        use crate::task::TaskStatus;

        // Get current status field and options
        let (field_id, existing_options) = Self::get_status_field_info(client, project_id)
            .context("Failed to get status field info")?;

        // Define required TaskGuard statuses with their preferred column names
        let required_statuses = vec![
            (TaskStatus::Todo, "Backlog"),
            (TaskStatus::Doing, "In Progress"),
            (TaskStatus::Review, "In Review"),
            (TaskStatus::Done, "Done"),
            (TaskStatus::Blocked, "Blocked"),
        ];

        // Check which statuses are missing
        let mut missing_columns = Vec::new();

        for (status, preferred_name) in required_statuses {
            if TaskIssueMapper::find_best_status_option(&status, &existing_options).is_none() {
                missing_columns.push((status, preferred_name));
            }
        }

        if missing_columns.is_empty() {
            return Ok(0);
        }

        // Create missing columns
        let mut created_count = 0;
        println!("   🔧 Creating missing status columns...");

        for (status, column_name) in missing_columns {
            match Self::create_status_column(client, project_id, &field_id, column_name) {
                Ok(_) => {
                    println!("      ✅ Created '{}' column for {:?} status", column_name, status);
                    created_count += 1;
                }
                Err(e) => {
                    // Don't fail the entire sync if column creation fails
                    // Just warn and continue with existing columns
                    println!("      ⚠️  Could not create '{}' column: {}", column_name, e);
                    println!("      💡 You may need to create this column manually on GitHub");
                }
            }
        }

        if created_count > 0 {
            println!("      🎉 Successfully created {} status column(s)", created_count);
        }

        Ok(created_count)
    }

    /// Create a new status column option on a GitHub Projects v2 board
    ///
    /// This is an internal helper function that adds a new option to the Status
    /// single-select field using the GitHub GraphQL API.
    ///
    /// Note: The GitHub API requires sending ALL existing options plus the new one.
    /// We fetch current options from the parent function.
    ///
    /// # Arguments
    ///
    /// * `client` - Authenticated GitHub client
    /// * `project_id` - GraphQL node ID of the Projects v2 board
    /// * `field_id` - GraphQL node ID of the Status field
    /// * `option_name` - Name for the new status column (e.g., "In Review")
    ///
    /// # Returns
    ///
    /// The GraphQL node ID of the newly created option
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - User lacks project write permissions
    /// - Option name already exists
    /// - Network request fails
    fn create_status_column(
        client: &GitHubClient,
        project_id: &str,
        field_id: &str,
        option_name: &str,
    ) -> Result<String> {
        // First, get current options so we can preserve them
        let (_, current_options) = Self::get_status_field_info(client, project_id)?;

        // Assign colors to status columns
        let color = match option_name {
            "In Review" => "YELLOW",
            "Blocked" => "RED",
            "Backlog" | "Todo" => "GRAY",
            "In Progress" => "BLUE",
            "Done" => "GREEN",
            _ => "GRAY",
        };

        let description = match option_name {
            "In Review" => "Tasks awaiting review",
            "Blocked" => "Tasks that are blocked",
            "Backlog" | "Todo" => "Tasks to do",
            "In Progress" => "Tasks in progress",
            "Done" => "Completed tasks",
            _ => "",
        };

        // Build option list: existing options + new option with proper fields
        let mut all_options: Vec<serde_json::Value> = current_options
            .iter()
            .map(|(_, name)| {
                // For existing options, provide name, color, and description
                let existing_color = match name.as_str() {
                    "Todo" => "GRAY",
                    "In Progress" => "BLUE",
                    "Done" => "GREEN",
                    _ => "GRAY",
                };
                let existing_desc = match name.as_str() {
                    "Todo" => "Tasks to do",
                    "In Progress" => "Tasks in progress",
                    "Done" => "Completed tasks",
                    _ => "",
                };
                json!({
                    "name": name,
                    "color": existing_color,
                    "description": existing_desc
                })
            })
            .collect();

        // Add the new option with all required fields
        all_options.push(json!({
            "name": option_name,
            "color": color,
            "description": description
        }));

        let mutation = r#"
            mutation($fieldId: ID!, $options: [ProjectV2SingleSelectFieldOptionInput!]!) {
                updateProjectV2Field(input: {
                    fieldId: $fieldId,
                    singleSelectOptions: $options
                }) {
                    projectV2Field {
                        ... on ProjectV2SingleSelectField {
                            id
                            options {
                                id
                                name
                            }
                        }
                    }
                }
            }
        "#;

        let variables = json!({
            "fieldId": field_id,
            "options": all_options
        });

        let response = client
            .query(mutation, variables)
            .context(format!("Failed to create status column '{}'", option_name))?;

        // Extract the new option ID from the response
        let options = response["data"]["updateProjectV2Field"]["projectV2Field"]["options"]
            .as_array()
            .context("Invalid response when creating status column")?;

        // Find the newly created option by name
        for opt in options {
            if opt["name"].as_str() == Some(option_name) {
                let option_id = opt["id"]
                    .as_str()
                    .context("Missing option ID")?
                    .to_string();
                return Ok(option_id);
            }
        }

        Err(anyhow::anyhow!("Created option not found in response"))
    }

    // ========================================
    // HELPER FUNCTIONS
    // ========================================

    /// Get repository GraphQL node ID from owner and name
    ///
    /// This is a helper function used internally by mutations that need
    /// the repository ID.
    ///
    /// # Arguments
    ///
    /// * `client` - Authenticated GitHub client
    /// * `owner` - Repository owner (username or organization)
    /// * `repo` - Repository name
    ///
    /// # Returns
    ///
    /// The GraphQL node ID of the repository
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Repository not found
    /// - Network request fails
    fn get_repository_id(client: &GitHubClient, owner: &str, repo: &str) -> Result<String> {
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

        let response = client
            .query(query, variables)
            .context("Failed to get repository ID")?;

        let repo_id = response["data"]["repository"]["id"]
            .as_str()
            .context("Missing repository ID in response")?;

        Ok(repo_id.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests require a real GitHub token and repository
    // They are marked as #[ignore] by default

    #[test]
    #[ignore]
    fn test_get_repository_id() {
        // This test requires gh CLI authentication
        let client = GitHubClient::new().expect("Failed to create client");
        let result = GitHubMutations::get_repository_id(&client, "octocat", "Hello-World");
        assert!(result.is_ok());
    }

    #[test]
    #[ignore]
    fn test_create_issue() {
        // This test requires gh CLI authentication and write access
        let client = GitHubClient::new().expect("Failed to create client");
        let result = GitHubMutations::create_issue(
            &client,
            "your-username",
            "your-repo",
            "Test Issue",
            Some("Test body"),
        );
        assert!(result.is_ok());
    }
}
