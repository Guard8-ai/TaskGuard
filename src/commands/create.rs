use anyhow::{Context, Result};
use chrono::Utc;
use std::fs;

use crate::config::{get_tasks_dir, get_config_path, Config};
use crate::task::{Task, TaskStatus, Priority};

/// Add a new area to config if it doesn't exist
fn add_area_to_config(config: &mut Config, config_path: &std::path::Path, area: &str) -> Result<bool> {
    if config.project.areas.contains(&area.to_string()) {
        return Ok(false); // Already exists
    }

    // Add area to config
    config.project.areas.push(area.to_string());
    config.project.areas.sort();

    // Save config
    config.save(config_path)?;

    Ok(true) // Was added
}

pub fn run(
    title: String,
    area: Option<String>,
    priority: Option<String>,
    complexity: Option<u8>,
    tags: Option<String>,
    dependencies: Option<String>,
    assignee: Option<String>,
    estimate: Option<String>,
) -> Result<()> {
    let tasks_dir = get_tasks_dir()?;
    let config_path = get_config_path()?;
    let mut config = Config::load_or_default(&config_path)?;

    // Determine area
    let area = area.unwrap_or_else(|| {
        if config.project.areas.contains(&"setup".to_string()) {
            "setup".to_string()
        } else {
            config.project.areas.first().cloned().unwrap_or_else(|| "general".to_string())
        }
    });

    // Auto-add new area to config
    if add_area_to_config(&mut config, &config_path, &area)? {
        println!("📁 Area '{}' added to config", area);
    }

    // Determine priority
    let priority = match priority.as_deref() {
        Some("low") => Priority::Low,
        Some("medium") => Priority::Medium,
        Some("high") => Priority::High,
        Some("critical") => Priority::Critical,
        Some(p) => {
            println!("⚠️  Invalid priority '{}'. Using 'medium'. Valid priorities: {:?}",
                p, config.settings.priorities);
            Priority::Medium
        }
        None => Priority::Medium,
    };

    // Determine complexity (1-10 scale)
    let complexity = match complexity {
        Some(c) if c >= 1 && c <= 10 => Some(c),
        Some(c) => {
            println!("⚠️  Invalid complexity '{}'. Using default '3'. Valid range: 1-10", c);
            Some(3)
        }
        None => Some(3), // Default complexity
    };

    // Parse tags (comma-separated, always include area)
    let mut tag_list: Vec<String> = tags
        .map(|t| t.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())
        .unwrap_or_default();
    if !tag_list.contains(&area) {
        tag_list.insert(0, area.clone());
    }

    // Parse dependencies (comma-separated task IDs)
    let dependency_list: Vec<String> = dependencies
        .map(|d| d.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())
        .unwrap_or_default();

    // Determine assignee (default: "developer")
    let assignee = assignee.or_else(|| Some("developer".to_string()));

    // Generate task ID
    let area_dir = tasks_dir.join(&area);
    let task_id = generate_task_id(&area, &area_dir)?;

    // Create task
    let task = Task {
        id: task_id.clone(),
        title: title.clone(),
        status: TaskStatus::Todo,
        priority,
        tags: tag_list,
        dependencies: dependency_list,
        assignee,
        created: Utc::now(),
        estimate,
        complexity,
        area: area.clone(),
        content: format!(r#"# {}

> **⚠️ SESSION WORKFLOW NOTICE (for AI Agents):**
>
> **This task should be completed in ONE dedicated session.**
>
> When you mark this task as `done`, you MUST:
> 1. Fill the "Session Handoff" section at the bottom with complete implementation details
> 2. Document what was changed, what runtime behavior to expect, and what dependencies were affected
> 3. Create a clear handoff for the developer/next AI agent working on dependent tasks
>
> **If this task has dependents,** the next task will be handled in a NEW session and depends on your handoff for context.

## Context
Brief description of what needs to be done and why.

## Objectives
- Clear, actionable objectives
- Measurable outcomes
- Success criteria

## Tasks
- [ ] Break down the work into specific tasks
- [ ] Each task should be clear and actionable
- [ ] Mark tasks as completed when done

## Acceptance Criteria
✅ **Criteria 1:**
- Specific, testable criteria

✅ **Criteria 2:**
- Additional criteria as needed

## Technical Notes
- Implementation details
- Architecture considerations
- Dependencies and constraints

## Testing
- [ ] Write unit tests for new functionality
- [ ] Write integration tests if applicable
- [ ] Ensure all tests pass before marking task complete
- [ ] Consider edge cases and error conditions

## Version Control

**⚠️ CRITICAL: Always test AND run before committing!**

- [ ] **BEFORE committing**: Build, test, AND run the code to verify it works
  - Run `cargo build --release` (or `cargo build` for debug)
  - Run `cargo test` to ensure tests pass
  - **Actually run/execute the code** to verify runtime behavior
  - Fix all errors, warnings, and runtime issues
- [ ] Commit changes incrementally with clear messages
- [ ] Use descriptive commit messages that explain the "why"
- [ ] Consider creating a feature branch for complex changes
- [ ] Review changes before committing

**Testing requirements by change type:**
- Code changes: Build + test + **run the actual program/command** to verify behavior
- Bug fixes: Verify the bug is actually fixed by running the code, not just compiling
- New features: Test the feature works as intended by executing it
- Minor changes: At minimum build, check warnings, and run basic functionality

## Updates
- {}: Task created

## Session Handoff (AI: Complete this when marking task done)
**For the next session/agent working on dependent tasks:**

### What Changed
- [Document code changes, new files, modified functions]
- [What runtime behavior is new or different]

### Causality Impact
- [What causal chains were created or modified]
- [What events trigger what other events]
- [Any async flows or timing considerations]

### Dependencies & Integration
- [What dependencies were added/changed]
- [How this integrates with existing code]
- [What other tasks/areas are affected]

### Verification & Testing
- [How to verify this works]
- [What to test when building on this]
- [Any known edge cases or limitations]

### Context for Next Task
- [What the next developer/AI should know]
- [Important decisions made and why]
- [Gotchas or non-obvious behavior]
"#, title, Utc::now().format("%Y-%m-%d")),
        file_path: std::path::PathBuf::new(), // Will be set when saved
    };

    // Ensure area directory exists
    fs::create_dir_all(&area_dir)
        .with_context(|| format!("Failed to create area directory: {}", area))?;

    // Write task file
    let file_path = area_dir.join(task.file_name());

    // Check if file already exists to prevent overwrites
    if file_path.exists() {
        return Err(anyhow::anyhow!(
            "Task file already exists: {}. This should not happen with proper ID generation.",
            file_path.display()
        ));
    }

    let content = task.to_file_content()?;

    fs::write(&file_path, content)
        .with_context(|| format!("Failed to write task file: {}", file_path.display()))?;

    println!("✅ Created task: {}", file_path.strip_prefix(&tasks_dir).unwrap_or(&file_path).display());
    println!("   ID: {}", task.id);
    println!("   Title: {}", task.title);
    println!("   Area: {}", task.area);
    println!("   Priority: {}", task.priority);
    println!();
    println!("📝 Next steps:");
    println!("   taskguard show {}  # View task details", task.id);
    println!("   Edit the file to add more details");

    Ok(())
}

fn generate_task_id(area: &str, area_dir: &std::path::Path) -> Result<String> {
    // Find existing tasks in both active and archive directories
    // to prevent ID reuse when tasks are archived
    let active_max = scan_dir_for_max_id(area, area_dir)?;
    let archive_max = get_archive_max_id(area)?;

    let max_num = active_max.max(archive_max);
    let next_num = max_num + 1;
    Ok(format!("{}-{:03}", area, next_num))
}

/// Scan a directory for the highest task ID number
fn scan_dir_for_max_id(area: &str, dir: &std::path::Path) -> Result<u32> {
    let mut max_num = 0;

    if dir.exists() {
        for entry in fs::read_dir(dir).context("Failed to read directory")? {
            let entry = entry.context("Failed to read directory entry")?;
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(".md") {
                    // Extract number from filename like "backend-001.md"
                    let stem = file_name.trim_end_matches(".md");
                    if let Some(dash_pos) = stem.rfind('-') {
                        let area_part = &stem[..dash_pos];
                        let num_part = &stem[dash_pos + 1..];

                        if area_part == area {
                            if let Ok(num) = num_part.parse::<u32>() {
                                max_num = max_num.max(num);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(max_num)
}

/// Get the max task ID from the archive directory for an area
fn get_archive_max_id(area: &str) -> Result<u32> {
    use crate::config::find_taskguard_root;

    let root = match find_taskguard_root() {
        Some(r) => r,
        None => return Ok(0),
    };

    let archive_dir = root.join(".taskguard").join("archive").join(area);
    scan_dir_for_max_id(area, &archive_dir)
}