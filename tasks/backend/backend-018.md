---
id: backend-018
title: Fix taskguard create to validate code works before committing
status: todo
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

- [ ] Review current CLAUDE.md instructions for commit workflow
- [ ] Add explicit "MUST test before commit" requirement
- [ ] Document testing commands to run before commit
- [ ] Add examples showing correct test-then-commit flow
- [ ] Consider adding pre-commit hook that runs tests
- [ ] Update instructions to catch common rushing patterns

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
