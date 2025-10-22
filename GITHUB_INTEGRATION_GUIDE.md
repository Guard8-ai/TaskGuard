# GitHub Projects Integration Implementation Guide

## Overview

This guide provides step-by-step instructions for implementing bidirectional synchronization between TaskGuard and GitHub Projects v2.

## Architecture

```
TaskGuard (Local)              GitHub GraphQL API           GitHub Projects
─────────────────              ──────────────────           ───────────────
tasks/backend-001.md    ←→     Mutations/Queries     ←→     Project Board
  - id: backend-001                                          - Issue #123
  - status: doing                                            - Column: In Progress
  - title: "Auth"                                            - Status: Doing
```

## Implementation Steps

### Step 1: Add Dependencies to Cargo.toml

```toml
[dependencies]
# Existing dependencies...
reqwest = { version = "0.11", features = ["json", "blocking"] }
serde_json = "1.0"
dotenv = "0.15"
```

### Step 2: Create GitHub Module Structure

Create the following file structure:

```
src/
├── github/
│   ├── mod.rs           # Module exports
│   ├── client.rs        # GraphQL client
│   ├── mutations.rs     # Create/update operations
│   ├── queries.rs       # Read operations
│   ├── types.rs         # GitHub-specific types
│   └── mapper.rs        # Task ↔ Issue mapping
└── commands/
    └── sync.rs          # Extend existing sync command
```

### Step 3: Implement GitHub Types (src/github/types.rs)

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubIssue {
    pub id: String,
    pub number: i64,
    pub title: String,
    pub state: String,
    pub body: Option<String>,
    pub labels: Vec<String>,
    pub assignees: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectItem {
    pub id: String,
    pub issue_id: String,
    pub project_id: String,
    pub status: String,
    pub field_values: Vec<FieldValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldValue {
    pub field_id: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct GitHubConfig {
    pub token: String,
    pub repository: String,  // "owner/repo"
    pub project_number: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskMapping {
    pub task_id: String,
    pub issue_number: i64,
    pub project_item_id: String,
    pub last_synced: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub enum SyncDirection {
    Push,     // TaskGuard → GitHub
    Pull,     // GitHub → TaskGuard
    BiDirectional,
}

#[derive(Debug)]
pub struct SyncConflict {
    pub task_id: String,
    pub local_status: String,
    pub remote_status: String,
}
```

### Step 4: Implement GraphQL Client (src/github/client.rs)

```rust
use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde_json::{json, Value};
use super::types::GitHubConfig;

pub struct GitHubClient {
    client: Client,
    config: GitHubConfig,
    api_url: String,
}

impl GitHubClient {
    pub fn new(config: GitHubConfig) -> Result<Self> {
        let client = Client::builder()
            .user_agent("TaskGuard/0.2.2")
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            config,
            api_url: "https://api.github.com/graphql".to_string(),
        })
    }

    pub fn execute_query(&self, query: &str, variables: Value) -> Result<Value> {
        let body = json!({
            "query": query,
            "variables": variables
        });

        let response = self.client
            .post(&self.api_url)
            .header("Authorization", format!("Bearer {}", self.config.token))
            .header("Accept", "application/vnd.github+json")
            .json(&body)
            .send()
            .context("Failed to send GraphQL request")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "GitHub API request failed with status: {}",
                response.status()
            ));
        }

        let result: Value = response.json()
            .context("Failed to parse GitHub API response")?;

        if let Some(errors) = result.get("errors") {
            return Err(anyhow::anyhow!(
                "GraphQL errors: {}",
                serde_json::to_string_pretty(errors)?
            ));
        }

        Ok(result)
    }

    pub fn get_repository(&self) -> (&str, &str) {
        let parts: Vec<&str> = self.config.repository.split('/').collect();
        (parts[0], parts[1])
    }
}
```

### Step 5: Implement GraphQL Queries (src/github/queries.rs)

```rust
use anyhow::{Context, Result};
use serde_json::{json, Value};
use super::client::GitHubClient;
use super::types::{GitHubIssue, ProjectItem};

