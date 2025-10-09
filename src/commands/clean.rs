use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;
use chrono::{DateTime, Utc, Duration};

use crate::config::get_tasks_dir;
use crate::task::Task;

pub fn run(dry_run: bool, days: Option<u32>) -> Result<()> {
    let tasks_dir = get_tasks_dir()?;

    if !tasks_dir.exists() {
        println!("📁 No tasks directory found.");
        return Ok(());
    }

    let retention_days = days.unwrap_or(30);
    let cutoff_date = Utc::now() - Duration::days(retention_days as i64);

    println!("🧹 TaskGuard Clean - Mobile Storage Optimization");
    println!("   Retention: {} days (completed tasks older than this will be removed)", retention_days);
    if dry_run {
        println!("   Mode: DRY RUN (no files will be deleted)");
    }
    println!();

    // Find old completed tasks
    let mut files_to_delete = Vec::new();
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
                // Check if task is completed and old
                if task.status == crate::task::TaskStatus::Done && task.created < cutoff_date {
                    let metadata = fs::metadata(path)?;
                    total_size += metadata.len();
                    files_to_delete.push((path.to_path_buf(), task.id.clone(), task.title.clone()));
                }
            }
            Err(_) => {
                // Skip files that can't be parsed
                continue;
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

    // Display what will be deleted
    if files_to_delete.is_empty() && empty_dirs.is_empty() {
        println!("✅ No cleanup needed!");
        println!("   All completed tasks are within {} day retention period", retention_days);
        println!("   No empty directories found");
        return Ok(());
    }

    println!("📋 CLEANUP SUMMARY");
    println!();

    if !files_to_delete.is_empty() {
        println!("   Old completed tasks to remove ({}):", files_to_delete.len());
        for (path, id, title) in &files_to_delete {
            println!("   ❌ {} - {} ({})", id, title, format_size(*&total_size / files_to_delete.len() as u64));
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
