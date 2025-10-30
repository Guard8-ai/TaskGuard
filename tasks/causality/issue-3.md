---
id: issue-3
title: List Command - Archive Blindness
status: todo
priority: medium
tags:
- causality
- issue
dependencies: []
assignee: developer
created: 2025-10-30T14:27:25.252371493Z
estimate: ~
complexity: 2
area: causality
---


**File:** `src/commands/list.rs`

**Problem:**
```rust
let task_files: Vec<_> = WalkDir::new(&tasks_dir)  // Only searches tasks/
    .into_iter()
    ...
```

**Missing:** No scanning of `.taskguard/archive/` directory

**Impact:**
- Archived dependencies are completely invisible to `taskguard list`
- Users cannot see that referenced tasks exist in archive
- Appears as if dependencies are missing when they're just archived

---