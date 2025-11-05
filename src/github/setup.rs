//! GitHub Projects v2 automatic setup and configuration
//!
//! This module provides zero-config setup for GitHub integration with `--yes` flag support
//! for AI agents and automation.
//!
//! # Features
//!
//! - Automatic project detection and creation
//! - Organization vs user account detection
//! - Repository linking
//! - Configuration file management
//! - AI-agent-friendly (no interactive prompts with `--yes`)
//!
//! # Example
//!
//! ```no_run
//! use taskguard::github::{GitHubClient, GitHubProjectSetup};
//!
//! let client = GitHubClient::new()?;
//!
//! // Auto-create project with progress messages
//! let (project_number, project_id) = GitHubProjectSetup::auto_create_project(
//!     &client,
//!     "owner",
//!     "repo",
//!     true  // verbose
//! )?;
//!
//! println!("Created project #{}", project_number);
//! # Ok::<(), anyhow::Error>(())
//! ```

use anyhow::{Context, Result};
use serde_json::json;

use super::client::GitHubClient;
use super::config::get_github_config_path;
use super::mutations::GitHubMutations;
use super::types::GitHubConfig;

/// GitHub Projects v2 setup automation
pub struct GitHubProjectSetup;

impl GitHubProjectSetup {
    // ========================================
    // PUBLIC API
    // ========================================

    /// Check if a GitHub Projects board is configured and exists
    ///
    /// # Arguments
    ///
    /// * `client` - Authenticated GitHub client
    /// * `config` - GitHub configuration with project_number
    ///
    /// # Returns
    ///
    /// - `Ok(Some(project_id))` if project exists
    /// - `Ok(None)` if no project configured or doesn't exist
    /// - `Err` if query fails unexpectedly
    ///
    /// # Example
    ///
    /// ```no_run
    /// use taskguard::github::{GitHubClient, GitHubProjectSetup, load_github_config};
    ///
    /// let client = GitHubClient::new()?;
    /// let config = load_github_config()?;
    ///
    /// match GitHubProjectSetup::check_project_exists(&client, &config)? {
    ///     Some(id) => println!("Project exists: {}", id),
    ///     None => println!("No project found"),
    /// }
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn check_project_exists(
        client: &GitHubClient,
        config: &GitHubConfig,
    ) -> Result<Option<String>> {
        match Self::get_project_id(client, &config.owner, config.project_number) {
            Ok(id) => Ok(Some(id)),
            Err(_) => Ok(None),
        }
    }

