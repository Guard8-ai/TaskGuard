use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;
use chrono::{DateTime, Utc, Duration};

use crate::config::{get_tasks_dir, find_taskguard_root};
use crate::task::{Task, TaskStatus};

pub fn run(dry_run: bool, days: Option<u32>) -> Result<()> {
    let tasks_dir = get_tasks_dir()?;
    let root = find_taskguard_root()
        .context("Not in a TaskGuard project")?;
    let archive_dir = root.join(".taskguard").join("archive");

    if !tasks_dir.exists() {
        println!("📁 No tasks directory found.");
        return Ok(());
    }

    let retention_days = days.unwrap_or(30);
    let cutoff_date = Utc::now() - Duration::days(retention_days as i64);

    println!("📦 TaskGuard Archive - Mobile Storage Optimization");
    println!("   Retention: {} days (completed tasks older than this will be archived)", retention_days);
    if dry_run {
        println!("   Mode: DRY RUN (no files will be moved)");
    }
    println!();

    // Find old completed tasks
    let mut files_to_archive = Vec::new();
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
                if task.status == TaskStatus::Done && task.created < cutoff_date {
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
            Err(_) => continue,
        }
    }

    if files_to_archive.is_empty() {
        println!("✅ No tasks to archive!");
        println!("   All completed tasks are within {} day retention period", retention_days);
        return Ok(());
    }

    println!("📋 ARCHIVE SUMMARY");
    println!();
    println!("   Old completed tasks to archive ({}):", files_to_archive.len());
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

    for (path, area, id, _) in files_to_archive {
        let area_archive_dir = archive_dir.join(&area);
        fs::create_dir_all(&area_archive_dir)?;

        let archive_path = area_archive_dir.join(path.file_name().unwrap());

        match fs::rename(&path, &archive_path) {
            Ok(_) => {
                archived_count += 1;
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
