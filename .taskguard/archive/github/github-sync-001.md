---
id: github-sync-001
title: Add --github flag to sync command for bidirectional GitHub sync
status: done
priority: critical
tags:
- github
- sync
- dashboard
dependencies:
- github-infra-005
assignee: developer
created: 2025-10-30T22:30:00Z
estimate: 4h
complexity: 8
area: github
---

# Add --github Flag to Sync Command for Bidirectional GitHub Sync

## Context
**END GOAL:** Enable users to see TaskGuard tasks in their GitHub dashboard by syncing with GitHub Issues/Projects.

When users run `taskguard sync --github`, the command should:
1. **Push local tasks to GitHub** - Create/update issues for tasks
2. **Pull GitHub changes back** - Update local tasks based on issue changes
3. **Show sync status** - Display what was created/updated/skipped
4. **Handle conflicts** - Resolve differences between local and remote state

## Dependencies
**Requires:** github-infra-005 (Complete GitHub API: client, mapper, mutations, queries)
**Why:** Needs full read-write API to create issues, update states, and query existing issues

## Objectives
1. Add `--github` flag to sync command CLI
2. Implement push workflow (local tasks → GitHub issues)
3. Implement pull workflow (GitHub issues → local tasks)
4. Handle conflict resolution
5. Show comprehensive sync report

## Implementation Plan

### 1. Update CLI Arguments (src/main.rs)

```rust
#[derive(Parser)]
pub struct SyncArgs {
    /// Number of commits to analyze
    #[arg(short, long, default_value = "50")]
    limit: usize,

    /// Show detailed analysis
    #[arg(short, long)]
    verbose: bool,

    /// Fetch and analyze remote repository changes
    #[arg(short, long)]
    remote: bool,

    /// Sync with GitHub Issues (requires GitHub integration)
    #[arg(long)]
    github: bool,

    /// Dry run mode
    #[arg(long)]
    dry_run: bool,
}
```

### 2. Add GitHub Sync Function (src/commands/sync.rs)

