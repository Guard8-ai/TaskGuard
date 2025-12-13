use crate::config::load_all_tasks;
use crate::git::{ConflictResolution, GitAnalyzer};
use crate::task::{Task, TaskStatus};
use anyhow::{Context, Result};
use std::env;
use std::io::{self, Write};

use crate::github::{
    GitHubClient, GitHubConfig, GitHubMutations, GitHubProjectSetup, GitHubQueries, IssueMapping,
    TaskIssueMapper, is_github_sync_enabled, load_github_config,
};

/// Get the current git branch name
fn get_current_branch() -> Option<String> {
    std::process::Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout)
                    .ok()
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty() && s != "HEAD")
            } else {
                None
            }
        })
}

/// Generate a short hash of task file content for duplicate detection
fn hash_task_content(task: &Task) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    task.id.hash(&mut hasher);
    task.title.hash(&mut hasher);
    task.content.hash(&mut hasher);
    let hash = hasher.finish();
    format!("{:x}", hash)[..8].to_string()
}

/// Search GitHub issues for existing TaskGuard ID
#[allow(clippy::type_complexity)]
fn search_github_for_task_id(
    _client: &GitHubClient,
    config: &GitHubConfig,
    task_id: &str,
) -> Result<Option<(u64, String, String, Option<String>)>> {
    let output = std::process::Command::new("gh")
        .args([
            "issue",
            "list",
            "--repo",
            &format!("{}/{}", config.owner, config.repo),
            "--search",
            &format!("\"**TaskGuard ID:** {}\" in:body", task_id),
            "--json",
            "number,title,body,state",
            "--limit",
            "1",
        ])
        .output()
        .context("Failed to search GitHub issues")?;

    if !output.status.success() {
        return Ok(None);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.trim().is_empty() || stdout.trim() == "[]" {
        return Ok(None);
    }

    // Parse JSON response
    let issues: Vec<serde_json::Value> = serde_json::from_str(&stdout).unwrap_or_default();

    if let Some(issue) = issues.first() {
        let number = issue["number"].as_u64().unwrap_or(0);
        let title = issue["title"].as_str().unwrap_or("").to_string();
        let body = issue["body"].as_str().unwrap_or("");

        // Extract branch from issue body if present
        let branch = body
            .lines()
            .find(|line| line.starts_with("**Source Branch:**"))
            .map(|line| {
                line.trim_start_matches("**Source Branch:**")
                    .trim()
                    .to_string()
            });

        let state = issue["state"].as_str().unwrap_or("OPEN").to_string();

        if number > 0 {
            return Ok(Some((number, title, state, branch)));
        }
    }

    Ok(None)
}

/// Extract the Context section from task markdown content.
/// Returns the content between "## Context" and the next "##" header.
fn extract_context_section(content: &str) -> Option<String> {
    let lines: Vec<&str> = content.lines().collect();

    // Find "## Context" header (case-insensitive match)
    let context_start = lines
        .iter()
        .position(|line| line.trim().to_lowercase() == "## context")?;

    // Collect lines until next ## header or end
    let context_content: String = lines[context_start + 1..]
        .iter()
        .take_while(|line| !line.starts_with("## "))
        .skip_while(|line| line.trim().is_empty())
        .copied()
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string();

    if context_content.is_empty() {
        None
    } else {
        Some(context_content)
    }
}

pub fn run(
    limit: usize,
    verbose: bool,
    remote: bool,
    github: bool,
    backfill_project: bool,
    dry_run: bool,
) -> Result<()> {
    // Load all tasks first
    let current_tasks = load_all_tasks().context("Failed to load tasks")?;

    // GitHub sync mode
    if github {
        return run_github_sync(&current_tasks, backfill_project, dry_run);
    }

    let current_dir = env::current_dir().context("Failed to get current directory")?;

    // Initialize git analyzer
    let git_analyzer = GitAnalyzer::new(&current_dir)
        .context("Failed to initialize Git analyzer. Make sure you're in a Git repository.")?;

    if remote {
        println!("🌐 REMOTE SYNC MODE");
        return run_remote_sync(&git_analyzer, &current_tasks, limit, verbose, dry_run);
    }

    println!("🔍 ANALYZING LOCAL GIT HISTORY");
    println!(
        "   Scanning {} recent commits for task activity...\n",
        limit
    );

    // Analyze git activity
    let activities = git_analyzer
        .analyze_task_activity(Some(limit))
        .context("Failed to analyze Git activity")?;

    if activities.is_empty() {
        println!("ℹ️  No task-related activity found in recent commits.");
        println!("   Tip: Reference task IDs in commit messages (e.g., 'Fix bug in backend-001')");
        return Ok(());
    }

    println!("📊 TASK ACTIVITY ANALYSIS");
    println!("   Found activity for {} tasks:\n", activities.len());

    let mut suggestions_count = 0;

    for activity in &activities {
        // Find corresponding task
        let current_task = current_tasks.iter().find(|t| t.id == activity.task_id);

        let task_title = current_task
            .map(|t| t.title.as_str())
            .unwrap_or("Unknown task");

        let current_status = current_task
            .map(|t| t.status.to_string())
            .unwrap_or("unknown".to_string());

        println!("📝 {} - {}", activity.task_id, task_title);

        if let Some(last_activity) = activity.last_activity {
            let days_ago = (chrono::Utc::now() - last_activity).num_days();
            println!("   Last activity: {} days ago", days_ago);
        }

        println!("   Current status: {}", current_status);
        println!("   Commits found: {}", activity.commits.len());

        if verbose {
            println!("   Recent commits:");
            for commit in activity.commits.iter().take(3) {
                let short_oid = &commit.oid[..8];
                let short_msg = commit.message.lines().next().unwrap_or("");
                let short_msg = if short_msg.len() > 60 {
                    format!("{}...", &short_msg[..60])
                } else {
                    short_msg.to_string()
                };
                println!("     {} - {}", short_oid, short_msg);
            }
        }

        // Show suggestions
        if let Some(suggested_status) = &activity.suggested_status
            && suggested_status != &current_status && activity.confidence > 0.5 {
                suggestions_count += 1;
                println!(
                    "   💡 SUGGESTION: Consider changing status to '{}'",
                    suggested_status
                );
                println!("      Confidence: {:.0}%", activity.confidence * 100.0);
                println!("      Rationale: Based on commit message patterns");
            }

        println!();
    }

    // Repository statistics
    if verbose {
        println!("🔧 REPOSITORY STATISTICS");
        let stats = git_analyzer
            .get_repo_stats()
            .context("Failed to get repository statistics")?;

        for (key, value) in stats {
            println!("   {}: {}", key, value);
        }
        println!();
    }

    // Summary
    if suggestions_count > 0 {
        println!("✨ RECOMMENDATIONS");
        println!(
            "   Found {} task status suggestions based on Git activity",
            suggestions_count
        );
        println!("   Review the suggestions above and update task files manually");
        println!("   Future versions will support automatic updates with confirmation");
    } else {
        println!("✅ ALL GOOD");
        println!("   No status changes recommended based on current Git activity");
    }

    // Next steps guidance
    println!("\n🎯 NEXT STEPS");

    // Find tasks with no recent activity
    let all_task_ids: Vec<String> = current_tasks.iter().map(|t| t.id.clone()).collect();
    let active_task_ids: Vec<String> = activities.iter().map(|a| a.task_id.clone()).collect();

    let stale_tasks: Vec<String> = all_task_ids
        .into_iter()
        .filter(|id| !active_task_ids.contains(id))
        .collect();

    if !stale_tasks.is_empty() {
        println!(
            "   📋 Tasks with no recent Git activity: {}",
            stale_tasks.len()
        );
        if verbose {
            for task_id in stale_tasks.iter().take(5) {
                if let Some(task) = current_tasks.iter().find(|t| t.id == *task_id)
                    && task.status.to_string() != "done" {
                        println!("      {} - {} ({})", task_id, task.title, task.status);
                    }
            }
        }
    }

    // Show most active tasks
    if !activities.is_empty() {
        println!(
            "   🔥 Most active task: {} ({} commits)",
            activities[0].task_id,
            activities[0].commits.len()
        );
    }

    Ok(())
}

/// Handle remote synchronization workflow
fn run_remote_sync(
    git_analyzer: &GitAnalyzer,
    current_tasks: &[Task],
    limit: usize,
    verbose: bool,
    dry_run: bool,
) -> Result<()> {
    // Get available remotes
    let remotes = git_analyzer
        .get_remotes()
        .context("Failed to get repository remotes")?;

    if remotes.is_empty() {
        return Err(anyhow::anyhow!(
            "No remotes configured. Add a remote repository first."
        ));
    }

    // Use 'origin' if available, otherwise use first remote
    let remote_name = if remotes.contains(&"origin".to_string()) {
        "origin"
    } else {
        &remotes[0]
    };

    println!("   Using remote: {}", remote_name);
    println!("   Scanning {} recent commits from remote...\n", limit);

    // Analyze local activity
    println!("🔍 Analyzing local Git history...");
    let local_activities = git_analyzer
        .analyze_task_activity(Some(limit))
        .context("Failed to analyze local Git activity")?;

    // Analyze remote activity
    println!("🌐 Analyzing remote Git history...");
    let remote_activities = git_analyzer
        .analyze_remote_task_activity(remote_name, Some(limit))
        .context("Failed to analyze remote Git activity")?;

    if local_activities.is_empty() && remote_activities.is_empty() {
        println!("ℹ️  No task-related activity found in local or remote commits.");
        println!("   Tip: Reference task IDs in commit messages (e.g., 'Fix bug in backend-001')");
        return Ok(());
    }

    // Detect conflicts between local and remote suggestions
    println!("⚖️  Comparing local and remote task suggestions...\n");
    let conflicts = git_analyzer.detect_sync_conflicts(&local_activities, &remote_activities);

    // Show sync analysis results
    display_sync_analysis(&local_activities, &remote_activities, &conflicts, verbose)?;

    if conflicts.is_empty() {
        println!("✅ NO CONFLICTS");
        println!("   Local and remote task suggestions are consistent");
        return Ok(());
    }

    // Handle conflicts based on dry_run mode
    if dry_run {
        println!("🔍 DRY RUN MODE - No changes will be applied");
        display_conflict_preview(&conflicts)?;
    } else {
        handle_sync_conflicts(&conflicts, current_tasks)?;
    }

    Ok(())
}

/// Display comprehensive sync analysis results
fn display_sync_analysis(
    local_activities: &[crate::git::TaskActivity],
    remote_activities: &[crate::git::TaskActivity],
    conflicts: &[crate::git::SyncConflict],
    verbose: bool,
) -> Result<()> {
    println!("📊 SYNC ANALYSIS RESULTS");
    println!("   Local activities: {}", local_activities.len());
    println!("   Remote activities: {}", remote_activities.len());
    println!("   Conflicts detected: {}\n", conflicts.len());

    if verbose {
        if !local_activities.is_empty() {
            println!("📝 LOCAL ACTIVITY:");
            for activity in local_activities.iter().take(5) {
                println!(
                    "   {} - {} commits",
                    activity.task_id,
                    activity.commits.len()
                );
                if let Some(status) = &activity.suggested_status {
                    println!(
                        "     Suggested: {} (confidence: {:.0}%)",
                        status,
                        activity.confidence * 100.0
                    );
                }
            }
            println!();
        }

        if !remote_activities.is_empty() {
            println!("🌐 REMOTE ACTIVITY:");
            for activity in remote_activities.iter().take(5) {
                println!(
                    "   {} - {} commits",
                    activity.task_id,
                    activity.commits.len()
                );
                if let Some(status) = &activity.suggested_status {
                    println!(
                        "     Suggested: {} (confidence: {:.0}%)",
                        status,
                        activity.confidence * 100.0
                    );
                }
            }
            println!();
        }
    }

    Ok(())
}

/// Display conflict preview in dry-run mode
fn display_conflict_preview(conflicts: &[crate::git::SyncConflict]) -> Result<()> {
    println!("⚠️  CONFLICTS THAT WOULD BE RESOLVED:");
    println!();

    for conflict in conflicts {
        println!("📝 Task: {}", conflict.task_id);
        println!(
            "   Local suggestion: {} (confidence: {:.0}%)",
            conflict.local_status,
            conflict.local_confidence * 100.0
        );
        println!(
            "   Remote suggestion: {} (confidence: {:.0}%)",
            conflict.remote_suggested_status,
            conflict.remote_confidence * 100.0
        );

        match conflict.resolution {
            ConflictResolution::AcceptRemote => println!("   → Would accept REMOTE suggestion"),
            ConflictResolution::KeepLocal => println!("   → Would keep LOCAL suggestion"),
            ConflictResolution::Interactive => {
                println!("   → Would prompt for INTERACTIVE resolution")
            }
            ConflictResolution::NoConflict => println!("   → No conflict"),
        }
        println!();
    }

    println!("💡 Run without --dry-run to apply these resolutions");
    Ok(())
}

/// Handle sync conflicts with interactive resolution
fn handle_sync_conflicts(
    conflicts: &[crate::git::SyncConflict],
    current_tasks: &[Task],
) -> Result<()> {
    println!("⚠️  RESOLVING {} CONFLICTS\n", conflicts.len());

    for (i, conflict) in conflicts.iter().enumerate() {
        println!(
            "📝 Conflict {} of {}: {}",
            i + 1,
            conflicts.len(),
            conflict.task_id
        );

        // Find current task for context
        let current_task = current_tasks.iter().find(|t| t.id == conflict.task_id);
        if let Some(task) = current_task {
            println!("   Current status: {}", task.status);
        }

        println!(
            "   Local suggestion: {} (confidence: {:.0}%)",
            conflict.local_status,
            conflict.local_confidence * 100.0
        );
        println!(
            "   Remote suggestion: {} (confidence: {:.0}%)",
            conflict.remote_suggested_status,
            conflict.remote_confidence * 100.0
        );

        let resolution = match conflict.resolution {
            ConflictResolution::AcceptRemote => {
                println!("   💡 Recommendation: Accept remote suggestion (higher confidence)");
                prompt_user_choice("Accept remote suggestion?", true)?
            }
            ConflictResolution::KeepLocal => {
                println!("   💡 Recommendation: Keep local suggestion (higher confidence)");
                prompt_user_choice("Keep local suggestion?", true)?
            }
            ConflictResolution::Interactive => {
                println!("   💡 Both suggestions have similar confidence - your choice");
                prompt_interactive_resolution()?
            }
            ConflictResolution::NoConflict => continue,
        };

        match resolution {
            UserChoice::AcceptRemote => {
                println!(
                    "   ✅ Accepting remote suggestion: {}",
                    conflict.remote_suggested_status
                );
                // TODO: Apply the status change to task file
                println!(
                    "   📝 Manual update required: Change task status to '{}'",
                    conflict.remote_suggested_status
                );
            }
            UserChoice::KeepLocal => {
                println!("   ✅ Keeping local suggestion: {}", conflict.local_status);
            }
            UserChoice::Skip => {
                println!("   ⏭️  Skipping this conflict");
            }
        }
        println!();
    }

    println!("🎯 SYNC COMPLETE");
    println!("   All conflicts have been resolved");
    println!("   Note: Manual task file updates may be required");

    Ok(())
}

#[derive(Debug)]
enum UserChoice {
    AcceptRemote,
    KeepLocal,
    Skip,
}

/// Prompt user for a yes/no choice with recommendation
fn prompt_user_choice(question: &str, default: bool) -> Result<UserChoice> {
    let default_str = if default { "Y/n" } else { "y/N" };
    print!("   {} ({}) or (s)kip: ", question, default_str);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();

    match input.as_str() {
        "" => Ok(if default {
            UserChoice::AcceptRemote
        } else {
            UserChoice::KeepLocal
        }),
        "y" | "yes" => Ok(if default {
            UserChoice::AcceptRemote
        } else {
            UserChoice::KeepLocal
        }),
        "n" | "no" => Ok(if default {
            UserChoice::KeepLocal
        } else {
            UserChoice::AcceptRemote
        }),
        "s" | "skip" => Ok(UserChoice::Skip),
        _ => {
            println!("   Invalid input. Please enter y, n, or s.");
            prompt_user_choice(question, default)
        }
    }
}

/// Prompt user for interactive conflict resolution
fn prompt_interactive_resolution() -> Result<UserChoice> {
    print!("   Choose: (r)emote, (l)ocal, or (s)kip: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();

    match input.as_str() {
        "r" | "remote" => Ok(UserChoice::AcceptRemote),
        "l" | "local" => Ok(UserChoice::KeepLocal),
        "s" | "skip" => Ok(UserChoice::Skip),
        _ => {
            println!("   Invalid input. Please enter r, l, or s.");
            prompt_interactive_resolution()
        }
    }
}

// ========================================
// GITHUB SYNC FUNCTIONS
// ========================================

fn run_github_sync(tasks: &[Task], backfill_project: bool, dry_run: bool) -> Result<()> {
    println!("🌐 GITHUB SYNC MODE");
    if backfill_project {
        println!("   Mode: Backfill Projects v2 Board");
        println!("   Adding all existing issues to Projects v2 board...\n");
    } else {
        println!("   Syncing local tasks with GitHub Issues and Projects...\n");
    }

    // Create GitHub client early for auto-setup
    let client = GitHubClient::new()
        .context("Failed to create GitHub client. Make sure you've run `gh auth login`")?;

    // Check if GitHub is configured, if not try to auto-configure
    let config = if !is_github_sync_enabled()? {
        // Try to detect repository from git remote
        println!("🔍 No GitHub configuration found, attempting auto-setup...");

        let current_dir = env::current_dir().context("Failed to get current directory")?;

        let git_analyzer = GitAnalyzer::new(&current_dir)
            .context("Failed to initialize Git analyzer. Make sure you're in a Git repository.")?;

        // Get remote URL and parse owner/repo
        let remotes = git_analyzer.get_remotes()?;
        let remote_name = if remotes.contains(&"origin".to_string()) {
            "origin"
        } else if !remotes.is_empty() {
            &remotes[0]
        } else {
            println!("❌ No git remotes found");
            println!(
                "💡 TIP: Add a GitHub remote with: git remote add origin https://github.com/owner/repo"
            );
            return Ok(());
        };

        // Get remote URL using git command
        let remote_url_output = std::process::Command::new("git")
            .args(["remote", "get-url", remote_name])
            .current_dir(&current_dir)
            .output()
            .context("Failed to get git remote URL")?;

        if !remote_url_output.status.success() {
            println!("❌ Failed to get remote URL for '{}'", remote_name);
            return Ok(());
        }

        let remote_url = String::from_utf8_lossy(&remote_url_output.stdout)
            .trim()
            .to_string();

        // Parse GitHub owner/repo from remote URL
        let (owner, repo) = parse_github_repo(&remote_url)
            .context("Could not parse GitHub repository from remote URL")?;

        println!("✓ Detected repository: {}/{}", owner, repo);

        // Auto-create project
        let (_project_number, _project_id) = GitHubProjectSetup::auto_create_project(
            &client, &owner, &repo, true, // verbose
        )?;

        // Config was written by auto_create_project, reload it
        load_github_config().context("Failed to load auto-generated GitHub configuration")?
    } else {
        load_github_config().context("Failed to load GitHub configuration")?
    };

    // Load or create task-issue mapper
    let mut mapper = TaskIssueMapper::new().context("Failed to load task-issue mapper")?;

    // Ensure all required status columns exist on the Projects v2 board
    // This provides zero-configuration sync by auto-creating missing columns
    if !dry_run {
        println!("🔍 Checking GitHub Projects v2 board status columns...");
        let project_id =
            GitHubProjectSetup::get_project_id(&client, &config.owner, config.project_number)
                .context("Failed to get project ID")?;

        match GitHubMutations::ensure_status_columns(&client, &project_id) {
            Ok(created) => {
                if created == 0 {
                    println!("   ✅ All required status columns exist");
                } else {
                    println!("   ✅ Status columns setup complete");
                }
            }
            Err(e) => {
                // Don't fail the entire sync if column creation fails
                println!("   ⚠️  Could not verify/create status columns: {}", e);
                println!("   💡 Sync will continue with existing columns");
            }
        }
        println!();
    }

    if backfill_project {
        // Backfill mode: add all existing issues to project board
        println!("🔄 BACKFILL: Adding existing issues to Projects v2 board");
        backfill_project_board(&client, &config, tasks, &mut mapper, dry_run)?;
    } else {
        // Normal sync mode
        println!("📤 PUSH: Local Tasks → GitHub Issues");
        push_tasks_to_github(&client, &config, tasks, &mut mapper, dry_run)?;

        println!();
        println!("📥 PULL: GitHub Issues → Local Tasks");
        pull_issues_from_github(&client, &config, tasks, &mapper, dry_run)?;
    }

    // Save updated mapping
    if !dry_run {
        mapper.save().context("Failed to save task-issue mapping")?;
        println!();
        println!("✅ Sync mapping saved to .taskguard/github-mapping.json");
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
        // Detect if task is archived
        let is_archived = task.file_path.to_string_lossy().contains("archive");

        // Check if task already has a GitHub issue
        if let Some(mapping) = mapper.get_by_task_id(&task.id) {
            // Task has issue - check if update needed
            let issue = GitHubQueries::get_issue_by_id(client, &mapping.issue_id)
                .context(format!("Failed to get issue for task {}", task.id))?;

            // Compare states
            let github_state = map_github_state_to_taskguard(&issue.state);
            let local_state = task.status.to_string();

            if local_state != github_state {
                println!("   🔄 {} - {} (status mismatch)", task.id, task.title);
                println!("      Local: {:?}, GitHub: {}", task.status, issue.state);

                if !dry_run {
                    // Update GitHub to match local
                    let new_state = map_taskguard_status_to_github(&task.status);
                    GitHubMutations::update_issue_state(client, &issue.id, new_state)
                        .context(format!("Failed to update issue state for task {}", task.id))?;
                    println!("      ✅ Updated GitHub issue state to {}", new_state);

                    // Update Projects v2 status column if issue is on the board
                    if !mapping.project_item_id.is_empty() {
                        println!("      🎯 Updating project status...");

                        let project_id = GitHubProjectSetup::get_project_id(
                            client,
                            &config.owner,
                            config.project_number,
                        )
                        .context("Failed to get project ID")?;

                        let (field_id, options) =
                            GitHubMutations::get_status_field_info(client, &project_id)
                                .context("Failed to get status field info")?;

                        if let Some(option_id) =
                            TaskIssueMapper::find_best_status_option(&task.status, &options)
                        {
                            GitHubMutations::update_project_item_status(
                                client,
                                &project_id,
                                &mapping.project_item_id,
                                &field_id,
                                &option_id,
                            )
                            .context(format!(
                                "Failed to update project status for task {}",
                                task.id
                            ))?;
                            println!("      ✅ Updated project column");
                        }
                    }

                    updated += 1;
                } else {
                    println!("      Would update GitHub issue to {:?}", task.status);
                }
            } else {
                skipped += 1;
            }
        } else {
            // No issue exists in local mapping - check GitHub for cross-branch duplicates
            if let Ok(Some((existing_num, existing_title, _existing_state, existing_branch))) =
                search_github_for_task_id(client, config, &task.id)
            {
                // Found existing issue on GitHub - check if it's the same task
                let branch_info = existing_branch.as_deref().unwrap_or("unknown");

                if existing_title == task.title {
                    // Same task from different branch - offer to adopt
                    println!("   ⚠️  {} - {} (found on GitHub)", task.id, task.title);
                    println!(
                        "      Already synced from branch '{}' (Issue #{})",
                        branch_info, existing_num
                    );
                    println!("      Adopting existing issue into local mapping...");

                    if !dry_run {
                        // Get issue details via GraphQL to get the node ID
                        let issues = GitHubQueries::search_issues_by_taskguard_id(
                            client,
                            &config.owner,
                            &config.repo,
                            &task.id,
                        )
                        .unwrap_or_default();

                        if let Some(existing_issue) = issues.first() {
                            let mapping = IssueMapping {
                                task_id: task.id.clone(),
                                issue_number: existing_issue.number,
                                issue_id: existing_issue.id.clone(),
                                project_item_id: String::new(), // Will be populated if needed
                                synced_at: chrono::Utc::now().to_rfc3339(),
                                is_archived,
                            };
                            mapper.add_mapping(mapping).context(format!(
                                "Failed to adopt issue mapping for task {}",
                                task.id
                            ))?;
                            println!(
                                "      ✅ Adopted Issue #{} into local mapping",
                                existing_num
                            );
                        }
                    }
                    skipped += 1;
                    continue;
                } else {
                    // Different task with same ID - TRUE COLLISION
                    println!("   ❌ {} - ID CONFLICT DETECTED!", task.id);
                    println!("      Local:  \"{}\"", task.title);
                    println!(
                        "      GitHub: \"{}\" (Issue #{}, branch: {})",
                        existing_title, existing_num, branch_info
                    );
                    println!("      ⚠️  These are DIFFERENT tasks with the same ID!");
                    println!("      → Rename your local task ID to avoid conflict");
                    skipped += 1;
                    continue;
                }
            }

            // No existing issue found - create new one
            if is_archived {
                println!(
                    "   ➕ {} - {} (creating closed issue for archived task)",
                    task.id, task.title
                );
            } else {
                println!("   ➕ {} - {} (creating issue)", task.id, task.title);
            }

            if !dry_run {
                // Build issue body with TaskGuard ID, branch info, and hash for tracking
                let branch_name = get_current_branch().unwrap_or_else(|| "unknown".to_string());
                let task_hash = hash_task_content(task);

                // Try to extract Context section, fall back to first paragraph
                let description = if let Some(context) = extract_context_section(&task.content) {
                    // Use Context section content
                    if context.len() > 200 {
                        context[..200].to_string()
                    } else {
                        context
                    }
                } else {
                    // Fallback: Extract first paragraph
                    let first_para = task
                        .content
                        .lines()
                        .skip_while(|line| line.starts_with('#') || line.trim().is_empty())
                        .take_while(|line| !line.trim().is_empty())
                        .collect::<Vec<_>>()
                        .join("\n");

                    if first_para.is_empty() {
                        "No description".to_string()
                    } else if first_para.len() > 200 {
                        first_para[..200].to_string()
                    } else {
                        first_para
                    }
                };

                let description = description.as_str();

                let archived_note = if is_archived {
                    "\n\n📦 **Note:** This task was archived when the issue was created."
                } else {
                    ""
                };

                // Build task file link
                let file_path = format!("tasks/{}/{}.md", task.area, task.id);
                let file_url = format!(
                    "https://github.com/{}/{}/blob/{}/{}",
                    config.owner, config.repo, branch_name, file_path
                );

                let body = format!(
                    "**TaskGuard ID:** {}  \n**Task File:** [{}]({})\n**Source Branch:** {}\n**Hash:** {}\n\n## Description\n\n{}{}\n\n---\n*Synced from TaskGuard*",
                    task.id,
                    file_path,
                    file_url,
                    branch_name,
                    task_hash,
                    description,
                    archived_note
                );

                let mut issue = GitHubMutations::create_issue(
                    client,
                    &config.owner,
                    &config.repo,
                    &task.title,
                    Some(&body),
                )
                .context(format!("Failed to create issue for task {}", task.id))?;

                println!("      ✅ Created issue #{}", issue.number);

                // If task is archived, immediately close the issue
                if is_archived {
                    GitHubMutations::update_issue_state(client, &issue.id, "CLOSED").context(
                        format!("Failed to close issue for archived task {}", task.id),
                    )?;
                    println!("      🔒 Closed issue (archived task)");
                    issue.state = "CLOSED".to_string();
                }

                // Add to Projects v2 board
                println!("      📋 Adding to project...");

                // 1. Get project ID
                let project_id = GitHubProjectSetup::get_project_id(
                    client,
                    &config.owner,
                    config.project_number,
                )
                .context("Failed to get project ID")?;

                // 2. Add issue to project
                let project_item_id =
                    GitHubMutations::add_issue_to_project(client, &project_id, &issue.id)
                        .context(format!("Failed to add issue #{} to project", issue.number))?;
                println!(
                    "      ✅ Added to project (item: {})",
                    &project_item_id[..8]
                );

                // 3. Get status field info
                let (field_id, options) =
                    GitHubMutations::get_status_field_info(client, &project_id)
                        .context("Failed to get status field info")?;

                // 4. Find matching status column and set it
                if let Some(option_id) =
                    TaskIssueMapper::find_best_status_option(&task.status, &options)
                {
                    // 5. Set status
                    GitHubMutations::update_project_item_status(
                        client,
                        &project_id,
                        &project_item_id,
                        &field_id,
                        &option_id,
                    )
                    .context(format!(
                        "Failed to update status for issue #{}",
                        issue.number
                    ))?;
                    println!("      🎯 Status set successfully");
                } else {
                    println!(
                        "      ⚠️  No matching status column found for '{}'",
                        task.status
                    );
                }

                // 6. Save mapping with project_item_id and archived status
                let mapping = IssueMapping {
                    task_id: task.id.clone(),
                    issue_number: issue.number,
                    issue_id: issue.id.clone(),
                    project_item_id,
                    synced_at: chrono::Utc::now().to_rfc3339(),
                    is_archived,
                };
                mapper
                    .add_mapping(mapping)
                    .context(format!("Failed to save mapping for task {}", task.id))?;

                created += 1;
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
    let issues =
        GitHubQueries::get_repository_issues(client, &config.owner, &config.repo, Some(100))
            .context("Failed to get repository issues")?;

    let mut mapped_count = 0;
    let mut orphaned_issues = Vec::new();
    let mut updates_needed = Vec::new();
    let mut archived_with_changes = Vec::new();

    for issue in issues {
        // Check if this issue is tracked
        if let Some(mapping) = mapper.get_by_issue_number(issue.number) {
            mapped_count += 1;

            // Find the task (including archived)
            if let Some(task) = tasks.iter().find(|t| t.id == mapping.task_id) {
                let is_archived = task.file_path.to_string_lossy().contains("archive");
                let github_state = map_github_state_to_taskguard(&issue.state);
                let local_state = task.status.to_string();

                if github_state != local_state {
                    if is_archived {
                        // Archived task with status mismatch - special handling
                        archived_with_changes.push((
                            task.id.clone(),
                            local_state,
                            github_state.to_string(),
                            issue.number,
                        ));
                    } else {
                        updates_needed.push((
                            task.id.clone(),
                            local_state,
                            github_state.to_string(),
                        ));
                    }
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
        println!(
            "   ⚠️  {} ORPHANED ISSUES (no matching TaskGuard task):",
            orphaned_issues.len()
        );
        for issue in orphaned_issues.iter().take(10) {
            println!("      #{} - \"{}\"", issue.number, issue.title);
        }

        if orphaned_issues.len() > 10 {
            println!("      ... and {} more", orphaned_issues.len() - 10);
        }

        println!();
        println!("   💡 SUGGESTED ACTIONS:");
        println!("      1. Create tasks manually for these issues");
        println!("      2. Or ignore them (they'll stay on GitHub only)");
        println!("      3. Or ask AI: \"Create TaskGuard tasks for orphaned GitHub issues\"");
    }

    // Report status mismatches for active tasks
    if !updates_needed.is_empty() {
        println!();
        println!(
            "   ⚠️  {} active tasks have status changes on GitHub:",
            updates_needed.len()
        );
        for (task_id, local, github) in &updates_needed {
            println!("      {} - Local: {}, GitHub: {}", task_id, local, github);
        }

        if !dry_run {
            println!();
            println!("   💡 TIP: Update local task files to match GitHub state");
            println!("      Or next sync will push local status back to GitHub");
        }
    }

    // Report status mismatches for archived tasks
    if !archived_with_changes.is_empty() {
        println!();
        println!(
            "   📦 {} ARCHIVED tasks have status changes on GitHub:",
            archived_with_changes.len()
        );
        for (task_id, local, github, issue_num) in &archived_with_changes {
            println!(
                "      {} - Local: {}, GitHub: {} (Issue #{})",
                task_id, local, github, issue_num
            );
        }

        println!();
        println!("   💡 ARCHIVED TASK SYNC OPTIONS:");
        println!("      1. Run 'taskguard restore <task-id>' to unarchive and sync");
        println!("      2. Or ignore - archived tasks won't auto-update from GitHub");
        println!("      3. Next sync will push local archived status back to GitHub");
    }

    if orphaned_issues.is_empty() && updates_needed.is_empty() && archived_with_changes.is_empty() {
        println!("   ✅ All tasks in sync with GitHub");
    }

    Ok(())
}

fn backfill_project_board(
    client: &GitHubClient,
    config: &GitHubConfig,
    tasks: &[Task],
    mapper: &mut TaskIssueMapper,
    dry_run: bool,
) -> Result<()> {
    let mut added = 0;
    let mut skipped = 0;
    let mut already_on_board = 0;
    let mut archived_added = 0;

    // Get project ID once
    let project_id =
        GitHubProjectSetup::get_project_id(client, &config.owner, config.project_number)
            .context("Failed to get project ID")?;

    // Get status field info once
    let (field_id, options) = GitHubMutations::get_status_field_info(client, &project_id)
        .context("Failed to get status field info")?;

    for task in tasks {
        // Detect if task is archived
        let is_archived = task.file_path.to_string_lossy().contains("archive");

        // Check if task has a GitHub issue
        if let Some(mut mapping) = mapper.get_by_task_id(&task.id).cloned() {
            // Check if already on board
            if !mapping.project_item_id.is_empty() {
                already_on_board += 1;
                continue;
            }

            if is_archived {
                println!("   🔄 {} - {} (archived)", task.id, task.title);
            } else {
                println!("   🔄 {} - {}", task.id, task.title);
            }

            if !dry_run {
                // Add issue to project
                let project_item_id =
                    GitHubMutations::add_issue_to_project(client, &project_id, &mapping.issue_id)
                        .context(format!(
                        "Failed to add issue for task {} to project",
                        task.id
                    ))?;

                println!(
                    "      ✅ Added to project (item: {})",
                    &project_item_id[..8]
                );

                // Set status column
                if let Some(option_id) =
                    TaskIssueMapper::find_best_status_option(&task.status, &options)
                {
                    GitHubMutations::update_project_item_status(
                        client,
                        &project_id,
                        &project_item_id,
                        &field_id,
                        &option_id,
                    )
                    .context(format!("Failed to update status for task {}", task.id))?;
                    println!("      🎯 Status set to '{}'", task.status);
                } else {
                    println!(
                        "      ⚠️  No matching status column found for '{}'",
                        task.status
                    );
                }

                // Update mapping with project_item_id and archived status
                mapping.project_item_id = project_item_id;
                mapping.is_archived = is_archived;
                mapper.update_mapping(mapping)?;

                if is_archived {
                    archived_added += 1;
                } else {
                    added += 1;
                }
            } else {
                println!("      Would add to project and set status");
            }
        } else {
            skipped += 1;
        }
    }

    println!();
    println!("📊 BACKFILL SUMMARY");
    println!("   Added to board: {}", added);
    if archived_added > 0 {
        println!(
            "   Archived tasks added: {} (as closed issues)",
            archived_added
        );
    }
    println!("   Already on board: {}", already_on_board);
    println!("   Skipped (no issue): {}", skipped);

    Ok(())
}

// Helper functions for status mapping

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

/// Parse GitHub owner and repo from a git remote URL
///
/// Supports HTTPS and SSH URLs:
/// - https://github.com/owner/repo.git
/// - git@github.com:owner/repo.git
fn parse_github_repo(url: &str) -> Result<(String, String)> {
    // Remove trailing .git if present
    let url = url.trim_end_matches(".git");

    // Try HTTPS format first
    if let Some(captures) = regex::Regex::new(r"github\.com[:/]([^/]+)/(.+)$")
        .ok()
        .and_then(|re| re.captures(url))
    {
        let owner = captures
            .get(1)
            .map(|m| m.as_str().to_string())
            .context("Failed to parse owner from GitHub URL")?;
        let repo = captures
            .get(2)
            .map(|m| m.as_str().to_string())
            .context("Failed to parse repo from GitHub URL")?;
        return Ok((owner, repo));
    }

    Err(anyhow::anyhow!(
        "Could not parse GitHub repository from URL: {}. Expected format: https://github.com/owner/repo or git@github.com:owner/repo",
        url
    ))
}
