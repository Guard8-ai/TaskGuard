---
id: backend-006
title: Implement granular task item CLI management
status: done
priority: low
tags:
- backend
dependencies:
- backend-004
assignee: developer
created: 2025-09-28T07:07:54.718818542Z
estimate: ~
complexity: 3
area: backend
---

# Implement granular task item CLI management

## Context
Current TaskGuard requires manual markdown editing to update individual checklist items within tasks (e.g., changing `- [ ]` to `- [x]`). For agentic AI systems, this creates complexity when tracking granular progress within larger tasks. Need CLI commands to programmatically update individual task items.

## Objectives
- Enable CLI-based updates of individual checklist items within tasks
- Provide deterministic operations for agentic AI progress tracking
- Maintain markdown format compatibility while adding programmatic control
- Support item identification and status management

## Tasks
- [x] Design CLI command syntax: `taskguard update task <task-id> <item-index> <status>`
- [x] Parse markdown content to extract and identify checklist items
- [x] Implement item indexing system (1-based for user-friendliness)
- [x] Add markdown manipulation functions to update specific items
- [x] Create task item status validation (checked/unchecked)
- [x] Add support for `taskguard list items <task-id>` to show all items
- [x] Implement `taskguard update task <task-id> <item-index> done/todo`
- [x] Add error handling for invalid task IDs and item indexes
- [x] Preserve markdown formatting and spacing during updates
- [ ] Write comprehensive tests for markdown parsing and manipulation

## Acceptance Criteria
✅ **CLI Commands Work:**
- `taskguard list items backend-001` shows numbered checklist items
- `taskguard update task backend-001 3 done` marks 3rd item as `[x]`
- `taskguard update task backend-001 1 todo` marks 1st item as `[ ]`
- Commands handle edge cases (no items, invalid indexes)

✅ **Data Integrity:**
- Markdown formatting is preserved (indentation, spacing)
- Other content sections remain unchanged during item updates
- YAML frontmatter is unaffected by item updates
- Invalid operations return clear error messages

✅ **Agentic AI Compatibility:**
- Consistent exit codes for success/failure scenarios
- Machine-parseable output for list and update operations
- Deterministic item indexing (same order every time)

## Technical Notes
- Requires backend-004 (basic CLI updates) as prerequisite
- Parse markdown "## Tasks" section to extract `- [ ]` and `- [x]` items
- Use regex or markdown parser to identify and update specific items
- Consider supporting nested lists and different checkbox formats
- Item indexing should be 1-based for user-friendly CLI experience
- Example: `- [ ] Item 1` becomes index 1, `- [x] Item 2` becomes index 2

## Updates
- 2025-09-28: Task created