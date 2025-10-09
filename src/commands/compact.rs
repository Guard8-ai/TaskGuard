use anyhow::{Context, Result};
use std::fs;
use walkdir::WalkDir;

use crate::config::get_tasks_dir;
use crate::task::Task;

pub fn run(dry_run: bool) -> Result<()> {
    let tasks_dir = get_tasks_dir()?;

    if !tasks_dir.exists() {
        println!("📁 No tasks directory found.");
        return Ok(());
    }

    println!("🗜️  TaskGuard Compact - Mobile Storage Optimization");
    if dry_run {
        println!("   Mode: DRY RUN (no files will be modified)");
    }
    println!();

    let mut total_files = 0;
    let mut total_before: u64 = 0;
    let mut total_after: u64 = 0;
    let mut compacted_files = Vec::new();

    for entry in WalkDir::new(&tasks_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
    {
        let path = entry.path();
        total_files += 1;

        let content = fs::read_to_string(path)?;
        let before_size = content.len() as u64;
        total_before += before_size;

        // Compact the content
        let compacted = compact_content(&content)?;
        let after_size = compacted.len() as u64;
        total_after += after_size;

        if before_size > after_size {
            let saved = before_size - after_size;
            let reduction = (saved as f64 / before_size as f64) * 100.0;

            if let Ok(task) = Task::from_file(path) {
                compacted_files.push((
                    task.id.clone(),
                    task.title.clone(),
                    before_size,
                    after_size,
                    reduction,
                ));

                if !dry_run {
                    fs::write(path, compacted)?;
                }
            }
        }
    }

    if compacted_files.is_empty() {
        println!("✅ All files already optimized!");
        println!("   No compaction needed");
        return Ok(());
    }

    println!("📋 COMPACTION RESULTS");
    println!();
    for (id, title, before, after, reduction) in &compacted_files {
        println!("   {} - {}", id, title);
        println!("      {} → {} ({:.1}% reduction)", format_size(*before), format_size(*after), reduction);
    }
    println!();

    let total_saved = total_before - total_after;
    let total_reduction = if total_before > 0 {
        (total_saved as f64 / total_before as f64) * 100.0
    } else {
        0.0
    };

    println!("💾 SUMMARY");
    println!("   Total files scanned: {}", total_files);
    println!("   Files compacted: {}", compacted_files.len());
    println!("   Before: {}", format_size(total_before));
    println!("   After: {}", format_size(total_after));
    println!("   Saved: {} ({:.1}% reduction)", format_size(total_saved), total_reduction);
    println!();

    if dry_run {
        println!("🔍 DRY RUN MODE - No files were modified");
        println!("   Run without --dry-run to actually compact");
    } else {
        println!("✅ COMPACTION COMPLETE");
    }

    Ok(())
}

fn compact_content(content: &str) -> Result<String> {
    // Split into YAML and markdown
    let parts: Vec<&str> = content.splitn(3, "---").collect();

    if parts.len() < 3 {
        // No YAML front-matter, just compact markdown
        return Ok(compact_markdown(content));
    }

    let yaml = parts[1].trim();
    let markdown = parts[2];

    // Compact markdown (keep YAML as-is for structure)
    let compacted_markdown = compact_markdown(markdown);

    Ok(format!("---\n{}\n---\n\n{}", yaml, compacted_markdown))
}

fn compact_markdown(text: &str) -> String {
    let mut result = Vec::new();
    let mut prev_was_empty = false;

    for line in text.lines() {
        let trimmed = line.trim_end();

        // Skip excessive empty lines (max 1 consecutive)
        if trimmed.is_empty() {
            if !prev_was_empty {
                result.push(String::new());
                prev_was_empty = true;
            }
            continue;
        }

        result.push(trimmed.to_string());
        prev_was_empty = false;
    }

    // Remove trailing empty lines
    while result.last().map(|s| s.is_empty()).unwrap_or(false) {
        result.pop();
    }

    result.join("\n")
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
