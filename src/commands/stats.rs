use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;

use crate::config::get_tasks_dir;
use crate::task::{Task, TaskStatus};

pub fn run() -> Result<()> {
    let tasks_dir = get_tasks_dir()?;

    if !tasks_dir.exists() {
        println!("📁 No tasks directory found.");
        return Ok(());
    }

    // Collect all tasks and stats
    let mut total_size: u64 = 0;
    let mut area_stats: HashMap<String, AreaStats> = HashMap::new();
    let mut status_counts: HashMap<String, usize> = HashMap::new();
    let mut largest_tasks: Vec<(String, String, u64)> = Vec::new();

    for entry in WalkDir::new(&tasks_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
    {
        let path = entry.path();
        let metadata = fs::metadata(path)?;
        let size = metadata.len();
        total_size += size;

        match Task::from_file(path) {
            Ok(task) => {
                // Track area stats
                let area_stat = area_stats.entry(task.area.clone()).or_insert(AreaStats {
                    count: 0,
                    total_size: 0,
                    area: task.area.clone(),
                });
                area_stat.count += 1;
                area_stat.total_size += size;

                // Track status counts
                *status_counts.entry(task.status.to_string()).or_insert(0) += 1;

                // Track largest tasks
                largest_tasks.push((task.id.clone(), task.title.clone(), size));
            }
            Err(_) => {
                // Skip unparseable files
                continue;
            }
        }
    }

    // Sort for display
    largest_tasks.sort_by_key(|(_, _, size)| std::cmp::Reverse(*size));
    let mut sorted_areas: Vec<_> = area_stats.into_iter().collect();
    sorted_areas.sort_by_key(|(area, _)| area.clone());

    // Display stats
    println!("📊 TaskGuard Storage Statistics");
    println!();

    println!("💾 TOTAL STORAGE");
    println!("   Tasks directory: {}", format_size(total_size));
    println!("   Total task files: {}", largest_tasks.len());
    println!("   Average file size: {}", format_size(if largest_tasks.is_empty() { 0 } else { total_size / largest_tasks.len() as u64 }));
    println!();

    println!("📁 BY AREA");
    for (area, stats) in sorted_areas {
        let avg_size = if stats.count > 0 { stats.total_size / stats.count as u64 } else { 0 };
        println!("   {} - {} tasks, {} total (avg: {})",
            area,
            stats.count,
            format_size(stats.total_size),
            format_size(avg_size)
        );
    }
    println!();

    println!("📋 BY STATUS");
    let mut sorted_status: Vec<_> = status_counts.into_iter().collect();
    sorted_status.sort_by_key(|(status, _)| status.clone());
    for (status, count) in sorted_status {
        println!("   {}: {}", status, count);
    }
    println!();

    println!("📈 LARGEST TASKS (Top 10)");
    for (id, title, size) in largest_tasks.iter().take(10) {
        println!("   {} - {} ({})", id, title, format_size(*size));
    }

    Ok(())
}

#[derive(Debug)]
struct AreaStats {
    area: String,
    count: usize,
    total_size: u64,
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
