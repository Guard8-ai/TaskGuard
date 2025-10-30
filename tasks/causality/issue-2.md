---
id: issue-2
title: Clean Command - Destroys Causality Chains
status: todo
priority: medium
tags:
- causality
- issue
dependencies: []
assignee: developer
created: 2025-10-30T14:27:25.252353825Z
estimate: ~
complexity: 4
area: causality
---


**File:** `src/commands/clean.rs`

**Problem:**
```rust
// Current implementation deletes ALL completed tasks
// WITHOUT checking if they're referenced
if task.status == TaskStatus::Done {
    fs::remove_file(&path)?;  // PERMANENT DELETION!
}
```

**Example Catastrophic Failure:**
```yaml
# tasks/setup/setup-001.md (status: done)
id: setup-001
title: Project initialization
status: done

# 15 other tasks depend on setup-001:
# backend-001, frontend-001, api-001, etc.
dependencies: [setup-001]
```

**After running `taskguard clean`:**
1. `setup-001.md` is **permanently deleted**
2. All 15 dependent tasks now have broken dependencies
3. **Historical causality data is lost forever**
4. No way to recover the causality chain

**Impact:**
- Irreversible data loss
- Complete causality chain corruption
- Impossible to understand task relationships
- Conflicts with CAUSALITY_AWARE_UPGRADE.md goals

---