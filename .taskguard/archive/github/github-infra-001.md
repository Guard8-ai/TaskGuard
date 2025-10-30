---
id: github-infra-001
title: Add Git Commit Tracking to Archive Command
status: done
priority: high
tags:
- github
- infrastructure
dependencies: []
assignee: developer
created: 2025-10-30T21:50:00Z
estimate: 2h
complexity: 4
area: github
---

# Add Git Commit Tracking to Archive Command

## Context
When tasks are archived, we need to create Git commits that reference the archived task IDs. This enables:
1. Git history analysis to track when tasks were completed
2. Future GitHub sync to understand task lifecycle
3. Commit-based activity tracking in `taskguard sync`

## Implementation

**Location:** `src/commands/archive.rs`

Added `create_archive_commit()` function that:
- Creates Git commits after archiving tasks
- Uses commit message format: `"Archive completed tasks: task-id-1, task-id-2, ..."`
- Stages both archive directory additions and task directory removals
- Provides clear error messages if Git operations fail

## Changes Made

1. Added imports:
   - `git2::Repository` for Git operations
   - `std::path::Path` for path handling

2. Modified archive workflow:
   - Collect archived task IDs during archiving
   - Call `create_archive_commit()` after successful archiving
   - Handle Git errors gracefully with warnings

3. Created `create_archive_commit()` helper:
   - Opens repository with path validation
   - Stages all archive changes (`.taskguard/archive/`)
   - Updates index for removed tasks
   - Creates commit with task ID references

## Testing

```bash
# Create test task
taskguard create --title "Test Archive Integration" --area testing --priority low

# Mark as done (edit tasks/testing/testing-003.md, set status: done)

# Archive the task
taskguard archive

# Verify commit created
git log -1 --pretty=format:"%s"
# Output: Archive completed tasks: testing-003

# Verify sync detects it
taskguard sync
# Shows testing-003 with recent activity
```

## Verification

✅ Archive command creates Git commits with task IDs
✅ Git sync (`taskguard sync`) detects archived tasks in commit history
✅ Commit messages follow format: "Archive completed tasks: task-id-list"
✅ Error handling: warnings on Git failures, but archiving still succeeds

## Session Handoff

### What Changed
- `src/commands/archive.rs`:
  - Added `create_archive_commit()` function (lines 174-232)
  - Modified `run()` to collect task IDs and create commits (lines 111, 122, 137-143)
  - Added `git2::Repository` and path imports

### Causality Impact
- **Archive → Git Commit**: Archiving now triggers Git commit creation
- **Git Commit → Sync Detection**: Commits with task IDs are detected by `taskguard sync`
- **Sync → Activity Tracking**: Sync command can now track when tasks were archived

### Runtime Behavior
- After archiving tasks, a Git commit is automatically created
- Commit message includes all archived task IDs in comma-separated format
- If Git operations fail, a warning is shown but archiving succeeds
- No user interaction required - fully automatic

### Dependencies Unblocked
This task unblocks:
- **github-infra-002**: GitHub module foundation (can now build on Git integration)
- Future GitHub sync can leverage commit messages for activity tracking

### Next Task Context
The next task (github-infra-002) will build the GitHub API client foundation. The commit messages we create here will be used by GitHub sync to:
1. Correlate local task changes with GitHub activity
2. Understand task lifecycle from Git history
3. Provide suggestions for GitHub issue status updates