pub struct GitHubQueries;

impl GitHubQueries {
    /// Get project ID from project number
    pub fn get_project_id(client: &GitHubClient) -> Result<String> {
        let (owner, repo) = client.get_repository();

        let query = r#"
            query($owner: String!, $repo: String!, $number: Int!) {
                repository(owner: $owner, name: $repo) {
                    projectV2(number: $number) {
                        id
                        title
                    }
                }
            }
        "#;

        let variables = json!({
            "owner": owner,
            "repo": repo,
            "number": client.config.project_number
        });

        let result = client.execute_query(query, variables)?;

        let project_id = result["data"]["repository"]["projectV2"]["id"]
            .as_str()
            .context("Project ID not found")?
            .to_string();

        Ok(project_id)
    }

    /// Get all project items (issues in the project)
    pub fn get_project_items(client: &GitHubClient, project_id: &str) -> Result<Vec<ProjectItem>> {
        let query = r#"
            query($projectId: ID!, $cursor: String) {
                node(id: $projectId) {
                    ... on ProjectV2 {
                        items(first: 100, after: $cursor) {
                            pageInfo {
                                hasNextPage
                                endCursor
                            }
                            nodes {
                                id
                                content {
                                    ... on Issue {
                                        id
                                        number
                                        title
                                        state
                                        body
                                    }
                                }
                                fieldValues(first: 10) {
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
                }
            }
        "#;

        let mut all_items = Vec::new();
        let mut cursor: Option<String> = None;
        let mut has_next_page = true;

        while has_next_page {
            let variables = json!({
                "projectId": project_id,
                "cursor": cursor
            });

            let result = client.execute_query(query, variables)?;

            let items = result["data"]["node"]["items"]["nodes"]
                .as_array()
                .context("Failed to parse items")?;

            // Parse items (simplified - you'll need to expand this)
            for item in items {
                // Extract and convert to ProjectItem
                // This is a simplified version - implement full parsing
            }

            let page_info = &result["data"]["node"]["items"]["pageInfo"];
            has_next_page = page_info["hasNextPage"].as_bool().unwrap_or(false);
            cursor = page_info["endCursor"].as_str().map(String::from);
        }

        Ok(all_items)
    }

    /// Get repository issues
    pub fn get_repository_issues(client: &GitHubClient) -> Result<Vec<GitHubIssue>> {
        let (owner, repo) = client.get_repository();

        let query = r#"
            query($owner: String!, $repo: String!, $cursor: String) {
                repository(owner: $owner, name: $repo) {
                    issues(first: 100, after: $cursor, states: [OPEN, CLOSED]) {
                        pageInfo {
                            hasNextPage
                            endCursor
                        }
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
                            assignees(first: 5) {
                                nodes {
                                    login
                                }
                            }
                        }
                    }
                }
            }
        "#;

        let mut all_issues = Vec::new();
        let mut cursor: Option<String> = None;
        let mut has_next_page = true;

        while has_next_page {
            let variables = json!({
                "owner": owner,
                "repo": repo,
                "cursor": cursor
            });

            let result = client.execute_query(query, variables)?;

            let issues = result["data"]["repository"]["issues"]["nodes"]
                .as_array()
                .context("Failed to parse issues")?;

            for issue in issues {
                let github_issue = GitHubIssue {
                    id: issue["id"].as_str().unwrap_or("").to_string(),
                    number: issue["number"].as_i64().unwrap_or(0),
                    title: issue["title"].as_str().unwrap_or("").to_string(),
                    state: issue["state"].as_str().unwrap_or("").to_string(),
                    body: issue["body"].as_str().map(String::from),
                    labels: issue["labels"]["nodes"]
                        .as_array()
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|l| l["name"].as_str().map(String::from))
                                .collect()
                        })
                        .unwrap_or_default(),
                    assignees: issue["assignees"]["nodes"]
                        .as_array()
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|a| a["login"].as_str().map(String::from))
                                .collect()
                        })
                        .unwrap_or_default(),
                };