```rust
use crate::github::{
    GitHubClient, GitHubMutations, GitHubQueries,
    TaskIssueMapper, IssueMapping, load_github_config
};

fn run_github_sync(
    tasks: &[Task],
    dry_run: bool,
) -> Result<()> {
    println!("🌐 GITHUB SYNC MODE");
    println!("   Syncing local tasks with GitHub Issues...\n");

    // Check if GitHub is configured
    if !crate::github::is_github_sync_enabled()? {
        println!("❌ GitHub sync not configured");
        println!("   Create .taskguard/github.toml with:");
        println!("   token = \"your-github-token\"");
        println!("   owner = \"your-username\"");
        println!("   repo = \"your-repo\"");
        return Ok(());
    }

    // Load configuration
    let config = load_github_config()?;
    let client = GitHubClient::new(config.token)?;
    let mut mapper = TaskIssueMapper::load()?;

    println!("📤 PUSH: Local Tasks → GitHub Issues");
    push_tasks_to_github(&client, &config, tasks, &mut mapper, dry_run)?;

    println!();
    println!("📥 PULL: GitHub Issues → Local Tasks");
    pull_issues_from_github(&client, &config, tasks, &mapper, dry_run)?;

    // Save updated mapping
    if !dry_run {
        mapper.save()?;
        println!();
        println!("✅ Sync mapping saved");
    }

    Ok(())
}

fn push_tasks_to_github(
    client: &GitHubClient,
    config: &GitHubConfig,
    tasks: &[Task],
    mapper: &mut TaskIssueMapper,
    dry_run: bool,
) -> Result<()> {
    let mut created = 0;
    let mut updated = 0;
    let mut skipped = 0;

    for task in tasks {
        // Skip archived tasks (they should already be closed)
        if task.path.contains("archive") {
            continue;
        }

        // Check if task already has a GitHub issue
        if let Some(mapping) = mapper.get_mapping(&task.id) {
            // Task has issue - check if update needed
            let issue = GitHubQueries::get_issue_by_id(client, &mapping.issue_id)?;

            // Compare states
            let github_state = map_github_state_to_taskguard(&issue.state);
            if task.status.to_string() != github_state {
                println!("   🔄 {} - {} (status mismatch)", task.id, task.title);
                println!("      Local: {:?}, GitHub: {}", task.status, issue.state);

                if !dry_run {
                    // Update GitHub to match local
                    let new_state = map_taskguard_status_to_github(&task.status);
                    GitHubMutations::update_issue_state(client, &issue.id, new_state)?;
                    updated += 1;
                    println!("      ✅ Updated GitHub issue to {}", new_state);
                } else {
                    println!("      Would update GitHub issue to {:?}", task.status);
                }
            } else {
                skipped += 1;
            }
        } else {
            // No issue exists - create one
            println!("   ➕ {} - {} (creating issue)", task.id, task.title);

            if !dry_run {
                let body = format!(
                    "**TaskGuard ID:** {}\n\n## Description\n\n{}\n\n---\n*Synced from TaskGuard*",
                    task.id,
                    task.description.as_deref().unwrap_or("No description")
                );

                let issue = GitHubMutations::create_issue(
                    client,
                    &config.owner,
                    &config.repo,
                    &task.title,
                    Some(&body),
                )?;

                // Save mapping
                let mapping = IssueMapping {
                    task_id: task.id.clone(),
                    issue_number: issue.number,
                    issue_id: issue.id,
                    project_item_id: None,
                    synced_at: chrono::Utc::now().to_rfc3339(),
                    is_archived: false,
                };
                mapper.set_mapping(mapping);

                created += 1;
                println!("      ✅ Created issue #{}", issue.number);
            } else {
                println!("      Would create GitHub issue");
            }
        }
    }

    println!();
    println!("📊 PUSH SUMMARY");
    println!("   Created: {}", created);
    println!("   Updated: {}", updated);
    println!("   Skipped: {} (already in sync)", skipped);

    Ok(())
}

fn pull_issues_from_github(
    client: &GitHubClient,
    config: &GitHubConfig,
    tasks: &[Task],
    mapper: &TaskIssueMapper,
    dry_run: bool,
) -> Result<()> {
    let issues = GitHubQueries::get_repository_issues(
        client,
        &config.owner,
        &config.repo,
        Some(100),
    )?;

    let mut mapped_count = 0;
    let mut orphaned_issues = Vec::new();
    let mut updates_needed = Vec::new();

    for issue in issues {
        // Check if this issue is tracked
        if let Some(task_id) = mapper.get_task_id_by_issue(issue.number) {
            mapped_count += 1;

            // Find the task
            if let Some(task) = tasks.iter().find(|t| t.id == task_id) {
                let github_state = map_github_state_to_taskguard(&issue.state);
                let local_state = task.status.to_string();

                if github_state != local_state {
                    updates_needed.push((task.id.clone(), local_state, github_state));
                }
            }
        } else {
            // Orphaned issue - no TaskGuard task
            orphaned_issues.push(issue);
        }
    }

    println!("   ✅ {} issues mapped to existing tasks", mapped_count);

    // Report orphaned issues
    if !orphaned_issues.is_empty() {
        println!();
        println!("   ⚠️  {} ORPHANED ISSUES (no matching TaskGuard task):", orphaned_issues.len());
        for issue in &orphaned_issues {
            println!("      #{} - \"{}\"", issue.number, issue.title);
        }

        println!();
        println!("   💡 SUGGESTED ACTIONS:");
        println!("      1. Create tasks manually for these issues");
        println!("      2. Or ignore them (they'll stay on GitHub only)");
        println!("      3. Or ask AI: \"Create TaskGuard tasks for orphaned GitHub issues\"");
    }

    // Report status mismatches
    if !updates_needed.is_empty() {
        println!();
        println!("   ⚠️  {} tasks have status changes on GitHub:", updates_needed.len());
        for (task_id, local, github) in &updates_needed {
            println!("      {} - Local: {}, GitHub: {}", task_id, local, github);
        }

        if !dry_run {
            println!();
            println!("   💡 TIP: Update local task files to match GitHub state");
            println!("      Or next sync will push local status back to GitHub");
        }
    }

    if orphaned_issues.is_empty() && updates_needed.is_empty() {
        println!("   ✅ All tasks in sync with GitHub");
    }

    Ok(())
}

// Helper functions
fn map_taskguard_status_to_github(status: &TaskStatus) -> &str {
    match status {
        TaskStatus::Done => "CLOSED",
        _ => "OPEN",
    }
}

fn map_github_state_to_taskguard(state: &str) -> &str {
    match state.to_uppercase().as_str() {
        "CLOSED" => "done",
        _ => "todo",
    }
}
```