    /// Auto-create a GitHub Projects v2 board with full setup
    ///
    /// This function performs complete project setup:
    /// 1. Detects if owner is organization or user
    /// 2. Creates project with default settings
    /// 3. Links project to repository
    /// 4. Verifies default status columns exist
    /// 5. Updates `.taskguard/github.toml` configuration
    ///
    /// # Arguments
    ///
    /// * `client` - Authenticated GitHub client
    /// * `owner` - Repository owner (organization or user)
    /// * `repo` - Repository name
    /// * `verbose` - Print progress messages to stdout
    ///
    /// # Returns
    ///
    /// Tuple of `(project_number, project_id)`
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - User lacks permissions to create projects
    /// - Repository not found
    /// - Network request fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// use taskguard::github::{GitHubClient, GitHubProjectSetup};
    ///
    /// let client = GitHubClient::new()?;
    ///
    /// let (number, id) = GitHubProjectSetup::auto_create_project(
    ///     &client,
    ///     "Guard8-ai",
    ///     "TaskGuard",
    ///     true  // Show progress
    /// )?;
    ///
    /// println!("Created project #{}: {}", number, id);
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn auto_create_project(
        client: &GitHubClient,
        owner: &str,
        repo: &str,
        verbose: bool,
    ) -> Result<(i64, String)> {
        if verbose {
            println!("✨ Creating GitHub Projects board...");
        }

        // 1. Determine if owner is organization or user
        let is_org = Self::is_organization(client, owner)?;

        if verbose {
            if is_org {
                println!("✓ Detected organization: {}", owner);
            } else {
                println!("✓ Detected user account: {}", owner);
            }
        }

        // 2. Create project
        let project_name = format!("{} Tasks", repo);
        let (project_number, project_id) =
            Self::create_project(client, owner, &project_name, is_org)?;

        if verbose {
            println!(
                "✓ Created project: \"{}\" (#{})",
                project_name, project_number
            );
        }

        // 3. Link to repository
        let repo_id = Self::get_repository_id(client, owner, repo)?;
        Self::link_project_to_repository(client, &project_id, &repo_id)?;

        if verbose {
            println!("✓ Linked to repository: {}/{}", owner, repo);
        }

        // 4. Setup status columns for TaskGuard (add In Review and Blocked)
        if verbose {
            println!("⚙️  Setting up status columns...");
        }

        match GitHubMutations::ensure_status_columns(client, &project_id) {
            Ok(created) => {
                // Get updated column list
                if let Ok((_, options)) = GitHubMutations::get_status_field_info(client, &project_id) {
                    let column_names: Vec<_> = options.iter().map(|(_, name)| name.as_str()).collect();
                    if verbose {
                        println!("✓ Status columns ready: {}", column_names.join(", "));
                        if created > 0 {
                            println!("  (Added {} TaskGuard-specific columns)", created);
                        }
                    }
                }
            }
            Err(e) => {
                if verbose {
                    println!("⚠️  Could not configure all status columns: {}", e);
                    println!("   You may need to manually add 'In Review' and 'Blocked' columns");
                }
            }
        }

        // 5. Update github.toml
        Self::update_config_file(owner, repo, project_number)?;

        if verbose {
            println!("✓ Saved configuration to .taskguard/github.toml");

            let project_url = if is_org {
                format!(
                    "https://github.com/orgs/{}/projects/{}",
                    owner, project_number
                )
            } else {
                format!(
                    "https://github.com/users/{}/projects/{}",
                    owner, project_number
                )
            };

            println!("\n🚀 Project ready! View at: {}\n", project_url);
        }

        Ok((project_number, project_id))
    }

    /// Print setup instructions for users (when --yes not provided)
    ///
    /// # Arguments
    ///
    /// * `owner` - Repository owner
    /// * `repo` - Repository name
    ///
    /// # Example
    ///
    /// ```no_run
    /// use taskguard::github::GitHubProjectSetup;
    ///
    /// GitHubProjectSetup::print_setup_instructions("owner", "repo");
    /// ```
    pub fn print_setup_instructions(owner: &str, repo: &str) {
        println!("⚠️  No GitHub Projects board found for this repository.\n");
        println!("   TaskGuard can create one with:");
        println!("   - Name: \"{} Tasks\"", repo);
        println!("   - Default columns: Todo, In Progress, In Review, Done");
        println!("   - Linked to: {}/{}\n", owner, repo);
        println!("   Run with --yes to create automatically:");
        println!("   $ taskguard sync <task-id> --github --yes\n");
    }

    // ========================================
    // PRIVATE HELPER FUNCTIONS
    // ========================================

    /// Check if owner is an organization (vs user account)
    fn is_organization(client: &GitHubClient, owner: &str) -> Result<bool> {
        let query = r#"
            query($login: String!) {
                organization(login: $login) {
                    login
                }
            }
        "#;

        let variables = json!({ "login": owner });
        Ok(client.query(query, variables).is_ok())
    }

    /// Create project using GraphQL mutation
    fn create_project(
        client: &GitHubClient,
        owner: &str,
        title: &str,
        is_org: bool,
    ) -> Result<(i64, String)> {
        let mutation = r#"
            mutation($ownerId: ID!, $title: String!) {
                createProjectV2(input: {
                    ownerId: $ownerId,
                    title: $title
                }) {
                    projectV2 {
                        id
                        number
                    }
                }
            }
        "#;

        // Get owner ID
        let owner_id = if is_org {
            Self::get_organization_id(client, owner)?
        } else {
            Self::get_user_id(client, owner)?
        };

        let variables = json!({
            "ownerId": owner_id,
            "title": title,
        });

        let response = client
            .query(mutation, variables)
            .context("Failed to create project")?;

        let project = &response["data"]["createProjectV2"]["projectV2"];
        let project_id = project["id"]
            .as_str()
            .context("Missing project ID")?
            .to_string();
        let project_number = project["number"]
            .as_i64()
            .context("Missing project number")?;

        Ok((project_number, project_id))
    }

