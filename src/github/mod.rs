//! GitHub integration module for TaskGuard
//!
//! This module provides GitHub Projects v2 integration with focus on:
//! - GraphQL API client with `gh` CLI authentication
//! - Type-safe representations of GitHub entities (Issues, Projects v2)
//! - Configuration management (no token storage - uses `gh` CLI)
//!
//! # Authentication
//!
//! TaskGuard uses the `gh` CLI for authentication. Users must:
//! 1. Install the GitHub CLI: https://cli.github.com/
//! 2. Authenticate: `gh auth login`
//! 3. Ensure proper scopes: `gh auth refresh -s project`
//!
//! # Configuration
//!
//! Create `.taskguard/github.toml`:
//!
//! ```toml
//! owner = "your-username"
//! repo = "your-repo"
//! project_number = 1
//! ```
//!
//! # Example Usage
//!
//! ```no_run
//! use taskguard::github::{client::GitHubClient, config::load_github_config};
//!
//! // Create authenticated client
//! let client = GitHubClient::new()?;
//!
//! // Load project configuration
//! let config = load_github_config()?;
//!
//! // Execute GraphQL queries
//! let query = r#"query { viewer { login } }"#;
//! let result = client.query(query, serde_json::json!({}))?;
//! # Ok::<(), anyhow::Error>(())
//! ```

pub mod client;
pub mod config;
pub mod mapper;
pub mod types;

// Re-export commonly used items
pub use client::GitHubClient;
pub use config::{is_github_sync_enabled, load_github_config, get_github_config_path};
pub use mapper::{IssueMapping, TaskIssueMapper};
pub use types::{
    GitHubConfig, GitHubIssue, ProjectItem, FieldValue,
    ProjectV2StatusUpdate, TaskMapping,
};
