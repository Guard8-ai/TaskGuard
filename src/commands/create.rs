use anyhow::{Context, Result};
use chrono::Utc;
use std::fs;

use crate::config::{get_tasks_dir, get_config_path, Config};
use crate::task::{Task, TaskStatus, Priority};

pub fn run(title: String, area: Option<String>, priority: Option<String>) -> Result<()> {
    let tasks_dir = get_tasks_dir()?;
    let config_path = get_config_path()?;
    let config = Config::load_or_default(&config_path)?;

    // Determine area
    let area = area.unwrap_or_else(|| {
        if config.project.areas.contains(&"setup".to_string()) {
            "setup".to_string()
        } else {
            config.project.areas.first().cloned().unwrap_or_else(|| "general".to_string())
        }
    });

    // Validate area
    if !config.project.areas.contains(&area) {
        println!("⚠️  Warning: Area '{}' is not in configured areas: {:?}", area, config.project.areas);
        println!("   Continuing anyway...");
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

    // Generate task ID
    let area_dir = tasks_dir.join(&area);
    let task_id = generate_task_id(&area, &area_dir)?;

    // Create task
    let task = Task {
        id: task_id.clone(),
        title: title.clone(),
        status: TaskStatus::Todo,
        priority,
        tags: vec![area.clone()],
        dependencies: Vec::new(),
        assignee: Some("developer".to_string()),
        created: Utc::now(),
        estimate: None,
        complexity: Some(3), // Default complexity
        area: area.clone(),
        content: format!(r#"# {}

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

## Updates
- {}: Task created
"#, title, Utc::now().format("%Y-%m-%d")),
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
    // Find existing tasks in the area to determine next number
    let mut max_num = 0;

    if area_dir.exists() {
        for entry in fs::read_dir(area_dir).context("Failed to read area directory")? {
            let entry = entry.context("Failed to read directory entry")?;
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(".md") {
                    // Extract number from filename like "backend-001.md" or "area-123.md"
                    // Expected format: area-NNN.md
                    let stem = file_name.trim_end_matches(".md");
                    if let Some(dash_pos) = stem.rfind('-') {
                        let area_part = &stem[..dash_pos];
                        let num_part = &stem[dash_pos + 1..];

                        // Only consider files that match our area prefix
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

    let next_num = max_num + 1;
    Ok(format!("{}-{:03}", area, next_num))
}