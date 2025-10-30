---
id: issue-1
title: Archive Command - No Dependency Validation
status: todo
priority: medium
tags:
- causality
- issue
dependencies: []
assignee: developer
created: 2025-10-30T14:27:25.252321138Z
estimate: ~
complexity: 4
area: causality
---


**File:** `src/commands/archive.rs`

**Problem:**
```rust
// Current implementation archives ALL completed tasks
// WITHOUT checking if they're referenced by active tasks
if task.status == TaskStatus::Done {
    files_to_archive.push(...);  // No dependency check!
}
```

**Example Break:**
```yaml
# tasks/backend/backend-001.md (status: done)
id: backend-001
title: Setup authentication
status: done

# tasks/api/api-001.md (status: todo)
id: api-001
title: API endpoints
dependencies: [backend-001]  # ← References backend-001
status: todo
```

**After running `taskguard archive`:**
1. `backend-001.md` is moved to `.taskguard/archive/backend/`
2. `api-001` still contains `dependencies: [backend-001]`
3. `taskguard validate` reports: **"❌ api-001: Depends on missing task 'backend-001'"**
4. **Causality chain is broken** - the dependency appears "missing" even though it exists in archive

**Impact:** Active tasks lose their dependency links, breaking the entire causality tracking system.

---