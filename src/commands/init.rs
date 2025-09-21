use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::config::Config;

pub fn run() -> Result<()> {
    let current_dir = std::env::current_dir().context("Failed to get current directory")?;

    // Check if already initialized
    let taskguard_dir = current_dir.join(".taskguard");
    if taskguard_dir.exists() {
        println!("❌ TaskGuard already initialized in this directory");
        return Ok(());
    }

    println!("🚀 Initializing TaskGuard...");

    // Create .taskguard directory structure
    fs::create_dir_all(&taskguard_dir)
        .context("Failed to create .taskguard directory")?;

    fs::create_dir_all(taskguard_dir.join("templates"))
        .context("Failed to create templates directory")?;

    fs::create_dir_all(taskguard_dir.join("state"))
        .context("Failed to create state directory")?;

    // Create tasks directory
    let tasks_dir = current_dir.join("tasks");
    fs::create_dir_all(&tasks_dir)
        .context("Failed to create tasks directory")?;

    // Create initial area directories
    let default_areas = ["setup", "backend", "frontend", "api", "auth", "testing"];
    for area in &default_areas {
        fs::create_dir_all(tasks_dir.join(area))
            .with_context(|| format!("Failed to create area directory: {}", area))?;
    }

    // Create default config
    let config = Config::default();
    let config_path = taskguard_dir.join("config.toml");
    config.save(&config_path)
        .context("Failed to save default config")?;

    // Create .gitignore if it doesn't exist
    let gitignore_path = current_dir.join(".gitignore");
    let gitignore_content = if gitignore_path.exists() {
        let existing = fs::read_to_string(&gitignore_path)
            .context("Failed to read existing .gitignore")?;

        if !existing.contains(".taskguard/state/") {
            format!("{}\n\n# TaskGuard\n.taskguard/state/\n", existing.trim())
        } else {
            existing
        }
    } else {
        "# TaskGuard\n.taskguard/state/\n".to_string()
    };

    fs::write(&gitignore_path, gitignore_content)
        .context("Failed to update .gitignore")?;

    // Create example task
    create_example_task(&tasks_dir)?;

    println!("✅ TaskGuard initialized successfully!");
    println!();
    println!("📁 Created directories:");
    println!("   .taskguard/         # Configuration and state");
    println!("   tasks/              # Task files organized by area");
    for area in &default_areas {
        println!("   tasks/{}/", area);
    }
    println!();
    println!("📋 Next steps:");
    println!("   taskguard list      # View all tasks");
    println!("   taskguard create    # Create a new task");
    println!("   taskguard validate  # Check task dependencies");

    Ok(())
}

fn create_example_task(tasks_dir: &Path) -> Result<()> {
    let example_content = r#"---
id: setup-001
title: "Project Setup and Dependencies"
status: todo
priority: high
tags: [setup, foundation]
dependencies: []
assignee: developer
created: 2025-01-15T10:00:00Z
estimate: 2h
complexity: 3
area: setup
---

# Project Setup and Dependencies

## Context
Initial project setup to establish the foundation for development.

## Objectives
- Set up development environment
- Install and configure necessary dependencies
- Establish basic project structure

## Tasks
- [ ] Initialize Git repository
- [ ] Set up package manager configuration
- [ ] Install core dependencies
- [ ] Configure development tools
- [ ] Create basic project structure
- [ ] Set up testing framework
- [ ] Create initial documentation

## Acceptance Criteria
✅ **Environment Ready:**
- All dependencies installed and working
- Development server can start successfully
- Tests can be executed

✅ **Documentation:**
- README.md with setup instructions
- Basic project structure documented

## Technical Notes
- Follow project conventions for structure
- Use latest stable versions of dependencies
- Ensure cross-platform compatibility

## Updates
- 2025-01-15: Task created as example
"#;

    let example_path = tasks_dir.join("setup").join("001-project-setup.md");
    fs::write(&example_path, example_content)
        .context("Failed to create example task")?;

    println!("📝 Created example task: tasks/setup/001-project-setup.md");

    Ok(())
}