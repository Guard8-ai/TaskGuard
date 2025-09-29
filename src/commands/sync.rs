use anyhow::{Result, Context};
use std::env;
use std::io::{self, Write};
use crate::git::{GitAnalyzer, ConflictResolution};
use crate::task::Task;
use crate::config::get_tasks_dir;
use walkdir::WalkDir;

pub fn run(limit: usize, verbose: bool, remote: bool, dry_run: bool) -> Result<()> {
    let current_dir = env::current_dir()
        .context("Failed to get current directory")?;

    // Initialize git analyzer
    let git_analyzer = GitAnalyzer::new(&current_dir)
        .context("Failed to initialize Git analyzer. Make sure you're in a Git repository.")?;

    // Load current tasks
    let tasks_dir = get_tasks_dir()
        .context("Failed to get tasks directory")?;

    if !tasks_dir.exists() {
        return Err(anyhow::anyhow!("No tasks directory found. Run 'taskguard init' first."));
    }

    // Find all task files
    let task_files: Vec<_> = WalkDir::new(&tasks_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
        .collect();

    // Parse all tasks
    let mut current_tasks = Vec::new();
    for file in task_files {
        if let Ok(task) = Task::from_file(file.path()) {
            current_tasks.push(task);
        }
    }

    if remote {
        println!("🌐 REMOTE SYNC MODE");
        return run_remote_sync(&git_analyzer, &current_tasks, limit, verbose, dry_run);
    }

    println!("🔍 ANALYZING LOCAL GIT HISTORY");
    println!("   Scanning {} recent commits for task activity...\n", limit);

    // Analyze git activity
    let activities = git_analyzer.analyze_task_activity(Some(limit))
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
        let current_task = current_tasks.iter()
            .find(|t| t.id == activity.task_id);

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
        if let Some(suggested_status) = &activity.suggested_status {
            if suggested_status != &current_status && activity.confidence > 0.5 {
                suggestions_count += 1;
                println!("   💡 SUGGESTION: Consider changing status to '{}'", suggested_status);
                println!("      Confidence: {:.0}%", activity.confidence * 100.0);
                println!("      Rationale: Based on commit message patterns");
            }
        }

        println!();
    }

    // Repository statistics
    if verbose {
        println!("🔧 REPOSITORY STATISTICS");
        let stats = git_analyzer.get_repo_stats()
            .context("Failed to get repository statistics")?;

        for (key, value) in stats {
            println!("   {}: {}", key, value);
        }
        println!();
    }

    // Summary
    if suggestions_count > 0 {
        println!("✨ RECOMMENDATIONS");
        println!("   Found {} task status suggestions based on Git activity", suggestions_count);
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

    let stale_tasks: Vec<String> = all_task_ids.into_iter()
        .filter(|id| !active_task_ids.contains(id))
        .collect();

    if !stale_tasks.is_empty() {
        println!("   📋 Tasks with no recent Git activity: {}", stale_tasks.len());
        if verbose {
            for task_id in stale_tasks.iter().take(5) {
                if let Some(task) = current_tasks.iter().find(|t| t.id == *task_id) {
                    if task.status.to_string() != "done" {
                        println!("      {} - {} ({})", task_id, task.title, task.status);
                    }
                }
            }
        }
    }

    // Show most active tasks
    if !activities.is_empty() {
        println!("   🔥 Most active task: {} ({} commits)",
                activities[0].task_id,
                activities[0].commits.len());
    }

    Ok(())
}

/// Handle remote synchronization workflow
fn run_remote_sync(git_analyzer: &GitAnalyzer, current_tasks: &[Task], limit: usize, verbose: bool, dry_run: bool) -> Result<()> {
    // Get available remotes
    let remotes = git_analyzer.get_remotes()
        .context("Failed to get repository remotes")?;

    if remotes.is_empty() {
        return Err(anyhow::anyhow!("No remotes configured. Add a remote repository first."));
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
    let local_activities = git_analyzer.analyze_task_activity(Some(limit))
        .context("Failed to analyze local Git activity")?;

    // Analyze remote activity
    println!("🌐 Analyzing remote Git history...");
    let remote_activities = git_analyzer.analyze_remote_task_activity(remote_name, Some(limit))
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
fn display_sync_analysis(local_activities: &[crate::git::TaskActivity], remote_activities: &[crate::git::TaskActivity], conflicts: &[crate::git::SyncConflict], verbose: bool) -> Result<()> {
    println!("📊 SYNC ANALYSIS RESULTS");
    println!("   Local activities: {}", local_activities.len());
    println!("   Remote activities: {}", remote_activities.len());
    println!("   Conflicts detected: {}\n", conflicts.len());

    if verbose {
        if !local_activities.is_empty() {
            println!("📝 LOCAL ACTIVITY:");
            for activity in local_activities.iter().take(5) {
                println!("   {} - {} commits", activity.task_id, activity.commits.len());
                if let Some(status) = &activity.suggested_status {
                    println!("     Suggested: {} (confidence: {:.0}%)", status, activity.confidence * 100.0);
                }
            }
            println!();
        }

        if !remote_activities.is_empty() {
            println!("🌐 REMOTE ACTIVITY:");
            for activity in remote_activities.iter().take(5) {
                println!("   {} - {} commits", activity.task_id, activity.commits.len());
                if let Some(status) = &activity.suggested_status {
                    println!("     Suggested: {} (confidence: {:.0}%)", status, activity.confidence * 100.0);
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
        println!("   Local suggestion: {} (confidence: {:.0}%)", conflict.local_status, conflict.local_confidence * 100.0);
        println!("   Remote suggestion: {} (confidence: {:.0}%)", conflict.remote_suggested_status, conflict.remote_confidence * 100.0);

        match conflict.resolution {
            ConflictResolution::AcceptRemote => println!("   → Would accept REMOTE suggestion"),
            ConflictResolution::KeepLocal => println!("   → Would keep LOCAL suggestion"),
            ConflictResolution::Interactive => println!("   → Would prompt for INTERACTIVE resolution"),
            ConflictResolution::NoConflict => println!("   → No conflict"),
        }
        println!();
    }

    println!("💡 Run without --dry-run to apply these resolutions");
    Ok(())
}

/// Handle sync conflicts with interactive resolution
fn handle_sync_conflicts(conflicts: &[crate::git::SyncConflict], current_tasks: &[Task]) -> Result<()> {
    println!("⚠️  RESOLVING {} CONFLICTS\n", conflicts.len());

    for (i, conflict) in conflicts.iter().enumerate() {
        println!("📝 Conflict {} of {}: {}", i + 1, conflicts.len(), conflict.task_id);

        // Find current task for context
        let current_task = current_tasks.iter().find(|t| t.id == conflict.task_id);
        if let Some(task) = current_task {
            println!("   Current status: {}", task.status);
        }

        println!("   Local suggestion: {} (confidence: {:.0}%)", conflict.local_status, conflict.local_confidence * 100.0);
        println!("   Remote suggestion: {} (confidence: {:.0}%)", conflict.remote_suggested_status, conflict.remote_confidence * 100.0);

        let resolution = match conflict.resolution {
            ConflictResolution::AcceptRemote => {
                println!("   💡 Recommendation: Accept remote suggestion (higher confidence)");
                prompt_user_choice("Accept remote suggestion?", true)?
            },
            ConflictResolution::KeepLocal => {
                println!("   💡 Recommendation: Keep local suggestion (higher confidence)");
                prompt_user_choice("Keep local suggestion?", true)?
            },
            ConflictResolution::Interactive => {
                println!("   💡 Both suggestions have similar confidence - your choice");
                prompt_interactive_resolution()?
            },
            ConflictResolution::NoConflict => continue,
        };

        match resolution {
            UserChoice::AcceptRemote => {
                println!("   ✅ Accepting remote suggestion: {}", conflict.remote_suggested_status);
                // TODO: Apply the status change to task file
                println!("   📝 Manual update required: Change task status to '{}'", conflict.remote_suggested_status);
            },
            UserChoice::KeepLocal => {
                println!("   ✅ Keeping local suggestion: {}", conflict.local_status);
            },
            UserChoice::Skip => {
                println!("   ⏭️  Skipping this conflict");
            },
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
        "" => Ok(if default { UserChoice::AcceptRemote } else { UserChoice::KeepLocal }),
        "y" | "yes" => Ok(if default { UserChoice::AcceptRemote } else { UserChoice::KeepLocal }),
        "n" | "no" => Ok(if default { UserChoice::KeepLocal } else { UserChoice::AcceptRemote }),
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