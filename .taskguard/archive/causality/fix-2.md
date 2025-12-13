---
id: fix-2
title: Clean Command - Add Dependency Protection
status: done
priority: medium
tags:
- causality
- fix
dependencies: []
assignee: developer
created: 2025-10-30T14:27:25.252545969Z
estimate: ~
complexity: 6
area: causality
---


**Location:** `src/commands/clean.rs`

**Implementation:**
```rust
pub fn run(dry_run: bool, _days: Option<u32>) -> Result<()> {
    let tasks_dir = get_tasks_dir()?;

    // Load ALL tasks (to check dependencies)
    let all_tasks = load_all_tasks_from_dir(&tasks_dir)?;

    // Find completed tasks that are SAFE to delete
    let mut files_to_delete = Vec::new();
    let mut protected_tasks = Vec::new();

    for entry in WalkDir::new(&tasks_dir)... {
        match Task::from_file(path) {
            Ok(task) => {
                if task.status == TaskStatus::Done {
                    // CRITICAL: Check if any active task depends on this
                    if is_task_referenced(&task.id, &all_tasks) {
                        protected_tasks.push((task.id.clone(), task.title.clone()));
                    } else {
                        files_to_delete.push(...);
                    }
                }
            }
            Err(_) => continue,
        }
    }

    // Show protected tasks
    if !protected_tasks.is_empty() {
        println!("🔒 PROTECTED TASKS (cannot delete - still referenced):");
        for (id, title) in &protected_tasks {
            println!("   🛡️  {} - {} (referenced by active tasks)", id, title);
        }
        println!();
        println!("💡 TIP: Use 'taskguard archive' instead to preserve history");
        println!();
    }

    // ... rest of clean logic
}
```

**Expected Behavior:**
- ✅ Delete only completed tasks with no active dependents
- ✅ Protect referenced tasks from deletion
- ✅ Suggest using `archive` instead
- ✅ Prevent permanent causality chain destruction

---


## Technical Notes
Location: `src/commands/clean.rs`