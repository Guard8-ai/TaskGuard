use anyhow::{Context, Result};
use git2::Repository;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use crate::config::{find_taskguard_root, get_tasks_dir, load_tasks_from_dir};
use crate::github::{GitHubClient, GitHubMutations, TaskIssueMapper, is_github_sync_enabled};
use crate::task::{Task, TaskStatus};

pub fn run(dry_run: bool, _days: Option<u32>) -> Result<()> {
    let tasks_dir = get_tasks_dir()?;
    let root = find_taskguard_root().context("Not in a TaskGuard project")?;
    let archive_dir = root.join(".taskguard").join("archive");

    if !tasks_dir.exists() {
        println!("📁 No tasks directory found.");
        return Ok(());
    }

    println!("📦 TaskGuard Archive - Efficiency Optimization");
    println!("   Action: Archive completed tasks (with dependency protection)");
    if dry_run {
        println!("   Mode: DRY RUN (no files will be moved)");
    }
    println!();

    // Check for GitHub integration
    let github_enabled = is_github_sync_enabled()?;
    let mut mapper = if github_enabled {
        Some(TaskIssueMapper::new()?)
    } else {
        None
    };

    // Load ALL tasks to check dependencies
    let all_tasks = load_tasks_from_dir(&tasks_dir)?;

    // Find ALL completed tasks (no age filtering)
    let mut files_to_archive = Vec::new();
    let mut blocked_from_archive = Vec::new();
    let mut github_issues_to_close = Vec::new();
    let mut total_size: u64 = 0;

    for entry in WalkDir::new(&tasks_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
    {
        let path = entry.path();

        match Task::from_file(path) {
            Ok(task) => {
                if task.status == TaskStatus::Done {
                    // Check if any active task depends on this
                    if is_task_referenced(&task.id, &all_tasks) {
                        blocked_from_archive.push((task.id.clone(), task.title.clone()));
                    } else {
                        let metadata = fs::metadata(path)?;
                        total_size += metadata.len();
                        files_to_archive.push((
                            path.to_path_buf(),
                            task.area.clone(),
                            task.id.clone(),
                            task.title.clone(),
                        ));

                        // Check if task has GitHub issue
                        if let Some(ref mapper) = mapper
                            && let Some(mapping) = mapper.get_by_task_id(&task.id)
                        {
                            github_issues_to_close.push((
                                task.id.clone(),
                                mapping.issue_number,
                                mapping.issue_id.clone(),
                            ));
                        }
                    }
                }
            }
            Err(_) => continue,
        }
    }

    // Show blocked tasks first
    if !blocked_from_archive.is_empty() {
        println!("🚫 BLOCKED FROM ARCHIVE (still referenced by active tasks):");
        for (id, title) in &blocked_from_archive {
            println!("   ⚠️  {} - {}", id, title);
        }
        println!();
    }

    if files_to_archive.is_empty() {
        if blocked_from_archive.is_empty() {
            println!("✅ No tasks to archive!");
            println!("   No completed tasks found");
        } else {
            println!("✅ No tasks can be archived!");
            println!("   All completed tasks are still referenced by active tasks");
        }
        return Ok(());
    }

    println!("📋 ARCHIVE SUMMARY");
    println!();
    println!(
        "   Completed tasks to archive ({}):",
        files_to_archive.len()
    );
    for (_, _, id, title) in &files_to_archive {
        println!("   📦 {} - {}", id, title);
    }
    println!();
    println!("💾 STORAGE");
    println!("   Files to archive: {}", files_to_archive.len());
    println!("   Total size: {}", format_size(total_size));
    println!("   Archive location: {}", archive_dir.display());
    println!();

    // Display GitHub integration summary
    if github_enabled && !github_issues_to_close.is_empty() {
        println!("🌐 GITHUB INTEGRATION");
        println!("   The following GitHub issues will be closed:");
        for (task_id, issue_num, _) in &github_issues_to_close {
            println!("   📌 {} → Issue #{} (will close)", task_id, issue_num);
        }
        println!();
        println!("   ⚠️  Archived tasks remain synced via .taskguard/archive/");
        println!("   ℹ️  Run 'taskguard restore <task-id>' to unarchive if needed");
        println!();
    }

    if dry_run {
        println!("🔍 DRY RUN MODE - No files were moved");
        println!("   Run without --dry-run to actually archive");
        return Ok(());
    }

    // Create archive directory structure
    fs::create_dir_all(&archive_dir).context("Failed to create archive directory")?;

    // Move files to archive
    let mut archived_count = 0;
    let mut archived_task_ids = Vec::new();

    for (path, area, id, _) in files_to_archive {
        let area_archive_dir = archive_dir.join(&area);
        fs::create_dir_all(&area_archive_dir)?;

        let archive_path = area_archive_dir.join(path.file_name().unwrap());

        match fs::rename(&path, &archive_path) {
            Ok(_) => {
                archived_count += 1;
                archived_task_ids.push(id.clone());
                println!(
                    "   ✅ Archived: {} → archive/{}/{}",
                    id,
                    area,
                    path.file_name().unwrap().to_string_lossy()
                );
            }
            Err(e) => {
                println!("   ❌ Failed to archive {}: {}", id, e);
            }
        }
    }

    println!();
    println!("✅ ARCHIVE COMPLETE");
    println!("   Archived {} tasks", archived_count);
    println!("   Freed {} in tasks directory", format_size(total_size));
    println!("   Archive: {}", archive_dir.display());

    // Close GitHub issues if enabled
    if github_enabled && !github_issues_to_close.is_empty() {
        println!();
        match close_github_issues(&github_issues_to_close, &mut mapper) {
            Ok(closed_count) => {
                println!("🌐 GITHUB SYNC COMPLETE");
                println!("   Closed {} GitHub issues", closed_count);
            }
            Err(e) => {
                eprintln!("\n⚠️  Warning: Failed to close some GitHub issues: {}", e);
                eprintln!(
                    "   Tasks were archived successfully, but GitHub sync may be incomplete."
                );
            }
        }
    }

    // Create Git commit for tracking
    if !archived_task_ids.is_empty()
        && let Err(e) = create_archive_commit(&root, &archived_task_ids)
    {
        eprintln!("\n⚠️  Warning: Failed to create Git commit: {}", e);
        eprintln!("   Tasks were archived successfully, but Git tracking may be incomplete.");
    }

    Ok(())
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;

    if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Check if a task is referenced by any active (non-done) tasks
fn is_task_referenced(task_id: &str, all_tasks: &[Task]) -> bool {
    for task in all_tasks {
        // Only check active tasks (not completed ones)
        if task.status != TaskStatus::Done && task.dependencies.contains(&task_id.to_string()) {
            return true; // Active task depends on this
        }
    }
    false
}

/// Close GitHub issues for archived tasks and update mappings
fn close_github_issues(
    issues_to_close: &[(String, i64, String)],
    mapper: &mut Option<TaskIssueMapper>,
) -> Result<usize> {
    let client = GitHubClient::new()?;
    let mut closed_count = 0;

    for (task_id, issue_num, issue_id) in issues_to_close {
        println!("   🌐 Closing GitHub issue #{} for {}", issue_num, task_id);

        match GitHubMutations::update_issue_state(&client, issue_id, "CLOSED") {
            Ok(_) => {
                println!("      ✅ Closed issue #{}", issue_num);
                closed_count += 1;

                // Update mapping to mark as archived
                if let Some(m) = mapper
                    && let Some(mut mapping) = m.get_by_task_id(task_id).cloned()
                {
                    mapping.is_archived = true;
                    m.update_mapping(mapping)?;
                }
            }
            Err(e) => {
                eprintln!("      ⚠️  Failed to close issue #{}: {}", issue_num, e);
            }
        }
    }

    Ok(closed_count)
}

/// Create a Git commit to track archived tasks
fn create_archive_commit(repo_path: &Path, task_ids: &[String]) -> Result<()> {
    let repo = Repository::open(repo_path).context("Failed to open Git repository")?;

    // Check if we're in a Git repository and not in a detached HEAD state
    if repo.is_bare() {
        return Err(anyhow::anyhow!("Cannot commit in a bare repository"));
    }

    // Stage all changes in the .taskguard/archive directory
    let mut index = repo.index().context("Failed to get repository index")?;

    // Add archive directory changes
    index
        .add_all(
            [".taskguard/archive/"].iter(),
            git2::IndexAddOption::DEFAULT,
            None,
        )
        .context("Failed to stage archive directory")?;

    // Also stage removed task files from tasks/ directory
    index
        .update_all(["."].iter(), None)
        .context("Failed to update index")?;

    index.write().context("Failed to write index")?;

    let tree_id = index.write_tree().context("Failed to write tree")?;
    let tree = repo.find_tree(tree_id).context("Failed to find tree")?;

    // Get HEAD commit as parent
    let head = repo.head().context("Failed to get HEAD")?;
    let parent_commit = head
        .peel_to_commit()
        .context("Failed to get parent commit")?;

    // Get signature for commit
    let signature = repo.signature().context("Failed to get Git signature")?;

    // Create commit message with task IDs
    let task_list = task_ids.join(", ");
    let commit_message = format!("Archive completed tasks: {}", task_list);

    // Create the commit
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        &commit_message,
        &tree,
        &[&parent_commit],
    )
    .context("Failed to create commit")?;

    println!("\n📝 Git commit created:");
    println!("   Message: {}", commit_message);

    Ok(())
}
