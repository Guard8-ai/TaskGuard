---
id: github-fix-1
title: Update `load_all_tasks()` to Include Archive
status: done
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

## Implementation Summary

✅ **COMPLETED** - Unified task loading across all commands

### Changes Made

1. **Updated sync.rs** ([sync.rs:1-18](src/commands/sync.rs#L1-L18))
   - Replaced manual task loading with `config::load_all_tasks()`
   - Removed duplicate WalkDir logic
   - Now includes archived tasks in Git sync analysis

2. **Updated ai.rs** ([ai.rs:1-8](src/commands/ai.rs#L1-L8), [ai.rs:36-345](src/commands/ai.rs#L36-L345))
   - Removed local `load_all_tasks()` method
   - Updated all 4 handler methods to use `config::load_all_tasks()`
   - Now includes archived tasks in AI analysis

3. **Verification**
   - Build successful: ✅ `cargo build --release`
   - Sync command tested: ✅ Shows 27 tasks with activity (including archived)
   - Validate command confirmed: ✅ 57 total tasks (25 archived + 32 active)

### Technical Details

**Central Implementation:** `config::load_all_tasks()` at [config.rs:155-171](src/config.rs#L155-L171)
- Already loads from both `tasks/` and `.taskguard/archive/`
- Implemented in fix-5

**Commands Now Using Archive:**
- `taskguard sync` - Git analysis with archived tasks
- `taskguard ai` - AI assistant with archived task context
- `taskguard validate` - Dependency checking (already had this)

**Impact:**
- GitHub sync won't suggest orphaning archived tasks
- AI assistant has full project history context
- Consistent task loading across all commands

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