/// Integration tests for GitHub module foundation
/// Tests verify acceptance criteria from github-infra-002

use taskguard::github::{
    GitHubClient, GitHubConfig, GitHubIssue, ProjectItem,
    FieldValue, ProjectV2StatusUpdate, TaskMapping,
    is_github_sync_enabled, load_github_config, get_github_config_path,
};

#[test]
fn test_github_types_can_be_constructed() {
    // Verify all GitHub types can be created

    let issue = GitHubIssue {
        id: "node_123".to_string(),
        number: 42,
        title: "Test Issue".to_string(),
        state: "OPEN".to_string(),
        body: Some("Test body".to_string()),
        labels: vec!["bug".to_string(), "priority:high".to_string()],
        assignees: vec!["developer".to_string()],
    };
    assert_eq!(issue.number, 42);
    assert_eq!(issue.labels.len(), 2);

    let field_value = FieldValue {
        field_id: "field_123".to_string(),
        value: "In Progress".to_string(),
    };
    assert_eq!(field_value.field_id, "field_123");

    let project_item = ProjectItem {
        id: "item_123".to_string(),
        issue_id: "node_123".to_string(),
        project_id: "project_123".to_string(),
        status: "In Progress".to_string(),
        field_values: vec![field_value],
    };
    assert_eq!(project_item.field_values.len(), 1);

    let status_update = ProjectV2StatusUpdate {
        id: "update_123".to_string(),
        status: "In Progress".to_string(),
        body: Some("Working on implementation".to_string()),
        created_at: "2025-01-15T10:00:00Z".to_string(),
    };
    assert!(status_update.body.is_some());

    let mapping = TaskMapping {
        task_id: "backend-001".to_string(),
        issue_number: 42,
        project_item_id: "item_123".to_string(),
        last_synced: "2025-01-15T10:00:00Z".to_string(),
    };
    assert_eq!(mapping.task_id, "backend-001");
    assert_eq!(mapping.project_item_id, "item_123");
}

#[test]
fn test_github_config_structure() {
    // Verify GitHubConfig has correct fields and NO token field
    let config = GitHubConfig {
        owner: "Eliran79".to_string(),
        repo: "TaskGuard".to_string(),
        project_number: 1,
    };

    assert_eq!(config.owner, "Eliran79");
    assert_eq!(config.repo, "TaskGuard");
    assert_eq!(config.project_number, 1);

    // Verify serialization/deserialization works
    let toml_str = toml::to_string(&config).unwrap();
    assert!(toml_str.contains("owner"));
    assert!(toml_str.contains("repo"));
    assert!(toml_str.contains("project_number"));
    assert!(!toml_str.contains("token"), "Config should not contain token field");

    let parsed: GitHubConfig = toml::from_str(&toml_str).unwrap();
    assert_eq!(parsed.owner, config.owner);
    assert_eq!(parsed.repo, config.repo);
    assert_eq!(parsed.project_number, config.project_number);
}

#[test]
fn test_serde_serialization() {
    // Verify all types support Serde serialization
    use serde_json;

    let issue = GitHubIssue {
        id: "node_123".to_string(),
        number: 42,
        title: "Test".to_string(),
        state: "OPEN".to_string(),
        body: None,
        labels: vec![],
        assignees: vec![],
    };

    let json = serde_json::to_string(&issue).unwrap();
    assert!(json.contains("node_123"));
    assert!(json.contains("\"number\":42"));

    let deserialized: GitHubIssue = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.id, issue.id);
    assert_eq!(deserialized.number, issue.number);
}

#[test]
#[ignore] // Requires being in a TaskGuard project
fn test_github_config_functions() {
    // These functions require being in a TaskGuard project
    // Run with: cargo test --test github_module_test test_github_config_functions -- --ignored

    // Test config path function
    let _ = get_github_config_path();

    // Test sync enabled check
    let _ = is_github_sync_enabled();
}

#[test]
#[ignore] // Requires gh CLI to be installed and authenticated
fn test_github_client_creation() {
    // This test requires gh CLI to be installed and authenticated
    // Run with: cargo test --test github_module_test test_github_client_creation -- --ignored

    match GitHubClient::new() {
        Ok(client) => {
            // If successful, verify we can create queries
            println!("✅ GitHub client created successfully");

            // Note: We can't actually run queries without a real repo
            // but we can verify the client exists
            let _ = client;
        }
        Err(e) => {
            // If failed, verify error message is helpful
            let msg = e.to_string();
            assert!(
                msg.contains("gh auth login") || msg.contains("gh CLI"),
                "Error should mention gh CLI or auth: {}",
                msg
            );
            println!("ℹ️  Client creation failed (expected if gh not installed): {}", msg);
        }
    }
}

#[test]
fn test_module_exports() {
    // Verify all expected types are exported from the module
    // This test ensures the module API is stable

    // These should all compile - if they don't, exports are broken
    let _: Option<GitHubClient> = None;
    let _: Option<GitHubConfig> = None;
    let _: Option<GitHubIssue> = None;
    let _: Option<ProjectItem> = None;
    let _: Option<FieldValue> = None;
    let _: Option<ProjectV2StatusUpdate> = None;
    let _: Option<TaskMapping> = None;

    // Function exports
    let _ = is_github_sync_enabled;
    let _ = load_github_config;
    let _ = get_github_config_path;
}

#[test]
fn test_projects_v2_types_are_primary() {
    // Verify Projects v2 types have all required fields

    let project_item = ProjectItem {
        id: "item_1".to_string(),
        issue_id: "issue_1".to_string(),
        project_id: "project_1".to_string(),
        status: "Todo".to_string(),
        field_values: vec![],
    };

    // ProjectItem must track both issue_id AND project_item_id
    assert!(!project_item.id.is_empty());
    assert!(!project_item.issue_id.is_empty());
    assert!(!project_item.project_id.is_empty());

    let mapping = TaskMapping {
        task_id: "task-001".to_string(),
        issue_number: 1,
        project_item_id: "item_1".to_string(),
        last_synced: "2025-01-15T10:00:00Z".to_string(),
    };

    // TaskMapping must track project_item_id for Projects v2 board sync
    assert!(!mapping.project_item_id.is_empty());
}
