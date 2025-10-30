use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use git2::Repository;

use crate::config::{get_tasks_dir, find_taskguard_root, load_tasks_from_dir};
use crate::task::{Task, TaskStatus};

pub fn run(dry_run: bool, _days: Option<u32>) -> Result<()> {
    let tasks_dir = get_tasks_dir()?;
    let root = find_taskguard_root()
        .context("Not in a TaskGuard project")?;
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

    // Load ALL tasks to check dependencies
    let all_tasks = load_tasks_from_dir(&tasks_dir)?;

    // Find ALL completed tasks (no age filtering)
    let mut files_to_archive = Vec::new();
    let mut blocked_from_archive = Vec::new();
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
    println!("   Completed tasks to archive ({}):", files_to_archive.len());
    for (_, _, id, title) in &files_to_archive {
        println!("   📦 {} - {}", id, title);
    }
    println!();
    println!("💾 STORAGE");
    println!("   Files to archive: {}", files_to_archive.len());
    println!("   Total size: {}", format_size(total_size));
    println!("   Archive location: {}", archive_dir.display());
    println!();

    if dry_run {
        println!("🔍 DRY RUN MODE - No files were moved");
        println!("   Run without --dry-run to actually archive");
        return Ok(());
    }

    // Create archive directory structure
    fs::create_dir_all(&archive_dir)
        .context("Failed to create archive directory")?;

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
                println!("   ✅ Archived: {} → archive/{}/{}", id, area, path.file_name().unwrap().to_string_lossy());
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

    // Create Git commit for tracking
    if !archived_task_ids.is_empty() {
        if let Err(e) = create_archive_commit(&root, &archived_task_ids) {
            eprintln!("\n⚠️  Warning: Failed to create Git commit: {}", e);
            eprintln!("   Tasks were archived successfully, but Git tracking may be incomplete.");
        }
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
        if task.status != TaskStatus::Done {
            if task.dependencies.contains(&task_id.to_string()) {
                return true;  // Active task depends on this
            }
        }
    }
    false
}

/// Create a Git commit to track archived tasks
fn create_archive_commit(repo_path: &Path, task_ids: &[String]) -> Result<()> {
    let repo = Repository::open(repo_path)
        .context("Failed to open Git repository")?;

    // Check if we're in a Git repository and not in a detached HEAD state
    if repo.is_bare() {
        return Err(anyhow::anyhow!("Cannot commit in a bare repository"));
    }

    // Stage all changes in the .taskguard/archive directory
    let mut index = repo.index()
        .context("Failed to get repository index")?;

    // Add archive directory changes
    index.add_all([".taskguard/archive/"].iter(), git2::IndexAddOption::DEFAULT, None)
        .context("Failed to stage archive directory")?;

    // Also stage removed task files from tasks/ directory
    index.update_all(["."].iter(), None)
        .context("Failed to update index")?;

    index.write()
        .context("Failed to write index")?;

    let tree_id = index.write_tree()
        .context("Failed to write tree")?;
    let tree = repo.find_tree(tree_id)
        .context("Failed to find tree")?;

    // Get HEAD commit as parent
    let head = repo.head()
        .context("Failed to get HEAD")?;
    let parent_commit = head.peel_to_commit()
        .context("Failed to get parent commit")?;

    // Get signature for commit
    let signature = repo.signature()
        .context("Failed to get Git signature")?;

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
    ).context("Failed to create commit")?;

    println!("\n📝 Git commit created:");
    println!("   Message: {}", commit_message);

    Ok(())
}
