---
id: github-fix-1
title: Update `load_all_tasks()` to Include Archive
status: todo
priority: medium
tags:
- github
- fix
dependencies: []
assignee: developer
created: 2025-10-30T14:22:23.822050724Z
estimate: ~
complexity: 6
area: github
---


**Location:** `src/commands/sync.rs` or create `src/task_loader.rs`

```rust
/// Load tasks from both active and archive directories
pub fn load_all_tasks_including_archive() -> Result<Vec<Task>> {
    let mut tasks = Vec::new();

    // Load from tasks/
    let tasks_dir = get_tasks_dir()?;
    if tasks_dir.exists() {
        tasks.extend(load_tasks_from_dir(&tasks_dir)?);
    }

    // Load from .taskguard/archive/
    let root = find_taskguard_root()
        .context("Not in a TaskGuard project")?;
    let archive_dir = root.join(".taskguard").join("archive");
    if archive_dir.exists() {
        tasks.extend(load_tasks_from_dir(&archive_dir)?);
    }

    Ok(tasks)
}

/// Extended task metadata including location
pub struct TaskWithLocation {
    pub task: Task,
    pub is_archived: bool,
    pub file_path: PathBuf,
}

pub fn load_all_tasks_with_metadata() -> Result<Vec<TaskWithLocation>> {
    let tasks_dir = get_tasks_dir()?;
    let archive_dir = find_taskguard_root()?.join(".taskguard").join("archive");

    let mut all_tasks = Vec::new();

    // Load active tasks
    for task in load_tasks_from_dir(&tasks_dir)? {
        all_tasks.push(TaskWithLocation {
            task,
            is_archived: false,
            file_path: /* ... */,
        });
    }

    // Load archived tasks
    if archive_dir.exists() {
        for task in load_tasks_from_dir(&archive_dir)? {
            all_tasks.push(TaskWithLocation {
                task,
                is_archived: true,
                file_path: /* ... */,
            });
        }
    }

    Ok(all_tasks)
}
```


## Technical Notes
Location: `src/commands/sync.rs` or create `src/task_loader.rs`