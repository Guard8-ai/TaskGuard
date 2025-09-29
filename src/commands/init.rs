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

    // Copy AI agent integration guide to project root
    copy_ai_guide(&current_dir)?;

    // Create AI agent notification file
    create_ai_notification(&current_dir)?;

    println!("✅ TaskGuard initialized successfully!");
    println!();
    println!("📁 Created directories:");
    println!("   .taskguard/         # Configuration and state");
    println!("   tasks/              # Task files organized by area");
    for area in &default_areas {
        println!("   tasks/{}/", area);
    }
    println!();
    println!("🤖 AI Agent Integration:");
    println!("   AGENTIC_AI_TASKGUARD_GUIDE.md    # AI collaboration guide copied to root");
    println!("   AI_AGENT_SETUP_NOTIFICATION.md  # Instructions for updating tool configs");
    println!();
    println!("📋 Next steps:");
    println!("   taskguard list      # View all tasks");
    println!("   taskguard create    # Create a new task");
    println!("   taskguard validate  # Check task dependencies");
    println!();
    println!("🚨 IMPORTANT for AI Agents:");
    println!("   Read AI_AGENT_SETUP_NOTIFICATION.md and update your tool instruction files!");

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

fn copy_ai_guide(project_dir: &Path) -> Result<()> {
    // This would normally embed the guide content or copy from installation
    // For now, we'll create the guide content directly
    let guide_content = include_str!("../../AGENTIC_AI_TASKGUARD_GUIDE.md");

    let guide_path = project_dir.join("AGENTIC_AI_TASKGUARD_GUIDE.md");
    fs::write(&guide_path, guide_content)
        .context("Failed to copy AI integration guide")?;

    Ok(())
}

fn create_ai_notification(project_dir: &Path) -> Result<()> {
    let notification_content = r#"# 🚨 AI Agent Setup Required

**TaskGuard has been initialized with AI agent integration support.**

## Required Actions for AI Agents

### 1. Update Your Tool Instruction Files

Add the following to your AI tool instruction files:

#### For Claude Code (CLAUDE.md):
```markdown
## TaskGuard Integration

This project uses TaskGuard for AI-ready task management.

**Essential Reference**: See `AGENTIC_AI_TASKGUARD_GUIDE.md` for complete AI collaboration guide.

### Quick Commands
- `taskguard init` - Initialize TaskGuard
- `taskguard create --title "Task" --area backend --priority high` - Create tasks
- `taskguard list` - View all tasks
- `taskguard validate` - Check dependencies
- `taskguard update status <task-id> doing` - Update status

### Workflow
1. Use CLI commands for ALL task metadata updates
2. Create tasks across different areas (setup, backend, api, frontend, etc.)
3. Set dependencies with `taskguard update dependencies <task-id> "dep1,dep2"`
4. Validate frequently with `taskguard validate`
```

#### For Other AI Tools (GEMINI.md, etc.):
```markdown
## TaskGuard Integration

**Reference**: `AGENTIC_AI_TASKGUARD_GUIDE.md`

Use TaskGuard CLI commands for deterministic task management:
- Create: `taskguard create --title "Task" --area <area> --priority <priority>`
- Update: `taskguard update status <task-id> <status>`
- Dependencies: `taskguard update dependencies <task-id> "dep1,dep2"`
- Validate: `taskguard validate`
```

### 2. Key Integration Points

- **CLI-First**: Always use `taskguard update` commands instead of manual file editing
- **Area Distribution**: Spread tasks across multiple areas to avoid ID conflicts
- **Validation**: Run `taskguard validate` frequently to check dependencies
- **Granular Updates**: Use `taskguard task update <task-id> <item> done` for checklist items

### 3. Success Metrics

Your TaskGuard integration is working correctly when:
- ✅ Tasks are distributed across multiple areas
- ✅ Dependencies form clear chains
- ✅ `taskguard validate` shows no errors
- ✅ All metadata updates use CLI commands
- ✅ No template content remains in tasks

## Next Steps

1. **Read** `AGENTIC_AI_TASKGUARD_GUIDE.md` thoroughly
2. **Update** your tool instruction files with TaskGuard integration
3. **Test** the workflow with a few sample tasks
4. **Validate** your setup with `taskguard validate`

---

**This file can be deleted once you've completed the AI agent integration setup.**
"#;

    let notification_path = project_dir.join("AI_AGENT_SETUP_NOTIFICATION.md");
    fs::write(&notification_path, notification_content)
        .context("Failed to create AI agent notification file")?;

    Ok(())
}