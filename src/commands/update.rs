use anyhow::Result;
use std::path::PathBuf;

use crate::config::get_tasks_dir;
use crate::task::{Task, TaskStatus, Priority};
use regex::Regex;

pub fn run(field: String, task_id: String, value: String) -> Result<()> {
    let tasks_dir = get_tasks_dir()?;

    // Find the task file
    let task_file_path = find_task_file(&tasks_dir, &task_id)?;

    // Load the task
    let mut task = Task::from_file(&task_file_path)?;

    // Update the specified field
    match field.as_str() {
        "status" => update_status(&mut task, value)?,
        "priority" => update_priority(&mut task, value)?,
        "assignee" => update_assignee(&mut task, value)?,
        "dependencies" => update_dependencies(&mut task, value)?,
        _ => {
            return Err(anyhow::anyhow!(
                "Invalid field '{}'. Valid fields: status, priority, assignee, dependencies",
                field
            ));
        }
    }

    // Save the updated task
    task.save_to_file(&task_file_path)?;

    println!("✅ Updated task {}: {} = {}", task_id, field,
        match field.as_str() {
            "status" => task.status.to_string(),
            "priority" => task.priority.to_string(),
            "assignee" => task.assignee.as_deref().unwrap_or("None").to_string(),
            "dependencies" => task.dependencies.join(", "),
            _ => unreachable!(),
        }
    );

    Ok(())
}

