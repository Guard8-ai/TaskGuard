/// Integration tests for GitHub mutations with real GitHub Projects v2
///
/// These tests require:
/// - GitHub CLI authenticated: gh auth login
/// - Project scope enabled: gh auth refresh -s project
/// - Configuration file: .taskguard/github.toml
///
/// Run with: cargo test --test github_integration_test -- --ignored --nocapture

use taskguard::github::{
    GitHubClient, GitHubMutations, TaskIssueMapper,
    config::load_github_config,
};
use taskguard::task::TaskStatus;

#[test]
#[ignore] // Run manually with: cargo test test_create_issue_real -- --ignored --nocapture
fn test_create_issue_real() {
    println!("\n🧪 Testing issue creation with real GitHub API...\n");

    // Load configuration
    let config = match load_github_config() {
        Ok(c) => {
            println!("✓ Loaded config: {}/{}", c.owner, c.repo);
            c
        }
        Err(e) => {
            eprintln!("❌ Failed to load config: {}", e);
            eprintln!("   Make sure .taskguard/github.toml exists");
            panic!("Config load failed");
        }
    };

    // Create authenticated client
    let client = match GitHubClient::new() {
        Ok(c) => {
            println!("✓ Authenticated with GitHub");
            c
        }
        Err(e) => {
            eprintln!("❌ Failed to authenticate: {}", e);
            eprintln!("   Run: gh auth login");
            panic!("Authentication failed");
        }
    };

    // Create test issue
    println!("\n📝 Creating test issue...");
    let issue = match GitHubMutations::create_issue(
        &client,
        &config.owner,
        &config.repo,
        "[TEST] TaskGuard Integration Test",
        Some("This is a test issue created by TaskGuard's integration tests.\n\nYou can safely close this issue."),
    ) {
        Ok(issue) => {
            println!("✓ Created issue #{}: {}", issue.number, issue.title);
            println!("  ID: {}", issue.id);
            println!("  State: {}", issue.state);
            println!("  URL: https://github.com/{}/{}/issues/{}",
                     config.owner, config.repo, issue.number);
            issue
        }
        Err(e) => {
            eprintln!("❌ Failed to create issue: {}", e);
            panic!("Issue creation failed");
        }
    };

    println!("\n✅ Test passed! Check your GitHub repository to see the issue.");
    println!("   https://github.com/{}/{}/issues/{}",
             config.owner, config.repo, issue.number);
}

#[test]
#[ignore] // Run manually: cargo test test_full_project_sync -- --ignored --nocapture
fn test_full_project_sync() {
    println!("\n🧪 Testing full Projects v2 sync workflow...\n");

    // Load configuration
    let config = load_github_config()
        .expect("Failed to load config - create .taskguard/github.toml");
    println!("✓ Config: {}/{} - Project #{}",
             config.owner, config.repo, config.project_number);

    // Authenticate
    let client = GitHubClient::new()
        .expect("Failed to authenticate - run: gh auth login");
    println!("✓ Authenticated with GitHub");

    // Step 1: Create issue
    println!("\n📝 Step 1: Creating issue...");
    let issue = GitHubMutations::create_issue(
        &client,
        &config.owner,
        &config.repo,
        "[TEST] Full Projects v2 Sync",
        Some("Testing the complete TaskGuard → GitHub Projects workflow"),
    ).expect("Failed to create issue");
    println!("✓ Issue #{} created: {}", issue.number, issue.id);

    // Step 2: Get project ID
    println!("\n🔍 Step 2: Getting project ID...");
    let project_id = get_project_id(&client, &config.owner, config.project_number)
        .expect("Failed to get project ID");
    println!("✓ Project ID: {}", project_id);

    // Step 3: Add issue to project
    println!("\n➕ Step 3: Adding issue to project...");
    let item_id = GitHubMutations::add_issue_to_project(
        &client,
        &project_id,
        &issue.id,
    ).expect("Failed to add issue to project");
    println!("✓ Added to project - Item ID: {}", item_id);

    // Step 4: Get status field info
    println!("\n📊 Step 4: Getting status field information...");
    let (field_id, options) = GitHubMutations::get_status_field_info(
        &client,
        &project_id,
    ).expect("Failed to get status field info");
    println!("✓ Status field ID: {}", field_id);
    println!("  Available options:");
    for (opt_id, opt_name) in &options {
        println!("    - {} ({})", opt_name, opt_id);
    }

    // Step 5: Find best status option for "Doing"
    println!("\n🎯 Step 5: Finding best status option for 'Doing'...");
    let option_id = TaskIssueMapper::find_best_status_option(
        &TaskStatus::Doing,
        &options,
    ).expect("No matching status column found");
    println!("✓ Selected option ID: {}", option_id);

    // Step 6: Update project item status
    println!("\n✏️  Step 6: Updating project item status...");
    GitHubMutations::update_project_item_status(
        &client,
        &project_id,
        &item_id,
        &field_id,
        &option_id,
    ).expect("Failed to update status");
    println!("✓ Status updated successfully");

    // Summary
    println!("\n✅ Full workflow completed successfully!");
    println!("\n🎉 Check your GitHub Projects dashboard:");
    println!("   https://github.com/orgs/{}/projects/{}",
             config.owner, config.project_number);
    println!("\n   Your test issue should appear in the 'In Progress' column!");
}

/// Helper function to get project ID from organization and number
fn get_project_id(client: &GitHubClient, owner: &str, project_number: i64) -> Result<String, anyhow::Error> {
    let query = r#"
        query($owner: String!, $number: Int!) {
            organization(login: $owner) {
                projectV2(number: $number) {
                    id
                }
            }
        }
    "#;

    let variables = serde_json::json!({
        "owner": owner,
        "number": project_number,
    });

    let response = client.query(query, variables)?;

    let project_id = response["data"]["organization"]["projectV2"]["id"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Project not found"))?
        .to_string();

    Ok(project_id)
}