                all_issues.push(github_issue);
            }

            let page_info = &result["data"]["repository"]["issues"]["pageInfo"];
            has_next_page = page_info["hasNextPage"].as_bool().unwrap_or(false);
            cursor = page_info["endCursor"].as_str().map(String::from);
        }

        Ok(all_issues)
    }
}
```

### Step 6: Implement GraphQL Mutations (src/github/mutations.rs)

```rust
use anyhow::{Context, Result};
use serde_json::json;
use super::client::GitHubClient;
use super::types::GitHubIssue;

pub struct GitHubMutations;

impl GitHubMutations {
    /// Create a new issue
    pub fn create_issue(
        client: &GitHubClient,
        title: &str,
        body: &str,
        labels: Vec<String>,
    ) -> Result<GitHubIssue> {
        let (owner, repo) = client.get_repository();

        // First, get repository ID
        let repo_query = r#"
            query($owner: String!, $repo: String!) {
                repository(owner: $owner, name: $repo) {
                    id
                }
            }
        "#;

        let repo_vars = json!({
            "owner": owner,
            "repo": repo
        });

        let repo_result = client.execute_query(repo_query, repo_vars)?;
        let repo_id = repo_result["data"]["repository"]["id"]
            .as_str()
            .context("Repository ID not found")?;

        // Create issue
        let mutation = r#"
            mutation($repositoryId: ID!, $title: String!, $body: String!) {
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
            "body": body
        });

        let result = client.execute_query(mutation, variables)?;

        let issue_data = &result["data"]["createIssue"]["issue"];
        let issue = GitHubIssue {
            id: issue_data["id"].as_str().unwrap_or("").to_string(),
            number: issue_data["number"].as_i64().unwrap_or(0),
            title: issue_data["title"].as_str().unwrap_or("").to_string(),
            state: issue_data["state"].as_str().unwrap_or("").to_string(),
            body: issue_data["body"].as_str().map(String::from),
            labels: Vec::new(),
            assignees: Vec::new(),
        };

        Ok(issue)
    }

    /// Update issue state (open/close)
    pub fn update_issue_state(
        client: &GitHubClient,
        issue_id: &str,
        state: &str,
    ) -> Result<()> {
        let mutation = match state.to_lowercase().as_str() {
            "closed" | "done" => r#"
                mutation($issueId: ID!) {
                    closeIssue(input: { issueId: $issueId }) {
                        issue {
                            id
                            state
                        }
                    }
                }
            "#,
            _ => r#"
                mutation($issueId: ID!) {
                    reopenIssue(input: { issueId: $issueId }) {
                        issue {
                            id
                            state
                        }
                    }
                }
            "#,
        };

        let variables = json!({
            "issueId": issue_id
        });

        client.execute_query(mutation, variables)?;
        Ok(())
    }

    /// Add issue to project
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
            "contentId": issue_id
        });

        let result = client.execute_query(mutation, variables)?;

        let item_id = result["data"]["addProjectV2ItemById"]["item"]["id"]
            .as_str()
            .context("Project item ID not found")?
            .to_string();

        Ok(item_id)
    }

    /// Update project item status field
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

        client.execute_query(mutation, variables)?;
        Ok(())
    }

    /// Get status field ID and options for a project
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

        let variables = json!({
            "projectId": project_id
        });

        let result = client.execute_query(query, variables)?;

        let fields = result["data"]["node"]["fields"]["nodes"]
            .as_array()
            .context("Failed to parse fields")?;

        // Find "Status" field
        for field in fields {
            if let Some(name) = field["name"].as_str() {
                if name.to_lowercase() == "status" {
                    let field_id = field["id"].as_str()
                        .context("Field ID not found")?
                        .to_string();

                    let options = field["options"]
                        .as_array()
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|opt| {
                                    Some((
                                        opt["id"].as_str()?.to_string(),
                                        opt["name"].as_str()?.to_string(),
                                    ))
                                })
                                .collect()
                        })
                        .unwrap_or_default();

                    return Ok((field_id, options));
                }
            }
        }

        Err(anyhow::anyhow!("Status field not found in project"))
    }
}
```

### Step 7: Implement Task-Issue Mapper (src/github/mapper.rs)

```rust
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::task::{Task, TaskStatus};
use super::types::{GitHubIssue, TaskMapping};

