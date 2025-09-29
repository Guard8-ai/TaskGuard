use anyhow::Result;
use std::path::PathBuf;

use crate::config::get_tasks_dir;
use crate::task::{Task, TaskStatus, Priority};

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