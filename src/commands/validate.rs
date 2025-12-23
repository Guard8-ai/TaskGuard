use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};
use std::fs;
use walkdir::WalkDir;

use crate::config::{
    Config, find_taskguard_root, get_config_path, get_tasks_dir, load_tasks_from_dir,
};
use crate::github::{TaskIssueMapper, is_github_sync_enabled};
use crate::task::{Task, TaskStatus};

pub fn run(sync_areas: bool, show_orphans: bool) -> Result<()> {
    let tasks_dir = get_tasks_dir()?;

    // Sync areas first if requested
    if sync_areas {
        sync_config_areas()?;
    }

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

    // Load archived tasks for dependency validation
    let archive_dir = find_taskguard_root()
        .ok_or_else(|| anyhow::anyhow!("Not in a TaskGuard project"))?
        .join(".taskguard")
        .join("archive");

    if archive_dir.exists() {
        let archived_tasks = load_tasks_from_dir(&archive_dir).unwrap_or_default();
        tasks.extend(archived_tasks);
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

    // Separate active and archived tasks
    let archived_ids: HashSet<String> = tasks
        .iter()
        .filter(|t| t.file_path.starts_with(&archive_dir))
        .map(|t| t.id.clone())
        .collect();

    let active_tasks: Vec<&Task> = tasks
        .iter()
        .filter(|t| !archived_ids.contains(&t.id))
        .collect();

    // Find dependency issues (only check non-done active tasks)
    let mut dependency_issues = Vec::new();
    let mut circular_deps = Vec::new();
    // Track fully-verified nodes across all cycle checks for efficiency
    let mut cycle_verified: HashSet<String> = HashSet::new();

    for task in &active_tasks {
        // Skip done tasks - they don't need dependency validation
        if matches!(task.status, TaskStatus::Done) {
            continue;
        }

        for dep in &task.dependencies {
            if !all_ids.contains(dep) {
                dependency_issues
                    .push(format!("❌ {}: Depends on missing task '{}'", task.id, dep));
            }
        }

        // Check for circular dependencies
        if has_circular_dependency(task, &task_map, &mut HashSet::new(), &mut cycle_verified) {
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
            let missing_deps: Vec<&String> = task
                .dependencies
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
            let deps_str: Vec<String> = missing_deps
                .iter()
                .map(|dep_id| {
                    if archived_ids.contains(*dep_id) {
                        format!("{} 📦", dep_id)
                    } else {
                        dep_id.to_string()
                    }
                })
                .collect();
            println!(
                "      ❌ {} - {} (waiting for: {})",
                task.id,
                task.title,
                deps_str.join(", ")
            );
        }
        println!();
    }

    // Find orphan tasks (no dependencies AND nothing depends on them)
    let orphan_tasks = find_orphan_tasks(&active_tasks, &archived_ids);
    let orphan_count = orphan_tasks.len();

    // Show orphan details if requested
    if show_orphans {
        println!("🔍 ORPHAN TASKS");
        if orphan_tasks.is_empty() {
            println!(
                "   ✅ No orphan tasks found. All tasks are connected to the dependency graph."
            );
        } else {
            println!("   Tasks with no dependencies and nothing depends on them:");
            println!();
            for task in &orphan_tasks {
                println!("   ⚠️  {} - {}", task.id, task.title);
            }
            println!();
            println!("   To fix, add dependencies:");
            for task in &orphan_tasks {
                println!(
                    "     taskguard update dependencies {} \"<parent-task-id>\"",
                    task.id
                );
            }
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
        println!(
            "   Found {} issues across {} tasks",
            total_issues,
            tasks.len()
        );
    }

    println!();
    println!("📊 SUMMARY");
    println!("   Total tasks: {}", tasks.len());
    println!("   Available: {}", available_tasks.len());
    println!("   Blocked: {}", blocked_tasks.len());
    if orphan_count > 0 {
        println!(
            "   Orphans: {} (use --orphans to see details)",
            orphan_count
        );
    }
    if !archived_ids.is_empty() {
        println!("   Archived tasks: {}", archived_ids.len());
    }
    println!("   Parse errors: {}", parse_errors.len());
    println!("   Dependency issues: {}", dependency_issues.len());

    // GitHub sync validation
    if is_github_sync_enabled().unwrap_or(false)
        && let Ok(mapper) = TaskIssueMapper::new()
    {
        println!();
        println!("🌐 GITHUB SYNC VALIDATION");

        let mut orphaned_mappings = Vec::new();
        let mut archived_synced_tasks = Vec::new();

        for mapping in mapper.get_all_mappings() {
            if !all_ids.contains(&mapping.task_id) {
                orphaned_mappings.push((mapping.task_id.clone(), mapping.issue_number));
            } else if archived_ids.contains(&mapping.task_id) {
                archived_synced_tasks.push((mapping.task_id.clone(), mapping.issue_number));
            }
        }

        if !orphaned_mappings.is_empty() {
            println!("   ⚠️  ORPHANED MAPPINGS (task deleted but mapping remains):");
            for (task_id, issue_num) in &orphaned_mappings {
                println!("      {} → Issue #{} (task not found)", task_id, issue_num);
            }
        }

        if !archived_synced_tasks.is_empty() {
            println!("   📦 ARCHIVED SYNCED TASKS:");
            for (task_id, issue_num) in &archived_synced_tasks {
                println!("      {} → Issue #{} (task archived)", task_id, issue_num);
            }
        }

        if orphaned_mappings.is_empty() && archived_synced_tasks.is_empty() {
            println!("   ✅ No GitHub sync issues found");
        }
    }

    Ok(())
}

/// Find orphan tasks - tasks with no dependencies AND nothing depends on them
/// Note: setup-001 is exempt as it's the root task
fn find_orphan_tasks<'a>(
    active_tasks: &[&'a Task],
    archived_ids: &HashSet<String>,
) -> Vec<&'a Task> {
    // Build reverse dependency map (who depends on whom)
    let mut has_dependents: HashSet<String> = HashSet::new();
    for task in active_tasks {
        for dep in &task.dependencies {
            has_dependents.insert(dep.clone());
        }
    }

    // Orphan = no dependencies AND no dependents AND not setup-001 AND not archived
    active_tasks
        .iter()
        .filter(|t| {
            t.dependencies.is_empty()
                && !has_dependents.contains(&t.id)
                && t.id != "setup-001"
                && !archived_ids.contains(&t.id)
        })
        .copied()
        .collect()
}

/// Check for circular dependencies using proper DFS with gray/black coloring.
/// - `in_stack`: nodes currently being processed (gray) - a cycle exists if we hit one
/// - `visited`: nodes fully processed (black) - safe to skip, already verified no cycles
fn has_circular_dependency(
    task: &Task,
    task_map: &HashMap<String, &Task>,
    in_stack: &mut HashSet<String>,
    visited: &mut HashSet<String>,
) -> bool {
    // If already fully processed, no cycle through this path
    if visited.contains(&task.id) {
        return false;
    }

    // If in current recursion stack, we found a cycle!
    if in_stack.contains(&task.id) {
        return true;
    }

    // Mark as being processed (gray)
    in_stack.insert(task.id.clone());

    for dep_id in &task.dependencies {
        if let Some(dep_task) = task_map.get(dep_id)
            && has_circular_dependency(dep_task, task_map, in_stack, visited)
        {
            return true;
        }
    }

    // Done processing - remove from stack (gray -> black)
    in_stack.remove(&task.id);
    visited.insert(task.id.clone());
    false
}

/// Sync config areas with actual task directories
pub fn sync_config_areas() -> Result<()> {
    let tasks_dir = get_tasks_dir()?;
    let config_path = get_config_path()?;

    // Discover actual areas from filesystem (only directories)
    let mut discovered_areas: Vec<String> = fs::read_dir(&tasks_dir)
        .context("Failed to read tasks directory")?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().ok().is_some_and(|ft| ft.is_dir()))
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| !name.starts_with('.')) // Skip hidden directories
        .collect();
    discovered_areas.sort();

    if discovered_areas.is_empty() {
        println!("📁 No task area directories found.");
        return Ok(());
    }

    // Load current config
    let mut config = Config::load_or_default(&config_path)?;
    let current_areas: HashSet<String> = config.project.areas.iter().cloned().collect();

    // Find new areas not in config
    let new_areas: Vec<&String> = discovered_areas
        .iter()
        .filter(|area| !current_areas.contains(*area))
        .collect();

    if new_areas.is_empty() {
        println!("✅ Config areas are in sync with task directories");
        return Ok(());
    }

    // Report and add new areas
    println!("🔄 Syncing config areas with task directories");
    println!("   Adding new areas:");
    for area in &new_areas {
        println!("   + {}", area);
    }

    // Merge and update config (preserve existing, add new)
    let mut all_areas: Vec<String> = config.project.areas.clone();
    for area in new_areas {
        all_areas.push(area.clone());
    }
    all_areas.sort();
    all_areas.dedup();

    config.project.areas = all_areas;

    // Write back
    config.save(&config_path)?;
    println!("   ✅ Updated .taskguard/config.toml");

    Ok(())
}
