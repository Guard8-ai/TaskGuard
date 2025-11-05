---
id: github-004
title: Auto-create missing status columns on GitHub Projects v2 board during sync
status: todo
priority: critical
tags:
- github
dependencies: []
assignee: developer
created: 2025-11-05T08:20:46.664667214Z
estimate: ~
complexity: 3
area: github
---

# Auto-create missing status columns on GitHub Projects v2 board during sync

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

TaskGuard supports 5 task statuses (todo, doing, review, done, blocked), but the GitHub sync currently only works with status columns that already exist on the Projects v2 board. Most GitHub projects start with only 3 columns (Backlog, In Progress, Done), causing warnings when syncing tasks with "review" or "blocked" status.

**Current Behavior:**
```
⚠️  No matching status column found for 'review'
```

**Expected Behavior:**
TaskGuard should automatically create missing status columns during sync initialization, providing zero-configuration GitHub integration.

## Objectives

- Implement automatic status column creation on GitHub Projects v2 boards
- Ensure all 5 TaskGuard statuses have corresponding columns
- Provide seamless, zero-configuration GitHub sync experience
- Maintain backward compatibility with existing boards

## Tasks

- [ ] Research GitHub GraphQL API for Projects v2 field option management
- [ ] Implement `ensure_status_columns()` function in `src/github/mutations.rs`
- [ ] Add mutation to create missing status field options (In Review, Blocked)
- [ ] Call initialization during sync in `src/commands/sync.rs`
- [ ] Add error handling for API failures (permissions, rate limits)
- [ ] Test with fresh Projects v2 board (only default columns)
- [ ] Test with existing board that has custom columns
- [ ] Verify columns are created in logical order on the board
- [ ] Update documentation to reflect automatic column creation

## Acceptance Criteria

✅ **Zero-Configuration Sync:**
- First sync automatically creates missing columns
- No manual GitHub board setup required
- All 5 statuses work immediately

✅ **Column Creation:**
- "In Review" column created when missing
- "Blocked" column created when missing
- Existing columns are preserved (no duplicates)
- Columns appear in logical workflow order

✅ **Error Handling:**
- Gracefully handles API permission errors
- Provides clear error messages if column creation fails
- Falls back to existing behavior if creation is not possible
- Continues sync even if column creation fails

✅ **Testing:**
- Unit tests for column detection logic
- Integration test with real GitHub API (mocked)
- Manual verification with actual Projects v2 board

## Technical Notes

**GitHub GraphQL API:**
- Mutation: `updateProjectV2` with field option additions
- Query existing field: `projectV2.field(name: "Status")`
- SingleSelectFieldOption type for status columns

**Implementation Location:**
```rust
// src/github/mutations.rs
impl GitHubMutations {
    pub fn ensure_status_columns(
        client: &GitHubClient,
        project_id: &str
    ) -> Result<()> {
        // 1. Get current status field options
        // 2. Identify missing TaskGuard statuses
        // 3. Create missing options via GraphQL
        // 4. Return success/failure
    }
}
```

**Required Status Columns:**
1. "Backlog" or "Todo" (for todo status)
2. "In Progress" (for doing status)
3. "In Review" (for review status) ← **CREATE IF MISSING**
4. "Blocked" (for blocked status) ← **CREATE IF MISSING**
5. "Done" (for done status)

**Integration Point:**
Call during sync initialization in `src/commands/sync.rs`:
```rust
pub fn run_github_sync(dry_run: bool) -> Result<()> {
    let client = GitHubClient::new()?;
    let project_id = get_project_id()?;

    // Ensure status columns exist (new)
    GitHubMutations::ensure_status_columns(&client, &project_id)?;

    // Continue with normal sync...
}
```

**Backward Compatibility:**
- Boards with custom column names still work (fuzzy matching)
- Adding columns doesn't break existing workflows
- Optional: Add config flag `auto_create_columns` (default: true)

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
- 2025-11-05: Task created

## Session Handoff (AI: Complete this when marking task done)
**For the next session/agent working on dependent tasks:**

### What Changed
- [Document code changes, new files, modified functions]
- [What runtime behavior is new or different]

### Causality Impact
- [What causal chains were created or modified]
- [What events trigger what other events]
- [Any async flows or timing considerations]

### Dependencies & Integration
- [What dependencies were added/changed]
- [How this integrates with existing code]
- [What other tasks/areas are affected]

### Verification & Testing
- [How to verify this works]
- [What to test when building on this]
- [Any known edge cases or limitations]

### Context for Next Task
- [What the next developer/AI should know]
- [Important decisions made and why]
- [Gotchas or non-obvious behavior]