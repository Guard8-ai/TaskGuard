---
id: issue-4
title: Validate Command - Archive Blindness
status: done
priority: medium
tags:
- causality
- issue
dependencies: []
assignee: developer
created: 2025-10-30T14:27:25.252392283Z
estimate: ~
complexity: 4
area: causality
---


**File:** `src/commands/validate.rs`

**Problem:**
```rust
let task_files: Vec<_> = WalkDir::new(&tasks_dir)  // Only searches tasks/
    ...
let task_map: HashMap<String, &Task> = tasks.iter()
    .map(|t| (t.id.clone(), t))
    .collect();

for dep in &task.dependencies {
    if !all_ids.contains(dep) {  // Will fail for archived tasks!
        dependency_issues.push(format!(
            "❌ {}: Depends on missing task '{}'",
            task.id, dep
        ));
    }
}
```

**Missing:** No scanning of `.taskguard/archive/` directory

**Impact:**
- Archived tasks are falsely reported as "missing dependencies"
- False positive errors confuse users
- Cannot distinguish between truly missing tasks and archived tasks
- Validation becomes unreliable after using archive command

---

## Required Fixes