pub struct TaskIssueMapper {
    mapping_file: PathBuf,
    mappings: HashMap<String, TaskMapping>,
}

impl TaskIssueMapper {
    pub fn new(mapping_file: PathBuf) -> Result<Self> {
        let mappings = if mapping_file.exists() {
            let content = fs::read_to_string(&mapping_file)?;
            toml::from_str(&content).unwrap_or_default()
        } else {
            HashMap::new()
        };

        Ok(Self {
            mapping_file,
            mappings,
        })
    }

    pub fn add_mapping(&mut self, task_id: String, mapping: TaskMapping) {
        self.mappings.insert(task_id, mapping);
    }

    pub fn get_mapping(&self, task_id: &str) -> Option<&TaskMapping> {
        self.mappings.get(task_id)
    }

    pub fn get_task_id_by_issue(&self, issue_number: i64) -> Option<String> {
        self.mappings
            .iter()
            .find(|(_, m)| m.issue_number == issue_number)
            .map(|(id, _)| id.clone())
    }

    pub fn save(&self) -> Result<()> {
        let content = toml::to_string_pretty(&self.mappings)
            .context("Failed to serialize mappings")?;

        if let Some(parent) = self.mapping_file.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&self.mapping_file, content)?;
        Ok(())
    }

    /// Convert TaskGuard task to GitHub issue body
    pub fn task_to_issue_body(task: &Task) -> String {
        let mut body = String::new();

        body.push_str(&format!("**TaskGuard ID:** {}\n\n", task.id));
        body.push_str(&format!("**Area:** {}\n", task.area));
        body.push_str(&format!("**Priority:** {}\n", task.priority));

        if !task.dependencies.is_empty() {
            body.push_str(&format!("**Dependencies:** {}\n", task.dependencies.join(", ")));
        }

        body.push_str("\n---\n\n");
        body.push_str(&task.content);

        body
    }

    /// Map TaskGuard status to GitHub status
    pub fn taskguard_status_to_github(status: &TaskStatus) -> &'static str {
        match status {
            TaskStatus::Todo => "Todo",
            TaskStatus::Doing => "In Progress",
            TaskStatus::Review => "In Review",
            TaskStatus::Done => "Done",
            TaskStatus::Blocked => "Blocked",
        }
    }

    /// Map GitHub status to TaskGuard status
    pub fn github_status_to_taskguard(status: &str) -> TaskStatus {
        match status.to_lowercase().as_str() {
            "todo" | "backlog" | "to do" => TaskStatus::Todo,
            "in progress" | "doing" => TaskStatus::Doing,
            "in review" | "review" => TaskStatus::Review,
            "done" | "closed" => TaskStatus::Done,
            "blocked" => TaskStatus::Blocked,
            _ => TaskStatus::Todo,
        }
    }
}
```

### Step 8: Update Configuration (extend src/config.rs)

Add to `.taskguard/config.toml`:

```toml
[github]
enabled = false
token_env = "GITHUB_TOKEN"
repository = ""  # "owner/repo"
project_number = 0
sync_mode = "bidirectional"  # push, pull, bidirectional
auto_sync = false
status_mapping = { todo = "Todo", doing = "In Progress", review = "In Review", done = "Done", blocked = "Blocked" }
```

Update `src/config.rs`:

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubIntegration {
    pub enabled: bool,
    pub token_env: String,
    pub repository: String,
    pub project_number: i64,
    pub sync_mode: String,
    pub auto_sync: bool,
    pub status_mapping: HashMap<String, String>,
}

// Add to Config struct:
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    // ... existing fields ...
    #[serde(default)]
    pub github: Option<GitHubIntegration>,
}
```

