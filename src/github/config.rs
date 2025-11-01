use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use crate::config::find_taskguard_root;
use super::types::GitHubConfig;

/// Check if GitHub sync is enabled for the current project
///
/// Returns `true` if `.taskguard/github.toml` exists, `false` otherwise.
///
/// # Errors
///
/// Returns an error if not in a TaskGuard project directory.
///
/// # Example
///
/// ```no_run
/// use taskguard::github::config::is_github_sync_enabled;
///
/// if is_github_sync_enabled()? {
///     println!("GitHub sync is enabled");
/// }
/// ```
pub fn is_github_sync_enabled() -> Result<bool> {
    let root = find_taskguard_root()
        .context("Not in a TaskGuard project")?;
    let config_path = root.join(".taskguard/github.toml");
    Ok(config_path.exists())
}

/// Load GitHub configuration from `.taskguard/github.toml`
///
/// # Configuration Format
///
/// The configuration file should be in TOML format with the following fields:
///
/// ```toml
/// owner = "Eliran79"
/// repo = "TaskGuard"
/// project_number = 1
/// ```
///
/// **Note**: No token is stored in the configuration file.
/// Authentication is handled via `gh` CLI.
///
/// # Errors
///
/// Returns an error if:
/// - Not in a TaskGuard project directory
/// - GitHub config file doesn't exist
/// - Config file is invalid TOML
/// - Required fields are missing
///
/// # Example
///
/// ```no_run
/// use taskguard::github::config::load_github_config;
///
/// let config = load_github_config()?;
/// println!("Repository: {}/{}", config.owner, config.repo);
/// println!("Project number: {}", config.project_number);
/// ```
pub fn load_github_config() -> Result<GitHubConfig> {
    let root = find_taskguard_root()
        .context("Not in a TaskGuard project")?;
    let config_path = root.join(".taskguard/github.toml");

    if !config_path.exists() {
        anyhow::bail!(
            "GitHub configuration not found.\n\n\
            Create `.taskguard/github.toml` with:\n\n\
            owner = \"your-username\"\n\
            repo = \"your-repo\"\n\
            project_number = 1\n"
        );
    }

    let content = fs::read_to_string(&config_path)
        .context("Failed to read GitHub config")?;

    let config: GitHubConfig = toml::from_str(&content)
        .context("Failed to parse GitHub config")?;

    Ok(config)
}

/// Get the path to the GitHub configuration file
///
/// # Errors
///
/// Returns an error if not in a TaskGuard project directory.
pub fn get_github_config_path() -> Result<PathBuf> {
    let root = find_taskguard_root()
        .context("Not in a TaskGuard project")?;
    Ok(root.join(".taskguard/github.toml"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_path() {
        // This test just ensures the function doesn't panic
        // Actual functionality requires being in a TaskGuard project
        let _ = get_github_config_path();
    }
}
