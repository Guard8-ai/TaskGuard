use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde_json::Value;
use std::process::Command;

/// GitHub GraphQL API client
/// Authentication is handled via `gh` CLI for better UX
pub struct GitHubClient {
    client: Client,
    token: String,
    api_url: String,
}

impl GitHubClient {
    /// Create a new GitHub client using `gh` CLI authentication
    ///
    /// This is the ONLY supported authentication method for simplicity.
    /// Users must have the `gh` CLI installed and authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - `gh` CLI is not installed
    /// - User is not authenticated (needs to run `gh auth login`)
    /// - Token retrieval fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// use taskguard::github::client::GitHubClient;
    ///
    /// let client = GitHubClient::new()?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn new() -> Result<Self> {
        // Get token from `gh auth token` command
        let output = Command::new("gh")
            .args(["auth", "token"])
            .output()
            .context("Failed to run 'gh auth token'. Is gh CLI installed?")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!(
                "GitHub authentication failed.\n\n\
                Please run: gh auth login\n\n\
                Error: {}",
                stderr
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
            .user_agent("TaskGuard/0.3.0")
            .build()
            .context("Failed to create HTTP client")?;

        Ok(GitHubClient {
            client,
            token,
            api_url: "https://api.github.com/graphql".to_string(),
        })
    }

    /// Execute a GraphQL query against the GitHub API
    ///
    /// # Arguments
    ///
    /// * `query` - GraphQL query string
    /// * `variables` - Query variables as JSON value
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Network request fails
    /// - Response parsing fails
    /// - GraphQL API returns errors
    ///
    /// # Example
    ///
    /// ```no_run
    /// use taskguard::github::client::GitHubClient;
    /// use serde_json::json;
    ///
    /// let client = GitHubClient::new()?;
    /// let query = r#"
    ///     query { viewer { login } }
    /// "#;
    /// let result = client.query(query, json!({}))?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn query(&self, query: &str, variables: Value) -> Result<Value> {
        let body = serde_json::json!({
            "query": query,
            "variables": variables,
        });

        let response = self
            .client
            .post(&self.api_url)
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&body)
            .send()
            .context("Failed to send GraphQL request")?;

        let json: Value = response
            .json()
            .context("Failed to parse GraphQL response")?;

        // Check for GraphQL errors
        if let Some(errors) = json.get("errors") {
            anyhow::bail!("GitHub API error: {}", errors);
        }

        Ok(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Requires gh CLI to be installed and authenticated
    fn test_client_creation() {
        let result = GitHubClient::new();
        // This test will pass if gh is installed and authenticated
        // Otherwise it should fail with a clear error message
        match result {
            Ok(_client) => {
                // Success - gh is installed and authenticated
            }
            Err(e) => {
                // Check that error message is helpful
                let error_msg = e.to_string();
                assert!(
                    error_msg.contains("gh auth login") || error_msg.contains("gh CLI"),
                    "Error message should guide user to authenticate: {}",
                    error_msg
                );
            }
        }
    }
}