### Step 9: Extend Sync Command (src/commands/sync.rs)

Add GitHub sync support to existing `sync.rs`:

```rust
// Add after existing imports
use crate::github::{GitHubClient, GitHubQueries, GitHubMutations, TaskIssueMapper};
use crate::github::types::{GitHubConfig, SyncDirection};

// Add new subcommand handler
pub fn run_github_sync(
    direction: SyncDirection,
    dry_run: bool,
) -> Result<()> {
    println!("🌐 GITHUB PROJECTS SYNC");

    // Load configuration
    let config = crate::config::load_config()?;
    let github_config = config.github
        .context("GitHub integration not configured. Run 'taskguard sync github --setup'")?;

    if !github_config.enabled {
        return Err(anyhow::anyhow!("GitHub integration is disabled in config"));
    }

    // Get GitHub token from environment
    let token = std::env::var(&github_config.token_env)
        .context("GitHub token not found in environment. Set GITHUB_TOKEN")?;

    let gh_config = GitHubConfig {
        token,
        repository: github_config.repository.clone(),
        project_number: github_config.project_number,
    };

    // Initialize GitHub client
    let client = GitHubClient::new(gh_config)?;

    // Load task-issue mappings
    let mapping_file = crate::config::get_taskguard_dir()?.join("state/github_mapping.toml");
    let mut mapper = TaskIssueMapper::new(mapping_file)?;

    // Load local tasks
    let tasks = load_all_tasks()?;

    match direction {
        SyncDirection::Push => {
            push_to_github(&client, &tasks, &mut mapper, dry_run)?;
        }
        SyncDirection::Pull => {
            pull_from_github(&client, &tasks, &mut mapper, dry_run)?;
        }
        SyncDirection::BiDirectional => {
            bidirectional_sync(&client, &tasks, &mut mapper, dry_run)?;
        }
    }

    // Save mappings
    if !dry_run {
        mapper.save()?;
    }

    Ok(())
}

fn push_to_github(
    client: &GitHubClient,
    tasks: &[Task],
    mapper: &mut TaskIssueMapper,
    dry_run: bool,
) -> Result<()> {
    println!("📤 Pushing tasks to GitHub...\n");

    let project_id = GitHubQueries::get_project_id(client)?;
    let (status_field_id, status_options) = GitHubMutations::get_status_field_info(client, &project_id)?;

    for task in tasks {
        // Check if task already has an issue
        if let Some(mapping) = mapper.get_mapping(&task.id) {
            println!("📝 Updating existing issue #{} for {}", mapping.issue_number, task.id);

            if !dry_run {
                // Update issue status
                // ... implementation
            }
        } else {
            println!("✨ Creating new issue for {}: {}", task.id, task.title);

            if !dry_run {
                // Create new issue
                let body = TaskIssueMapper::task_to_issue_body(task);
                let issue = GitHubMutations::create_issue(
                    client,
                    &task.title,
                    &body,
                    vec![format!("taskguard:{}", task.area)],
                )?;

                println!("   ✅ Created issue #{}", issue.number);

                // Add to project
                let item_id = GitHubMutations::add_issue_to_project(
                    client,
                    &project_id,
                    &issue.id,
                )?;

                println!("   ✅ Added to project board");

                // Save mapping
                mapper.add_mapping(task.id.clone(), TaskMapping {
                    task_id: task.id.clone(),
                    issue_number: issue.number,
                    project_item_id: item_id,
                    last_synced: chrono::Utc::now(),
                });
            }
        }
    }

    Ok(())
}

fn pull_from_github(
    client: &GitHubClient,
    tasks: &[Task],
    mapper: &mut TaskIssueMapper,
    dry_run: bool,
) -> Result<()> {
    println!("📥 Pulling updates from GitHub...\n");

    // Get all issues from repository
    let issues = GitHubQueries::get_repository_issues(client)?;

    for issue in issues {
        // Check if we have a task for this issue
        if let Some(task_id) = mapper.get_task_id_by_issue(issue.number) {
            println!("📝 Checking issue #{} -> {}", issue.number, task_id);

            // Find the task
            if let Some(task) = tasks.iter().find(|t| t.id == task_id) {
                // Compare status
                // ... implementation to update task file if status changed
            }
        } else {
            // New issue - potentially create task
            println!("🆕 New issue #{}: {}", issue.number, issue.title);

            if !dry_run {
                // Optionally create new TaskGuard task from issue
                // ... implementation
            }
        }
    }

    Ok(())
}

fn bidirectional_sync(
    client: &GitHubClient,
    tasks: &[Task],
    mapper: &mut TaskIssueMapper,
    dry_run: bool,
) -> Result<()> {
    println!("🔄 Bidirectional sync...\n");

    // Detect conflicts
    // ... implementation similar to existing remote sync

    Ok(())
}

fn load_all_tasks() -> Result<Vec<Task>> {
    use walkdir::WalkDir;

    let tasks_dir = crate::config::get_tasks_dir()?;
    let mut tasks = Vec::new();

    for entry in WalkDir::new(tasks_dir) {
        let entry = entry?;
        if entry.file_type().is_file() && entry.path().extension().is_some_and(|e| e == "md") {
            if let Ok(task) = Task::from_file(entry.path()) {
                tasks.push(task);
            }
        }
    }

    Ok(tasks)
}
```

