---
id: bugs-001
title: Fix task ID incrementation bug in create command
status: done
priority: high
tags:
- bugs
dependencies: []
assignee: developer
created: 2025-09-24T19:55:40.359509183Z
estimate: ~
complexity: 3
area: bugs
---

# Fix task ID incrementation bug in create command

## Context
Critical bug where creating multiple tasks rapidly in the same area resulted in ID overwrites. Each new task would overwrite the previous one instead of generating sequential IDs (backend-001, backend-002, etc.).

## Objectives
- Fix ID generation logic to scan existing task files properly
- Ensure sequential ID generation works correctly
- Add validation to prevent file overwrites
- Test fix with multiple rapid task creations

## Tasks
- [x] Investigate ID generation logic in src/commands/create.rs
- [x] Identify root cause of ID overwriting issue
- [x] Fix the algorithm to scan existing task files and increment properly
- [x] Add validation to prevent file overwrites
- [x] Test by creating multiple tasks in same area rapidly

## Acceptance Criteria
✅ **Sequential ID Generation:**
- Multiple tasks in same area get unique, sequential IDs
- Algorithm correctly identifies highest existing number and increments
- Works across different areas independently

✅ **File Overwrite Prevention:**
- System detects if target file already exists
- Prevents data loss from overwriting existing tasks
- Clear error messages if conflicts occur

✅ **Cross-Area Compatibility:**
- New areas start with 001 correctly
- Mixed naming patterns are handled gracefully
- Ignores files that don't match area-NNN pattern

## Technical Notes
**Root Cause:** The `generate_task_id` function was parsing filenames incorrectly:
- Expected format: `001-something.md`
- Actual format: `backend-001.md`
- Fix: Parse from the end using `rfind('-')` and validate area prefix

**Changes Made:**
1. Fixed filename parsing in `src/commands/create.rs:115-145`
2. Added area prefix validation to ensure only matching files are counted
3. Added file existence check to prevent overwrites
4. Comprehensive testing with rapid task creation

## Updates
- 2025-09-24: Task created
- 2025-09-26: Bug investigated and root cause identified
- 2025-09-26: Fix implemented and thoroughly tested
- 2025-09-26: Task completed ✅
