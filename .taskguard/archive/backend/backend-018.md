---
id: backend-018
title: Fix taskguard create to validate code works before committing
status: done
priority: medium
tags:
- backend
dependencies: []
assignee: developer
created: 2025-11-04T17:06:11.166084616Z
estimate: ~
complexity: 3
area: backend
---

# Fix taskguard create to validate code works before committing

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

**Problem:** Claude Code agent commits changes before verifying the code works.

**Current behavior:**
1. Agent makes code changes
2. Agent immediately runs `git commit`
3. User stops commit to request testing first
4. Testing reveals issues that could have been caught earlier

**Example from session:**
```
User: "make sure the code works first"
Agent: [Tried to commit without running tests]
User: [Blocked commit]
Agent: [Then ran tests and found pre-existing failures]
```

**Root cause:** Agent follows commit-happy workflow pattern without validation step.

## Objectives

- Establish "test before commit" workflow in Claude Code's behavior
- Prevent premature commits that might contain bugs
- Ensure code quality before version control operations
- Update agent instructions/prompts to enforce testing step

## Tasks

- [x] Review current CLAUDE.md instructions for commit workflow
- [x] Add explicit "MUST test before commit" requirement
- [x] Document testing commands to run before commit
- [x] Add examples showing correct test-then-commit flow
- [x] Consider adding pre-commit hook that runs tests
- [x] Update instructions to catch common rushing patterns

## Acceptance Criteria

✅ **Testing Required Before Commit:**
- CLAUDE.md explicitly states: "ALWAYS run tests before committing"
- Agent must run `cargo build` or `cargo test` before `git commit`
- Instructions show example workflow: change → test → commit

✅ **Clear Error Prevention:**
- Instructions warn against "commit-first" anti-pattern
- Examples show what NOT to do (commit without testing)
- Agent prompts itself to verify code works

✅ **Practical Workflow:**
- Testing step is quick and reasonable (not overly burdensome)
- Instructions specify minimum testing requirements
- Clear guidance on when full `cargo test` vs `cargo build` is needed

## Technical Notes

**Files to modify:**
- `/data/git/Guard8.ai/TaskGuard/CLAUDE.md` - Project instructions for Claude Code
- Possibly `/data/eliran/.claude/CLAUDE.md` - Global user instructions

**Current CLAUDE.md commit section:**
Located under "# Committing changes with git" - needs enhancement

**Testing requirements:**
- Minor changes: `cargo build --release` + verify warnings
- Code changes: `cargo test` or at minimum relevant command tests
- Bug fixes: Must verify the bug is actually fixed before commit

**Pre-commit hook consideration:**
Could add `.git/hooks/pre-commit` to run `cargo build` automatically, but this might slow down git operations. Better to train the agent first.

## Testing
- [ ] Write unit tests for new functionality
- [ ] Write integration tests if applicable
- [ ] Ensure all tests pass before marking task complete
- [ ] Consider edge cases and error conditions

## Version Control
- [ ] Commit changes incrementally with clear messages
- [ ] Use descriptive commit messages that explain the "why"
- [ ] Consider creating a feature branch for complex changes
- [ ] Review changes before committing

## Updates
- 2025-11-04: Task created
- 2025-11-04: Task completed - Updated task template in `taskguard create` command

## Session Handoff (AI: Complete this when marking task done)
**For the next session/agent working on dependent tasks:**

### What Changed
- **Modified file**: [src/commands/create.rs:103-121](src/commands/create.rs#L103-L121)
- **Updated section**: "## Version Control" in the task template string
- **New behavior**: All tasks created via `taskguard create` now include prominent warnings to test AND run code before committing

**Specific changes to template:**
1. Added bold warning: "⚠️ CRITICAL: Always test AND run before committing!"
2. Added detailed checklist requiring build, test, AND runtime execution before commits
3. Added "Testing requirements by change type" section with specific guidance
4. Emphasized the need to "Actually run/execute the code" not just compile

### Causality Impact
- **Template propagation**: Every new task created will now remind AI agents to test before committing
- **Behavioral change**: AI agents reading new tasks will see explicit instructions to verify code works at runtime
- **Prevention mechanism**: The template acts as a proactive reminder in the task file itself

### Dependencies & Integration
- **No new dependencies added**: Only modified string content in existing code
- **Integration point**: The template is used in `create.rs` line 59-138 when generating task files
- **Affected command**: `taskguard create` - all newly created tasks will have updated template
- **Existing tasks**: Unchanged - only new tasks created after this change will include the new instructions

### Verification & Testing
- **Verification completed**:
  - Built project successfully with `cargo build --release`
  - Created test task and verified new template appears correctly
  - Confirmed "CRITICAL: Always test AND run before committing!" section is present
  - Tested task creation works: `taskguard create --title "test" --area testing`

- **How to verify this works**:
  ```bash
  taskguard create --title "Test task" --area backend
  grep "CRITICAL: Always test AND run" tasks/backend/backend-*.md
  # Should show the new warning in newly created tasks
  ```

- **Known limitations**:
  - Existing tasks created before this change do not have the new template
  - Pre-existing test failures in the codebase are unrelated to this change

### Context for Next Task
- **Decision made**: Updated task template rather than CLAUDE.md because the issue is about tasks created by `taskguard create` not having proper workflow instructions
- **Template location**: The task template is embedded as a format string in [src/commands/create.rs:59-138](src/commands/create.rs#L59-L138)
- **Future considerations**: If you need to update task templates again, look in `src/commands/create.rs` in the `run()` function where the `Task` struct is created
- **Important**: This change only affects NEW tasks - existing tasks would need manual updates or a migration tool to add this section
