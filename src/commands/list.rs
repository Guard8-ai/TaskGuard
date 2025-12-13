use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;

use crate::config::{find_taskguard_root, get_tasks_dir, load_tasks_from_dir};
use crate::task::{Task, TaskStatus};
use regex::Regex;

pub fn run(
    status_filter: Option<String>,
    area_filter: Option<String>,
    include_archive: bool,
) -> Result<()> {
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
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
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

    // Load archived tasks if flag is set
    let archive_dir = find_taskguard_root()
        .ok_or_else(|| anyhow::anyhow!("Not in a TaskGuard project"))?
        .join(".taskguard")
        .join("archive");

    let has_archive = archive_dir.exists();

    if include_archive && has_archive {
        let archived_tasks = load_tasks_from_dir(&archive_dir).unwrap_or_default();
        tasks.extend(archived_tasks);
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

            // Check if task is archived
            let archive_indicator = if task.file_path.starts_with(&archive_dir) {
                "📦 "
            } else {
                ""
            };

            println!(
                "   {}{} {} {} {}",
                archive_indicator, status_icon, priority_icon, task.id, task.title
            );

            // Show dependencies if any
            if !task.dependencies.is_empty() {
                println!("      └── Depends on: {}", task.dependencies.join(", "));
            }
        }
    }

    // Show summary
    let total = tasks.len();
    let by_status: HashMap<String, usize> = tasks.iter().fold(HashMap::new(), |mut acc, task| {
        *acc.entry(task.status.to_string()).or_insert(0) += 1;
        acc
    });

    println!("\n📊 SUMMARY");
    println!("   Total tasks: {}", total);
    for (status, count) in by_status {
        println!("   {}: {}", status, count);
    }

    // Show tip if archive exists but not included
    if !include_archive && has_archive {
        let archived_count = load_tasks_from_dir(&archive_dir).unwrap_or_default().len();
        if archived_count > 0 {
            println!();
            println!(
                "💡 TIP: {} archived task{} available. Use --include-archive to see them.",
                archived_count,
                if archived_count == 1 { "" } else { "s" }
            );
        }
    }

    Ok(())
}

pub fn run_items(task_id: String) -> Result<()> {
    let tasks_dir = get_tasks_dir()?;

    // Find the task file using the same logic as update command
    let task_file_path = find_task_file(&tasks_dir, &task_id)?;

    // Load the task
    let task = Task::from_file(&task_file_path)?;

    // Parse checklist items from the task content
    let items = parse_checklist_items(&task.content)?;

    if items.is_empty() {
        println!("📋 No checklist items found in task {}", task_id);
        return Ok(());
    }

    println!("📋 Checklist items for task {}:", task_id);
    println!("   {}", task.title);
    println!();

    for (index, item) in items.iter().enumerate() {
        let status_icon = if item.completed { "✅" } else { "⭕" };
        let status_text = if item.completed { "done" } else { "todo" };

        println!(
            "   {}. {} [{}] {}",
            index + 1,
            status_icon,
            status_text,
            item.text
        );
    }

    println!();
    println!("📊 SUMMARY");
    println!("   Total items: {}", items.len());
    let completed = items.iter().filter(|i| i.completed).count();
    println!(
        "   Completed: {} ({:.1}%)",
        completed,
        completed as f32 / items.len() as f32 * 100.0
    );
    println!("   Remaining: {}", items.len() - completed);

    Ok(())
}

#[derive(Debug, Clone)]
pub struct ChecklistItem {
    pub text: String,
    pub completed: bool,
    #[allow(dead_code)] // Reserved for future line-specific editing features
    pub line_number: usize,
}

fn parse_checklist_items(content: &str) -> Result<Vec<ChecklistItem>> {
    let mut items = Vec::new();

    // Regex to match checklist items like "- [ ] text" or "- [x] text"
    let checkbox_regex = Regex::new(r"^(\s*)-\s*\[([x\s])\]\s*(.+)$")?;

    for (line_number, line) in content.lines().enumerate() {
        if let Some(captures) = checkbox_regex.captures(line) {
            let checkbox_state = captures.get(2).unwrap().as_str().trim();
            let text = captures.get(3).unwrap().as_str().trim();

            let completed = matches!(checkbox_state, "x" | "X");

            items.push(ChecklistItem {
                text: text.to_string(),
                completed,
                line_number: line_number + 1, // 1-based for user display
            });
        }
    }

    Ok(items)
}

// Helper function to find task file (similar to update.rs)
fn find_task_file(tasks_dir: &Path, task_id: &str) -> Result<std::path::PathBuf> {
    // Extract area from task ID (e.g., "backend-001" -> "backend")
    let area = task_id
        .split('-')
        .next()
        .ok_or_else(|| anyhow::anyhow!("Invalid task ID format. Expected format: area-number"))?;

    let area_dir = tasks_dir.join(area);
    let task_file = area_dir.join(format!("{}.md", task_id));

    if !task_file.exists() {
        return Err(anyhow::anyhow!(
            "Task file not found: {}. Available tasks can be seen with 'taskguard list'",
            task_file.display()
        ));
    }

    Ok(task_file)
}
