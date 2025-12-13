use anyhow::Result;
use std::fs;
use walkdir::WalkDir;

use crate::config::{get_tasks_dir, load_tasks_from_dir};
use crate::github::{TaskIssueMapper, is_github_sync_enabled};
use crate::task::{Task, TaskStatus};

pub fn run(dry_run: bool, _days: Option<u32>) -> Result<()> {
    let tasks_dir = get_tasks_dir()?;

    if !tasks_dir.exists() {
        println!("📁 No tasks directory found.");
        return Ok(());
    }

    println!("🧹 TaskGuard Clean - Efficiency Optimization");
    println!("   Action: Delete ALL completed tasks");
    if dry_run {
        println!("   Mode: DRY RUN (no files will be deleted)");
    }
    println!();

    // Load ALL tasks to check dependencies
    let all_tasks = load_tasks_from_dir(&tasks_dir)?;

    // Find completed tasks that are SAFE to delete
    let mut files_to_delete = Vec::new();
    let mut protected_tasks = Vec::new();
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
                // Check if task is completed (no age check)
                if task.status == TaskStatus::Done {
                    // CRITICAL: Check if any active task depends on this
                    if is_task_referenced(&task.id, &all_tasks) {
                        protected_tasks.push((task.id.clone(), task.title.clone()));
                    } else {
                        let metadata = fs::metadata(path)?;
                        total_size += metadata.len();
                        files_to_delete.push((
                            path.to_path_buf(),
                            task.id.clone(),
                            task.title.clone(),
                        ));
                    }
                }
            }
            Err(_) => {
                // Skip files that can't be parsed
                continue;
            }
        }
    }

    // Check for GitHub integration BEFORE allowing clean
    let github_enabled = is_github_sync_enabled().unwrap_or(false);

    if github_enabled {
        if let Ok(mapper) = TaskIssueMapper::new() {
            let mut synced_tasks = Vec::new();

            for (path, task_id, title) in &files_to_delete {
                if mapper.get_by_task_id(task_id).is_some() {
                    synced_tasks.push((task_id.clone(), title.clone(), path.clone()));
                }
            }

            if !synced_tasks.is_empty() {
                println!("⚠️  GITHUB SYNC PROTECTION");
                println!();
                println!("   The following tasks are synced with GitHub Issues:");
                for (id, title, _) in &synced_tasks {
                    println!("   🌐 {} - {}", id, title);
                }
                println!();
                println!("   ❌ BLOCKED: Cannot delete synced tasks with 'clean'");
                println!();
                println!("💡 OPTIONS:");
                println!(
                    "   1. Use 'taskguard archive' instead (preserves history + closes GitHub issues)"
                );
                println!("   2. Manually close GitHub issues, then clean");
                println!("   3. Disable GitHub sync in .taskguard/github.toml");
                println!();

                // Remove synced tasks from deletion list
                files_to_delete.retain(|(_, id, _)| {
                    !synced_tasks.iter().any(|(synced_id, _, _)| synced_id == id)
                });

                if files_to_delete.is_empty() {
                    println!("   ℹ️  No non-synced tasks to clean");
                    return Ok(());
                }

                println!(
                    "   ℹ️  Continuing with {} non-synced tasks",
                    files_to_delete.len()
                );
                println!();
            }
        }
    }

    // Find empty directories
    let mut empty_dirs = Vec::new();
    for entry in WalkDir::new(&tasks_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
    {
        let path = entry.path();
        if path == tasks_dir {
            continue; // Skip root tasks directory
        }

        // Check if directory is empty
        if fs::read_dir(path)?.next().is_none() {
            empty_dirs.push(path.to_path_buf());
        }
    }

    // Show protected tasks
    if !protected_tasks.is_empty() {
        println!("🔒 PROTECTED TASKS (cannot delete - still referenced):");
        for (id, title) in &protected_tasks {
            println!("   🛡️  {} - {}", id, title);
        }
        println!();
        println!("💡 TIP: Use 'taskguard archive' instead to preserve history");
        println!();
    }

    // Display what will be deleted
    if files_to_delete.is_empty() && empty_dirs.is_empty() {
        println!("✅ No cleanup needed!");
        println!("   No completed tasks found");
        println!("   No empty directories found");
        return Ok(());
    }

    println!("📋 CLEANUP SUMMARY");
    println!();

    if !files_to_delete.is_empty() {
        println!("   Completed tasks to remove ({}):", files_to_delete.len());
        for (_path, id, title) in &files_to_delete {
            println!("   ❌ {} - {}", id, title);
        }
        println!();
    }

    if !empty_dirs.is_empty() {
        println!("   Empty directories to remove ({}):", empty_dirs.len());
        for dir in &empty_dirs {
            println!("   📁 {}", dir.display());
        }
        println!();
    }

    println!("💾 STORAGE SAVINGS");
    println!("   Files to delete: {}", files_to_delete.len());
    println!("   Directories to delete: {}", empty_dirs.len());
    println!("   Space to reclaim: ~{}", format_size(total_size));
    println!();

    if dry_run {
        println!("🔍 DRY RUN MODE - No files were deleted");
        println!("   Run without --dry-run to actually clean");
        return Ok(());
    }

    // Actually delete files
    let mut deleted_files = 0;
    let mut deleted_dirs = 0;

    for (path, id, _) in files_to_delete {
        match fs::remove_file(&path) {
            Ok(_) => {
                deleted_files += 1;
                println!("   ✅ Deleted: {}", id);
            }
            Err(e) => {
                println!("   ❌ Failed to delete {}: {}", id, e);
            }
        }
    }

    for dir in empty_dirs {
        match fs::remove_dir(&dir) {
            Ok(_) => {
                deleted_dirs += 1;
                println!("   ✅ Removed empty dir: {}", dir.display());
            }
            Err(e) => {
                println!("   ❌ Failed to remove {}: {}", dir.display(), e);
            }
        }
    }

    println!();
    println!("✅ CLEANUP COMPLETE");
    println!("   Deleted {} files", deleted_files);
    println!("   Removed {} empty directories", deleted_dirs);
    println!("   Reclaimed ~{}", format_size(total_size));

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
                return true;
            }
        }
    }
    false
}
