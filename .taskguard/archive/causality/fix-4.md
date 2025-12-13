---
id: fix-4
title: Update List Command to Show Archive
status: done
priority: medium
tags:
- causality
- fix
dependencies: []
assignee: developer
created: 2025-10-30T14:27:25.252651105Z
estimate: ~
complexity: 4
area: causality
---


**Location:** `src/commands/list.rs`

**Implementation:**
```rust
pub fn run(status_filter: Option<String>, area_filter: Option<String>) -> Result<()> {
    // Use the new helper function
    let all_tasks = load_all_tasks()?;

    // Mark which tasks are archived
    let tasks_dir = get_tasks_dir()?;
    let archive_dir = find_taskguard_root()?.join(".taskguard").join("archive");

    let mut tasks_with_location = Vec::new();
    for task in all_tasks {
        let is_archived = task.file_path.starts_with(&archive_dir);
        tasks_with_location.push((task, is_archived));
    }

    // ... filter logic ...

    // Display with archive indicator
    for (task, is_archived) in tasks_with_location {
        let archive_icon = if is_archived { "📦" } else { "" };
        println!("   {} {} {} {} {}",
            status_icon,
            priority_icon,
            archive_icon,  // Show archive indicator
            task.id,
            task.title
        );
    }
}
```

**Expected Behavior:**
- ✅ Show all tasks including archived ones
- ✅ Clear visual indicator for archived tasks (📦)
- ✅ Optional flag `--include-archived` / `--archived-only`
- ✅ Users can see full task landscape

---


## Technical Notes
Location: `src/commands/list.rs`