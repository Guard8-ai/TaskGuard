use anyhow::Result;
use std::collections::HashMap;
use walkdir::WalkDir;

use crate::config::get_tasks_dir;
use crate::task::{Task, TaskStatus};

pub fn run(status_filter: Option<String>, area_filter: Option<String>) -> Result<()> {
    let tasks_dir = get_tasks_dir()?;

    if !tasks_dir.exists() {
        println!("📁 No tasks directory found. Run 'taskguard init' first.");
        return Ok(());
    }

    // Find all task files
    let task_files: Vec<_> = WalkDir::new(&tasks_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
        .collect();

    if task_files.is_empty() {
        println!("📋 No tasks found. Create your first task with 'taskguard create'.");
        return Ok(());
    }

    // Parse all tasks
    let mut tasks = Vec::new();
    let mut warnings = Vec::new();

    for file in task_files {
        match Task::from_file(file.path()) {
            Ok(task) => tasks.push(task),
            Err(e) => {
                warnings.push(format!("⚠️  Skipping {}: {}", file.path().display(), e));
            }
        }
    }

    // Apply filters
    if let Some(status) = &status_filter {
        tasks.retain(|task| task.status.to_string() == status.to_lowercase());
    }

    if let Some(area) = &area_filter {
        tasks.retain(|task| task.area == *area);
    }

    // Show warnings
    for warning in warnings {
        println!("{}", warning);
    }

    if tasks.is_empty() {
        println!("📋 No tasks match the specified filters.");
        return Ok(());
    }

    // Group tasks by area
    let mut areas: HashMap<String, Vec<&Task>> = HashMap::new();
    for task in &tasks {
        areas.entry(task.area.clone()).or_default().push(task);
    }

    // Sort areas and tasks
    let mut sorted_areas: Vec<_> = areas.into_iter().collect();
    sorted_areas.sort_by_key(|(area, _)| area.clone());

    for (area, mut area_tasks) in sorted_areas {
        area_tasks.sort_by_key(|task| &task.id);

        println!("\n📁 {}", area.to_uppercase());
        println!("   {}", "─".repeat(area.len() + 4));

        for task in area_tasks {
            let status_icon = match task.status {
                TaskStatus::Todo => "⭕",
                TaskStatus::Doing => "🔄",
                TaskStatus::Review => "👀",
                TaskStatus::Done => "✅",
                TaskStatus::Blocked => "🚫",
            };

            let priority_icon = match task.priority {
                crate::task::Priority::Critical => "🔴",
                crate::task::Priority::High => "🟠",
                crate::task::Priority::Medium => "🟡",
                crate::task::Priority::Low => "🟢",
            };

            println!("   {} {} {} {}",
                status_icon,
                priority_icon,
                task.id,
                task.title
            );

            // Show dependencies if any
            if !task.dependencies.is_empty() {
                println!("      └── Depends on: {}", task.dependencies.join(", "));
            }
        }
    }

    // Show summary
    let total = tasks.len();
    let by_status: HashMap<String, usize> = tasks
        .iter()
        .fold(HashMap::new(), |mut acc, task| {
            *acc.entry(task.status.to_string()).or_insert(0) += 1;
            acc
        });

    println!("\n📊 SUMMARY");
    println!("   Total tasks: {}", total);
    for (status, count) in by_status {
        println!("   {}: {}", status, count);
    }

    Ok(())
}