    /// Get organization GraphQL node ID
    fn get_organization_id(client: &GitHubClient, login: &str) -> Result<String> {
        let query = r#"
            query($login: String!) {
                organization(login: $login) { id }
            }
        "#;

        let variables = json!({ "login": login });
        let response = client.query(query, variables)?;

        Ok(response["data"]["organization"]["id"]
            .as_str()
            .context("Missing organization ID")?
            .to_string())
    }

    /// Get user GraphQL node ID
    fn get_user_id(client: &GitHubClient, login: &str) -> Result<String> {
        let query = r#"
            query($login: String!) {
                user(login: $login) { id }
            }
        "#;

        let variables = json!({ "login": login });
        let response = client.query(query, variables)?;

        Ok(response["data"]["user"]["id"]
            .as_str()
            .context("Missing user ID")?
            .to_string())
    }

    /// Get repository GraphQL node ID
    fn get_repository_id(client: &GitHubClient, owner: &str, repo: &str) -> Result<String> {
        let query = r#"
            query($owner: String!, $name: String!) {
                repository(owner: $owner, name: $name) { id }
            }
        "#;

        let variables = json!({
            "owner": owner,
            "name": repo,
        });

        let response = client.query(query, variables)?;

        Ok(response["data"]["repository"]["id"]
            .as_str()
            .context("Missing repository ID")?
            .to_string())
    }

    /// Link project to repository for visibility
    fn link_project_to_repository(
        client: &GitHubClient,
        project_id: &str,
        repository_id: &str,
    ) -> Result<()> {
        let mutation = r#"
            mutation($projectId: ID!, $repositoryId: ID!) {
                linkProjectV2ToRepository(input: {
                    projectId: $projectId,
                    repositoryId: $repositoryId
                }) {
                    repository { id }
                }
            }
        "#;

        let variables = json!({
            "projectId": project_id,
            "repositoryId": repository_id,
        });

        client
            .query(mutation, variables)
            .context("Failed to link project to repository")?;

        Ok(())
    }

    /// Get project GraphQL node ID from owner and number
    pub fn get_project_id(client: &GitHubClient, owner: &str, project_number: i64) -> Result<String> {
        // Try organization first
        let query = r#"
            query($owner: String!, $number: Int!) {
                organization(login: $owner) {
                    projectV2(number: $number) {
                        id
                    }
                }
            }
        "#;

        let variables = json!({
            "owner": owner,
            "number": project_number,
        });

        if let Ok(response) = client.query(query, variables.clone()) {
            if let Some(id) = response["data"]["organization"]["projectV2"]["id"].as_str() {
                return Ok(id.to_string());
            }
        }

        // Try user account
        let query = r#"
            query($owner: String!, $number: Int!) {
                user(login: $owner) {
                    projectV2(number: $number) {
                        id
                    }
                }
            }
        "#;

        let response = client
            .query(query, variables)
            .context("Project not found for organization or user")?;

        Ok(response["data"]["user"]["projectV2"]["id"]
            .as_str()
            .context("Missing project ID")?
            .to_string())
    }

    /// Update .taskguard/github.toml with new project number
    fn update_config_file(owner: &str, repo: &str, project_number: i64) -> Result<()> {
        let config_path = get_github_config_path()?;

        // Ensure .taskguard directory exists
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)
                .context("Failed to create .taskguard directory")?;
        }

        let config_content = format!(
            "owner = \"{}\"\nrepo = \"{}\"\nproject_number = {}\n",
            owner, repo, project_number
        );

        std::fs::write(&config_path, config_content).context("Failed to write github.toml")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_check_project_exists() {
        // Requires real GitHub authentication
        let client = GitHubClient::new().expect("Failed to create client");
        let config = GitHubConfig {
            owner: "your-username".to_string(),
            repo: "your-repo".to_string(),
            project_number: 1,
        };

        let result = GitHubProjectSetup::check_project_exists(&client, &config);
        assert!(result.is_ok());
    }

    #[test]
    #[ignore]
    fn test_auto_create_project() {
        // Requires real GitHub authentication and write permissions
        let client = GitHubClient::new().expect("Failed to create client");

        let result =
            GitHubProjectSetup::auto_create_project(&client, "your-username", "your-repo", true);

        assert!(result.is_ok());
    }
}