### Step 10: Update Main CLI (src/main.rs)

Add GitHub sync subcommand:

```rust
#[derive(Subcommand)]
enum SyncCommands {
    /// Sync with GitHub Projects
    Github {
        /// Setup GitHub integration
        #[arg(long)]
        setup: bool,
        /// Push local tasks to GitHub
        #[arg(long)]
        push: bool,
        /// Pull GitHub updates to local
        #[arg(long)]
        pull: bool,
        /// Bidirectional sync
        #[arg(long)]
        bidirectional: bool,
        /// Dry run - show what would happen
        #[arg(long)]
        dry_run: bool,
    },
}

// Update Commands enum
#[derive(Subcommand)]
enum Commands {
    // ... existing commands ...

    /// Sync with Git or GitHub Projects
    Sync {
        #[command(subcommand)]
        command: Option<SyncCommands>,

        // Existing sync options for Git analysis
        #[arg(short, long, default_value = "50")]
        limit: usize,
        #[arg(short, long)]
        verbose: bool,
        #[arg(short, long)]
        remote: bool,
        #[arg(long)]
        dry_run: bool,
    },
}

// Update match in main()
Commands::Sync { command, limit, verbose, remote, dry_run } => {
    match command {
        Some(SyncCommands::Github { setup, push, pull, bidirectional, dry_run }) => {
            if setup {
                // Run setup wizard
                github_setup()?;
            } else {
                let direction = if push {
                    SyncDirection::Push
                } else if pull {
                    SyncDirection::Pull
                } else {
                    SyncDirection::BiDirectional
                };
                sync::run_github_sync(direction, dry_run)?;
            }
        }
        None => {
            // Existing Git sync
            sync::run(limit, verbose, remote, dry_run)?;
        }
    }
    Ok(())
}
```

### Step 11: Create Module Exports (src/github/mod.rs)

```rust
pub mod client;
pub mod queries;
pub mod mutations;
pub mod types;
pub mod mapper;

pub use client::GitHubClient;
pub use queries::GitHubQueries;
pub use mutations::GitHubMutations;
pub use mapper::TaskIssueMapper;
```

Update `src/lib.rs`:

```rust
pub mod github;
```

## Testing Plan

### Manual Testing Steps

1. **Setup Test**
   ```bash
   export GITHUB_TOKEN="your_github_token"
   taskguard sync github --setup
   ```

2. **Push Test**
   ```bash
   # Dry run first
   taskguard sync github --push --dry-run

   # Actual push
   taskguard sync github --push
   ```

3. **Pull Test**
   ```bash
   # Manually close an issue on GitHub
   taskguard sync github --pull
   # Verify task status updated locally
   ```

