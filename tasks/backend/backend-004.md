---
id: backend-004
title: Implement CLI update commands for deterministic task management
status: done
priority: high
tags:
- backend
dependencies: []
assignee: developer
created: 2025-09-28T07:05:57.112277112Z
estimate: ~
complexity: 3
area: backend
---

# Implement CLI update commands for deterministic task management

## Context
Current TaskGuard requires manual file editing to update task status, priority, and other fields. This creates friction for agentic AI systems that need deterministic, programmatic operations. Need CLI commands that provide atomic updates without file editing.

## Objectives
- Replace manual editing with deterministic CLI commands
- Enable agentic AI to update task state programmatically
- Maintain data integrity with atomic file operations
- Preserve existing file format compatibility

## Tasks
- [ ] Add new CLI commands to main.rs Commands enum
- [ ] Implement `taskguard update status <task-id> <new-status>` command
- [ ] Implement `taskguard update priority <task-id> <priority>` command
- [ ] Implement `taskguard update assignee <task-id> <assignee>` command
- [ ] Implement `taskguard update dependencies <task-id> <dep1,dep2,dep3>` command
- [ ] Add status transition validation (todo → doing → done)
- [ ] Add atomic file update operations to Task struct
- [ ] Create new commands/update.rs module
- [ ] Add comprehensive error handling and validation
- [ ] Write unit tests for all update operations

## Acceptance Criteria
✅ **CLI Commands Work:**
- `taskguard update status api-001 done` succeeds and updates file
- `taskguard update priority setup-001 critical` works correctly
- Commands validate task IDs exist before updating

✅ **Data Integrity:**
- All updates are atomic (no partial writes)
- YAML frontmatter remains valid after updates
- Markdown content is preserved during updates
- Invalid status transitions are rejected

✅ **Agentic AI Compatibility:**
- Commands return consistent exit codes (0 success, 1 error)
- Error messages are machine-parseable
- Operations are idempotent (same result if run multiple times)

## Technical Notes
- Extend Commands enum in src/main.rs with Update variant
- Create src/commands/update.rs module following existing patterns
- Use Task::from_file() and save_to_file() for atomic operations
- Add validation methods to Task struct for status transitions
- Maintain backward compatibility with existing task files

## Updates
- 2025-09-28: Task created