### 3. Wire Up in Main (src/commands/sync.rs)

```rust
pub fn run(
    limit: usize,
    verbose: bool,
    remote: bool,
    github: bool,
    dry_run: bool,
) -> Result<()> {
    // Load all tasks first
    let current_tasks = load_all_tasks()
        .context("Failed to load tasks")?;

    // GitHub sync mode
    if github {
        return run_github_sync(&current_tasks, dry_run);
    }

    // Remote git sync mode
    if remote {
        let current_dir = env::current_dir()?;
        let git_analyzer = GitAnalyzer::new(&current_dir)?;
        return run_remote_sync(&git_analyzer, &current_tasks, limit, verbose, dry_run);
    }

    // Local git analysis mode (existing)
    let current_dir = env::current_dir()?;
    let git_analyzer = GitAnalyzer::new(&current_dir)?;
    // ... existing local sync logic ...
}
```

## Acceptance Criteria

✅ **CLI Flag:**
- `taskguard sync --github` flag recognized
- Help text explains GitHub sync functionality
- Works with `--dry-run` for testing

✅ **Push Workflow:**
- Creates GitHub issues for tasks without issues
- Updates GitHub issue state to match local task status
- Skips tasks already in sync
- Shows summary of created/updated/skipped

✅ **Pull Workflow:**
- Detects when GitHub issues have different states
- Reports discrepancies to user
- Suggests updating local files (future: auto-update)

✅ **Conflict Resolution:**
- Push takes priority (local is source of truth)
- Pull shows warnings for manual resolution
- Dry run mode shows what would change

✅ **Mapping:**
- Task-issue mappings saved in `.taskguard/github-mapping.json`
- Mappings persist across sync runs
- Can lookup task by issue number and vice versa

✅ **Error Handling:**
- Graceful handling when GitHub not configured
- Network errors reported clearly
- Rate limiting handled appropriately

## Testing

```bash
# 1. Configure GitHub integration
cat > .taskguard/github.toml <<EOF
token = "github_pat_..."
owner = "your-username"
repo = "your-repo"
EOF

# 2. Dry run to see what would happen
taskguard sync --github --dry-run

# 3. Actual sync
taskguard sync --github

# 4. Check GitHub dashboard
# - Should see issues created for each task
# - Issue states should match task statuses

# 5. Change issue on GitHub
# - Close an issue on GitHub web UI

# 6. Pull changes
taskguard sync --github
# - Should report the discrepancy
# - Suggests updating local task
```

## Technical Notes

- **Authentication:** GitHub Personal Access Token with `repo` scope
- **Rate Limits:** GraphQL API has 5000 points/hour
- **Issue Body:** Include TaskGuard ID for mapping
- **State Mapping:** done=CLOSED, everything else=OPEN
- **Archived Tasks:** Skip during push (already closed)
- **Future:** Support GitHub Projects v2 for column tracking

## Success Metrics

**Primary Goal:** Users can run `taskguard sync --github` and see their tasks appear in the GitHub Issues dashboard

**Secondary Goals:**
- Bidirectional sync keeps local and GitHub in sync
- Status changes propagate in both directions
- Clear feedback about what was synced

## Session Handoff Template

### What Changed
- Added `--github` flag to sync command
- Implemented push_tasks_to_github() for local → GitHub
- Implemented pull_issues_from_github() for GitHub → local
- Integrated with GitHub API modules (client, mutations, queries, mapper)

### Causality Impact
- **Sync Command → GitHub API**: Sync triggers API calls to create/update issues
- **Task Creation → Issue Creation**: New tasks automatically get GitHub issues
- **Status Change → Issue Update**: Local status changes sync to GitHub
- **Issue State Change → Warning**: GitHub changes reported to user

### Runtime Behavior
- First run creates issues for all tasks (slow, API intensive)
- Subsequent runs only update changed tasks (fast)
- Dry run mode shows plan without making changes
- Mapping file tracks all task-issue relationships

### Dependencies Unblocked
- Users can now see tasks in GitHub dashboard ✅
- GitHub Projects integration can build on this
- Future: Automatic local updates from GitHub
- Future: Label/milestone sync

### Next Steps After This Task
1. Test with real GitHub repository
2. Add label sync (priority → labels)
3. Add GitHub Projects v2 integration (columns)
4. Add automatic local task updates from GitHub
5. Add webhook support for real-time sync