4. **Bidirectional Test**
   ```bash
   # Make changes both locally and on GitHub
   taskguard sync github --bidirectional
   # Should detect conflicts and prompt
   ```

### Unit Tests

Create `tests/github_integration_tests.rs`:

```rust
#[cfg(test)]
mod tests {
    use taskguard::github::mapper::TaskIssueMapper;
    use taskguard::task::TaskStatus;

    #[test]
    fn test_status_mapping() {
        assert_eq!(
            TaskIssueMapper::taskguard_status_to_github(&TaskStatus::Todo),
            "Todo"
        );

        assert_eq!(
            TaskIssueMapper::github_status_to_taskguard("In Progress"),
            TaskStatus::Doing
        );
    }

    #[test]
    fn test_task_to_issue_body() {
        // Create a test task
        // Verify body format
    }
}
```

## Configuration Example

After setup, `.taskguard/config.toml` should look like:

```toml
[project]
name = "MyProject"
version = "0.2.2"

[github]
enabled = true
token_env = "GITHUB_TOKEN"
repository = "owner/repo"
project_number = 1
sync_mode = "bidirectional"
auto_sync = false
status_mapping = { todo = "Todo", doing = "In Progress", review = "In Review", done = "Done", blocked = "Blocked" }
```

## Security Considerations

1. **Never commit GitHub token** to version control
2. Use environment variable `GITHUB_TOKEN`
3. Add to `.gitignore`:
   ```
   .taskguard/state/github_mapping.toml
   .env
   ```

4. Use minimal token scopes: `repo`, `project`

## Usage Examples

### Initial Setup
```bash
export GITHUB_TOKEN="ghp_xxxxxxxxxxxxx"
cd my-project
taskguard sync github --setup
# Follow prompts to configure repository and project
```

### Push Local Tasks to GitHub
```bash
taskguard sync github --push
```

### Pull GitHub Updates
```bash
taskguard sync github --pull
```

### Bidirectional Sync
```bash
taskguard sync github --bidirectional
```

### Check What Would Happen (Dry Run)
```bash
taskguard sync github --bidirectional --dry-run
```

## Troubleshooting

### "GitHub token not found"
```bash
export GITHUB_TOKEN="your_token_here"
```

### "Project not found"
Verify project number in config:
```bash
# Projects are numbered, find yours on GitHub
# https://github.com/users/YOU/projects/NUMBER
```

### "GraphQL errors"
Enable verbose logging to see full error:
```bash
RUST_LOG=debug taskguard sync github --push
```

## Next Steps

After implementing basic sync:

1. **Webhooks**: Listen for GitHub events for real-time sync
2. **Labels sync**: Map TaskGuard tags to GitHub labels
3. **Assignee sync**: Map TaskGuard assignee to GitHub assignees
4. **Dependencies**: Create issue dependencies from TaskGuard dependencies
5. **Attachments**: Sync task checklist items to issue tasks

## Implementation Checklist

- [ ] Add dependencies to Cargo.toml
- [ ] Create github module structure
- [ ] Implement types.rs
- [ ] Implement client.rs
- [ ] Implement queries.rs
- [ ] Implement mutations.rs
- [ ] Implement mapper.rs
- [ ] Update config.rs
- [ ] Extend sync.rs
- [ ] Update main.rs CLI
- [ ] Add module exports
- [ ] Create tests
- [ ] Update documentation
- [ ] Test with real GitHub repository

## Estimated Implementation Time

- Core GitHub client: 2-3 hours
- Queries/Mutations: 3-4 hours
- Mapper and sync logic: 3-4 hours
- CLI integration: 1-2 hours
- Testing and debugging: 2-3 hours
- **Total: 11-16 hours**

## Support

For questions or issues during implementation:
1. Check GitHub GraphQL API docs: https://docs.github.com/en/graphql
2. Test queries in GitHub GraphQL Explorer: https://docs.github.com/en/graphql/overview/explorer
3. Review TaskGuard existing sync implementation in `src/commands/sync.rs`
