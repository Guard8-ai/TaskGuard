---
id: causality-004
title: Strengthen archive command reverse dependency check
status: done
priority: high
tags:
- causality
- v0.4.0
- archive
- protection
dependencies:
- causality-003
assignee: developer
created: 2025-12-21T12:00:00Z
estimate: 1h
complexity: 3
area: causality
---

# Strengthen archive command reverse dependency check

> **AI Agent Instructions:**
> 1. Read the existing archive.rs - it already has `is_task_referenced()`
> 2. Verify the logic is correct and covers all cases
> 3. Improve messaging to be clearer about WHY tasks are blocked
> 4. Add tests to verify protection works correctly

## Context

The archive command already has dependency protection via `is_task_referenced()` function (archive.rs:219-227). This task ensures the protection is robust and the messaging is clear for AI agents.

## Current Implementation (Verify)

```rust
/// Check if a task is referenced by any active (non-done) tasks
fn is_task_referenced(task_id: &str, all_tasks: &[Task]) -> bool {
    for task in all_tasks {
        // Only check active tasks (not completed ones)
        if task.status != TaskStatus::Done && task.dependencies.contains(&task_id.to_string()) {
            return true; // Active task depends on this
        }
    }
    false
}
```

## Requirements

### 1. Verify Protection Logic

The current logic checks:
- ✅ Only active (non-done) tasks can block archiving
- ✅ Checks if task ID is in dependencies list

Need to verify:
- [ ] Archived tasks don't block archiving (they shouldn't)
- [ ] Case sensitivity of task IDs
- [ ] Edge case: task depends on itself

### 2. Improve Blocked Message

Current output:
```
🚫 BLOCKED FROM ARCHIVE (still referenced by active tasks):
   ⚠️  setup-001 - Project Setup
```

Improved output:
```
🚫 BLOCKED FROM ARCHIVE (causality protection):
   ⚠️  setup-001 - Project Setup
       └── Depended on by: backend-001, api-001

   These tasks cannot be archived because active tasks depend on them.
   Complete the dependent tasks first, then archive.
```

### 3. Show Specific Dependents

Modify `is_task_referenced` to return WHO depends on the task:

```rust
/// Find active tasks that depend on the given task
fn find_dependents(task_id: &str, all_tasks: &[Task]) -> Vec<String> {
    all_tasks.iter()
        .filter(|t| t.status != TaskStatus::Done)
        .filter(|t| t.dependencies.contains(&task_id.to_string()))
        .map(|t| t.id.clone())
        .collect()
}
```

### 4. Exit Code

| Scenario | Exit Code |
|----------|-----------|
| Some tasks archived | 0 |
| All blocked, none archived | 0 (informational) |
| No completed tasks | 0 |

Note: Blocking is NOT an error - it's correct behavior.

## Test Cases

### Test 1: Active dependent blocks archive
```bash
# setup-001 is done, backend-001 is active and depends on setup-001
taskguard update status setup-001 done
taskguard archive
# Expected: setup-001 blocked from archive, shows backend-001 as dependent
```

### Test 2: Done dependent doesn't block archive
```bash
# Both setup-001 and backend-001 are done
taskguard update status setup-001 done
taskguard update status backend-001 done
taskguard archive
# Expected: Both can be archived (or just backend-001 if setup-001 has other deps)
```

### Test 3: Transitive dependencies
```bash
# A → B → C (A depends on B, B depends on C)
# C is done, B is done, A is active
taskguard archive
# Expected: C is blocked (B depends on it), B is blocked (A depends on it)
```

## Files to Modify

- [ ] `src/commands/archive.rs` - Improve messaging, show dependents
- [ ] `tests/cli_integration_tests.rs` - Add archive protection tests

## Acceptance Criteria

- [ ] Blocked tasks show WHO depends on them
- [ ] Messaging is clear about causality protection
- [ ] All existing tests pass
- [ ] New tests verify protection logic
- [ ] No false positives (tasks wrongly blocked)
- [ ] No false negatives (tasks archived when they shouldn't be)