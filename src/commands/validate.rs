use anyhow::Result;
use std::collections::{HashMap, HashSet};
use walkdir::WalkDir;

use crate::config::get_tasks_dir;
use crate::task::{Task, TaskStatus};

pub fn run() -> Result<()> {
    let tasks_dir = get_tasks_dir()?;

    if !tasks_dir.exists() {
        println!("📁 No tasks directory found. Run 'taskguard init' first.");
        return Ok(());
    }

    // Find and parse all task files
    let task_files: Vec<_> = WalkDir::new(&tasks_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
        .collect();

    if task_files.is_empty() {
        println!("📋 No tasks found to validate.");
        return Ok(());
    }

    let mut tasks = Vec::new();
    let mut parse_errors = Vec::new();

    for file in task_files {
        match Task::from_file(file.path()) {
            Ok(task) => tasks.push(task),
            Err(e) => {
                parse_errors.push(format!("❌ {}: {}", file.path().display(), e));
            }
        }
    }

    // Show parse errors
    if !parse_errors.is_empty() {
        println!("🔍 PARSE ERRORS");
        for error in &parse_errors {
            println!("   {}", error);
        }
        println!();
    }

    if tasks.is_empty() {
        println!("❌ No valid tasks found to validate.");
        return Ok(());
    }

    // Build task ID map
    let task_map: HashMap<String, &Task> = tasks.iter().map(|t| (t.id.clone(), t)).collect();
    let all_ids: HashSet<String> = task_map.keys().cloned().collect();

    // Find dependency issues
    let mut dependency_issues = Vec::new();
    let mut circular_deps = Vec::new();

    for task in &tasks {
        for dep in &task.dependencies {
            if !all_ids.contains(dep) {
                dependency_issues.push(format!(
                    "❌ {}: Depends on missing task '{}'",
                    task.id, dep
                ));
            }
        }

        // Check for circular dependencies (simple check)
        if has_circular_dependency(task, &task_map, &mut HashSet::new()) {
            circular_deps.push(task.id.clone());
        }
    }

    // Show dependency issues
    if !dependency_issues.is_empty() {
        println!("🔗 DEPENDENCY ISSUES");
        for issue in &dependency_issues {
            println!("   {}", issue);
        }
        println!();
    }

    if !circular_deps.is_empty() {
        println!("🔄 CIRCULAR DEPENDENCIES");
        for task_id in &circular_deps {
            println!("   ❌ {}: Has circular dependency", task_id);
        }
        println!();
    }

    // Analyze blocked tasks
    let completed_tasks: HashSet<String> = tasks
        .iter()
        .filter(|t| matches!(t.status, TaskStatus::Done))
        .map(|t| t.id.clone())
        .collect();

    let mut blocked_tasks = Vec::new();
    let mut available_tasks = Vec::new();

    for task in &tasks {
        if matches!(task.status, TaskStatus::Done) {
            continue; // Skip completed tasks
        }

        if task.is_available(&completed_tasks.iter().cloned().collect::<Vec<_>>()) {
            available_tasks.push(task);
        } else {
            let missing_deps: Vec<&String> = task.dependencies
                .iter()
                .filter(|dep| !completed_tasks.contains(*dep))
                .collect();
            blocked_tasks.push((task, missing_deps));
        }
    }

    // Show task availability
    println!("🚦 TASK STATUS");

    if !available_tasks.is_empty() {
        println!("   ✅ Available tasks (dependencies satisfied):");
        for task in &available_tasks {
            let status_icon = match task.status {
                TaskStatus::Todo => "⭕",
                TaskStatus::Doing => "🔄",
                TaskStatus::Review => "👀",
                TaskStatus::Blocked => "🚫",
                _ => "❓",
            };
            println!("      {} {} - {}", status_icon, task.id, task.title);
        }
        println!();
    }

    if !blocked_tasks.is_empty() {
        println!("   🚫 Blocked tasks:");
        for (task, missing_deps) in &blocked_tasks {
            let deps_str: Vec<String> = missing_deps.iter().map(|s| s.to_string()).collect();
            println!("      ❌ {} - {} (waiting for: {})",
                task.id, task.title, deps_str.join(", "));
        }
        println!();
    }

    // Summary
    let total_issues = parse_errors.len() + dependency_issues.len() + circular_deps.len();

    if total_issues == 0 {
        println!("✅ VALIDATION PASSED");
        println!("   No issues found in {} tasks", tasks.len());
    } else {
        println!("❌ VALIDATION FAILED");
        println!("   Found {} issues across {} tasks", total_issues, tasks.len());
    }

    println!();
    println!("📊 SUMMARY");
    println!("   Total tasks: {}", tasks.len());
    println!("   Available: {}", available_tasks.len());
    println!("   Blocked: {}", blocked_tasks.len());
    println!("   Parse errors: {}", parse_errors.len());
    println!("   Dependency issues: {}", dependency_issues.len());

    Ok(())
}

fn has_circular_dependency(
    task: &Task,
    task_map: &HashMap<String, &Task>,
    visited: &mut HashSet<String>,
) -> bool {
    if visited.contains(&task.id) {
        return true; // Found a cycle
    }

    visited.insert(task.id.clone());

    for dep_id in &task.dependencies {
        if let Some(dep_task) = task_map.get(dep_id) {
            if has_circular_dependency(dep_task, task_map, visited) {
                return true;
            }
        }
    }

    visited.remove(&task.id);
    false
}