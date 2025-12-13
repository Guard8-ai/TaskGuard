---
id: fix-1
title: Archive Command - Add Dependency Protection
status: done
priority: medium
tags:
- causality
- fix
dependencies: []
assignee: developer
created: 2025-10-30T14:27:25.252467381Z
estimate: ~
complexity: 6
area: causality
---


**Location:** `src/commands/archive.rs`

**Implementation:**
```rust
pub fn run(dry_run: bool, _days: Option<u32>) -> Result<()> {
    let tasks_dir = get_tasks_dir()?;
    let root = find_taskguard_root()
        .context("Not in a TaskGuard project")?;
    let archive_dir = root.join(".taskguard").join("archive");

    // Load ALL tasks (to check dependencies)
    let all_tasks = load_all_tasks_from_dir(&tasks_dir)?;

    // Build map of which tasks are referenced
    let referenced_tasks = build_referenced_task_set(&all_tasks);

    // Find completed tasks that are SAFE to archive
    let mut files_to_archive = Vec::new();
    let mut blocked_from_archive = Vec::new();

    for entry in WalkDir::new(&tasks_dir)... {
        match Task::from_file(path) {
            Ok(task) => {
                if task.status == TaskStatus::Done {
                    // Check if any active task depends on this
                    if is_task_referenced(&task.id, &all_tasks) {
                        blocked_from_archive.push((task.id.clone(), task.title.clone()));
                    } else {
                        files_to_archive.push(...);
                    }
                }
            }
            Err(_) => continue,
        }
    }

    // Show blocked tasks
    if !blocked_from_archive.is_empty() {
        println!("🚫 BLOCKED FROM ARCHIVE (still referenced):");
        for (id, title) in &blocked_from_archive {
            println!("   ⚠️  {} - {} (referenced by active tasks)", id, title);
        }
        println!();
    }

    // ... rest of archive logic
}

fn is_task_referenced(task_id: &str, all_tasks: &[Task]) -> bool {
    for task in all_tasks {
        // Only check active tasks (not completed ones)
        if task.status != TaskStatus::Done {
            if task.dependencies.contains(&task_id.to_string()) {
                return true;  // Active task depends on this
            }
        }
    }
    false
}
```

**Expected Behavior:**
- ✅ Archive only completes tasks with no active dependents
- ✅ Block archiving of referenced tasks with clear message
- ✅ Preserve causality chain integrity
- ✅ User can see which tasks are blocked and why

---


## Technical Notes
Location: `src/commands/archive.rs`