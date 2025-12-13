---
id: backend-022
title: Prevent task ID reuse when archived tasks exist
status: done
priority: high
tags:
- backend
dependencies: []
assignee: developer
created: 2025-11-13T13:56:03.753461650Z
estimate: ~
complexity: 3
area: backend
---

# Prevent task ID reuse when archived tasks exist

> **⚠️ SESSION WORKFLOW NOTICE (for AI Agents):**
>
> **This task should be completed in ONE dedicated session.**
>
> When you mark this task as `done`, you MUST:
> 1. Fill the "Session Handoff" section at the bottom with complete implementation details
> 2. Document what was changed, what runtime behavior to expect, and what dependencies were affected
> 3. Create a clear handoff for the developer/next AI agent working on dependent tasks
>
> **If this task has dependents,** the next task will be handled in a NEW session and depends on your handoff for context.

## Context
When creating a new task with `taskguard create`, the system generates the next available ID by finding the highest existing ID number in the area and incrementing it. However, it doesn't check the `.taskguard/archive/` directory, which means archived task IDs can be reused.

This causes problems:
- Confusion about which backend-018 is which (active vs archived)
- Validation shows active tasks as archived (GitHub sync mapping confusion)
- Potential data conflicts if tasks are restored

**Example:** backend-018 was archived, then a new backend-018 was created. The system thinks the new one is archived.

## Objectives
- Modify `taskguard create` to check both active and archived tasks when generating new IDs
- Ensure task IDs are never reused, even after archiving
- Maintain a global ID counter per area that accounts for all tasks (active + archived)
- Keep performance reasonable (don't scan thousands of archived files every time)

## Tasks
- [ ] Locate the ID generation logic in src/commands/create.rs
- [ ] Add function to scan archived tasks for highest ID numbers
- [ ] Combine active + archived max IDs to find next available ID
- [ ] Add caching mechanism to avoid repeated archive scanning
- [ ] Test with existing archived tasks (backend-018 exists in archive)
- [ ] Verify new tasks get IDs that don't conflict with archive
- [ ] Document the ID generation strategy in code comments

## Acceptance Criteria
✅ **No ID Reuse:**
- Creating a new task never reuses an archived task's ID
- Works correctly even when archive has higher IDs than active tasks
- Example: If backend-018 is archived, next task should be backend-019 or higher

✅ **Performance:**
- ID generation remains fast (< 100ms even with large archives)
- Consider caching archive max IDs in `.taskguard/state/`
- Only rescan archive when necessary

✅ **Testing:**
- Create task in area with archived tasks → gets non-conflicting ID
- Archive a task, create new task → ID continues from archive
- Verify with backend area (has archived backend-018)

## Technical Notes

### Current ID Generation Location
Likely in `src/commands/create.rs` - find where it scans tasks/ directory to determine next ID.

### Proposed Solution
```rust
fn get_next_task_id(area: &str) -> Result<u32> {
    let active_max = scan_active_tasks(area)?;
    let archived_max = scan_archived_tasks(area)?;
    let max_id = active_max.max(archived_max);
    Ok(max_id + 1)
}
```

### Caching Strategy (Optional Enhancement)
Store max IDs per area in `.taskguard/state/max_ids.json`:
```json
{
  "backend": 22,
  "frontend": 5,
  "auth": 3
}
```

Update this file when:
- Creating new tasks
- Archiving tasks (if archived ID > current max)
- Restoring tasks (check if restored ID > current max)

### Files to Modify
- `src/commands/create.rs` - ID generation logic
- Possibly `src/task.rs` or `src/lib.rs` - shared ID utilities
- Consider adding to `src/commands/archive.rs` - update max IDs on archive

## Testing
- [ ] Write unit tests for new functionality
- [ ] Write integration tests if applicable
- [ ] Ensure all tests pass before marking task complete
- [ ] Consider edge cases and error conditions

## Version Control

**⚠️ CRITICAL: Always test AND run before committing!**

- [ ] **BEFORE committing**: Build, test, AND run the code to verify it works
  - Run `cargo build --release` (or `cargo build` for debug)
  - Run `cargo test` to ensure tests pass
  - **Actually run/execute the code** to verify runtime behavior
  - Fix all errors, warnings, and runtime issues
- [ ] Commit changes incrementally with clear messages
- [ ] Use descriptive commit messages that explain the "why"
- [ ] Consider creating a feature branch for complex changes
- [ ] Review changes before committing

**Testing requirements by change type:**
- Code changes: Build + test + **run the actual program/command** to verify behavior
- Bug fixes: Verify the bug is actually fixed by running the code, not just compiling
- New features: Test the feature works as intended by executing it
- Minor changes: At minimum build, check warnings, and run basic functionality

## Updates
- 2025-11-13: Task created
- 2025-12-08: Task completed

## Session Handoff
**For the next session/agent working on dependent tasks:**

### What Changed
- `src/commands/create.rs:224-275` - Refactored `generate_task_id()` to scan both active and archive directories
- Added `scan_dir_for_max_id()` helper function to extract max ID from a directory
- Added `get_archive_max_id()` to find highest ID in `.taskguard/archive/{area}/`
- `tests/end_to_end_tests.rs` - Added `create_task()` helper, updated all `create::run()` calls to use new 8-arg signature

### Causality Impact
- Task ID generation now considers archived tasks, preventing ID collisions
- No async flows affected - this is synchronous file scanning

### Dependencies & Integration
- Uses existing `find_taskguard_root()` from config module
- Integrates with archive command workflow - archived tasks now "reserve" their IDs

### Verification & Testing
```bash
# Test: Create task in area with only archived tasks
./target/release/taskguard create --title "Test" --area frontend
# Expected: frontend-002 (not 001, since frontend-001 is archived)

# All tests pass
cargo test  # 32 CLI tests, 12 e2e tests
```

### Context for Next Task
- ID generation scans both `tasks/{area}/` and `.taskguard/archive/{area}/`
- Returns max(active_max, archive_max) + 1
- If archive dir doesn't exist, returns 0 for archive_max
