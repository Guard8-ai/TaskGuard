use anyhow::{Result, Context};
use std::env;
use crate::git::GitAnalyzer;
use crate::task::Task;
use crate::config::get_tasks_dir;
use walkdir::WalkDir;

pub fn run(limit: usize, verbose: bool) -> Result<()> {
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
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
        .collect();

    // Parse all tasks
    let mut current_tasks = Vec::new();
    for file in task_files {
        if let Ok(task) = Task::from_file(file.path()) {
            current_tasks.push(task);
        }
    }

    println!("🔍 ANALYZING GIT HISTORY");
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