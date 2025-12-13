---
id: fix-3
title: Create Task Loading Helper
status: done
priority: medium
tags:
- causality
- fix
dependencies: []
assignee: developer
created: 2025-10-30T14:27:25.252601145Z
estimate: ~
complexity: 6
area: causality
---


**Location:** `src/task.rs` or `src/config.rs`

**Implementation:**
```rust
/// Load tasks from both active and archive directories
pub fn load_all_tasks() -> Result<Vec<Task>> {
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

fn load_tasks_from_dir(dir: &Path) -> Result<Vec<Task>> {
    let mut tasks = Vec::new();

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
    {
        match Task::from_file(entry.path()) {
            Ok(task) => tasks.push(task),
            Err(_) => continue,
        }
    }

    Ok(tasks)
}
```

**Expected Behavior:**
- ✅ Single helper function for loading all tasks
- ✅ Searches both `tasks/` and `.taskguard/archive/`
- ✅ Gracefully handles missing directories
- ✅ Reusable across all commands

---


## Technical Notes
Location: `src/task.rs` or `src/config.rs`