fn find_task_file(tasks_dir: &PathBuf, task_id: &str) -> Result<PathBuf> {
    // Extract area from task ID (e.g., "backend-001" -> "backend")
    let area = task_id.split('-').next()
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

fn update_status(task: &mut Task, value: String) -> Result<()> {
    let new_status = match value.as_str() {
        "todo" => TaskStatus::Todo,
        "doing" => TaskStatus::Doing,
        "review" => TaskStatus::Review,
        "done" => TaskStatus::Done,
        "blocked" => TaskStatus::Blocked,
        _ => {
            return Err(anyhow::anyhow!(
                "Invalid status '{}'. Valid statuses: todo, doing, review, done, blocked",
                value
            ));
        }
    };

    // Validate status transition (basic validation)
    validate_status_transition(&task.status, &new_status)?;

    task.status = new_status;
    Ok(())
}

fn update_priority(task: &mut Task, value: String) -> Result<()> {
    let new_priority = match value.as_str() {
        "low" => Priority::Low,
        "medium" => Priority::Medium,
        "high" => Priority::High,
        "critical" => Priority::Critical,
        _ => {
            return Err(anyhow::anyhow!(
                "Invalid priority '{}'. Valid priorities: low, medium, high, critical",
                value
            ));
        }
    };

    task.priority = new_priority;
    Ok(())
}

fn update_assignee(task: &mut Task, value: String) -> Result<()> {
    if value.is_empty() || value == "none" || value == "null" {
        task.assignee = None;
    } else {
        task.assignee = Some(value);
    }
    Ok(())
}

fn update_dependencies(task: &mut Task, value: String) -> Result<()> {
    if value.is_empty() || value == "none" || value == "null" {
        task.dependencies = Vec::new();
    } else {
        // Parse comma-separated dependencies
        let deps: Vec<String> = value
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        // Basic validation: ensure dependency IDs follow expected format
        for dep in &deps {
            if !dep.contains('-') {
                return Err(anyhow::anyhow!(
                    "Invalid dependency ID '{}'. Expected format: area-number (e.g., backend-001)",
                    dep
                ));
            }
        }

        task.dependencies = deps;
    }
    Ok(())
}

fn validate_status_transition(current: &TaskStatus, new: &TaskStatus) -> Result<()> {
    use TaskStatus::*;

    // Allow any transition for now, but warn about potentially problematic ones
    match (current, new) {
        // Direct todo -> done might indicate missing work
        (Todo, Done) => {
            println!("⚠️  Warning: Transitioning directly from 'todo' to 'done'. Consider using 'doing' for active work.");
        }
        // Done -> anything else might indicate rework
        (Done, new_status) if new_status != &Done => {
            println!("⚠️  Warning: Reopening completed task. Status: {} -> {}", current, new_status);
        }
        _ => {} // All other transitions are fine
    }

    Ok(())
}

pub fn run_task_item(task_id: String, item_index: usize, status: String) -> Result<()> {
    let tasks_dir = get_tasks_dir()?;

    // Validate status input
    let target_completed = match status.as_str() {
        "done" | "completed" | "true" => true,
        "todo" | "incomplete" | "false" => false,
        _ => {
            return Err(anyhow::anyhow!(
                "Invalid status '{}'. Valid values: done, todo",
                status
            ));
        }
    };

    // Find the task file
    let task_file_path = find_task_file(&tasks_dir, &task_id)?;

    // Load the task
    let mut task = Task::from_file(&task_file_path)?;

    // Parse checklist items
    let items = parse_checklist_items(&task.content)?;

    if items.is_empty() {
        return Err(anyhow::anyhow!(
            "No checklist items found in task {}",
            task_id
        ));
    }

    // Validate item index (1-based)
    if item_index == 0 || item_index > items.len() {
        return Err(anyhow::anyhow!(
            "Invalid item index {}. Valid range: 1-{}",
            item_index,
            items.len()
        ));
    }

    let target_item = &items[item_index - 1]; // Convert to 0-based

    // Check if already in target state
    if target_item.completed == target_completed {
        let current_status = if target_item.completed { "done" } else { "todo" };
        println!("✨ Item {} is already {}: {}",
            item_index,
            current_status,
            target_item.text
        );
        return Ok(());
    }

    // Update the task content by modifying the specific item
    let updated_content = update_checklist_item(&task.content, item_index - 1, target_completed)?;
    task.content = updated_content;

    // Save the updated task
    task.save_to_file(&task_file_path)?;

    // Show success message
    let new_status = if target_completed { "done" } else { "todo" };
    let status_icon = if target_completed { "✅" } else { "⭕" };

    println!("✅ Updated task {} item {}: {} [{}] {}",
        task_id,
        item_index,
        status_icon,
        new_status,
        target_item.text
    );

    Ok(())
}

#[derive(Debug, Clone)]
struct ChecklistItem {
    text: String,
    completed: bool,
    #[allow(dead_code)] // Reserved for future line-specific editing features
    line_number: usize,
}

fn parse_checklist_items(content: &str) -> Result<Vec<ChecklistItem>> {
    let mut items = Vec::new();

    // Regex to match checklist items like "- [ ] text" or "- [x] text"
    let checkbox_regex = Regex::new(r"^(\s*)-\s*\[([x\s])\]\s*(.+)$")?;

    for (line_number, line) in content.lines().enumerate() {
        if let Some(captures) = checkbox_regex.captures(line) {
            let checkbox_state = captures.get(2).unwrap().as_str().trim();
            let text = captures.get(3).unwrap().as_str().trim();

            let completed = match checkbox_state {
                "x" | "X" => true,
                _ => false,
            };

            items.push(ChecklistItem {
                text: text.to_string(),
                completed,
                line_number: line_number + 1, // 1-based for user display
            });
        }
    }

    Ok(items)
}

fn update_checklist_item(content: &str, item_index: usize, completed: bool) -> Result<String> {
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let checkbox_regex = Regex::new(r"^(\s*)-\s*\[([x\s])\]\s*(.+)$")?;

    let mut current_item_index = 0;

    for (line_index, line) in lines.iter().enumerate() {
        if checkbox_regex.is_match(line) {
            if current_item_index == item_index {
                // This is the line we need to update
                if let Some(captures) = checkbox_regex.captures(line) {
                    let indent = captures.get(1).unwrap().as_str();
                    let text = captures.get(3).unwrap().as_str();

                    let new_checkbox = if completed { "[x]" } else { "[ ]" };
                    lines[line_index] = format!("{}- {} {}", indent, new_checkbox, text);
                }
                break;
            }
            current_item_index += 1;
        }
    }

    Ok(lines.join("\n"))
}