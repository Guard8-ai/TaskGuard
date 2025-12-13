---
id: github-005
title: 'GitHub Hub: Cross-branch task sync via TaskGuard ID recognition'
status: done
priority: high
tags:
- github
- sync
- cross-branch
dependencies: []
assignee: developer
created: 2025-12-09T08:04:49.835020228Z
estimate: ~
complexity: 6
area: github
---

# GitHub Hub: Cross-branch task sync via TaskGuard ID recognition

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
The `github-mapping.json` is tracked by git, so branches diverge with different mappings. This creates a problem:

```
Branch A: creates backend-042 → sync → Issue #100 created
Branch B: creates backend-042 → sync → NO local mapping → Issue #101 created (DUPLICATE!)
```

GitHub ends up with duplicate issues for the same TaskGuard ID. The local mapping isn't enough - **GitHub must be the source of truth**.

## Objectives
- Make GitHub the Hub: query GitHub before creating issues to detect duplicates
- Track branch origin: issue body must include which branch created the task
- Enable cross-branch conflict detection and resolution

## Tasks
- [ ] Modify issue body format to include source branch name
- [ ] Before creating issue, search GitHub for existing `**TaskGuard ID:** {id}`
- [ ] If found with matching title: report as same task from different branch, offer adopt
- [ ] If found with different title: report as TRUE ID COLLISION, warn strongly, suggest rename
- [ ] If not found: create issue with branch info in body
- [ ] Optionally: adopt existing issue into local mapping (add to mapping without creating)

## Acceptance Criteria
✅ **Issue body includes branch + hash:**
```markdown
**TaskGuard ID:** backend-042
**Source Branch:** feature/user-auth
**Hash:** a1b2c3d4

## Description
...
```

✅ **Duplicate prevention:**
```bash
# Branch A syncs first
taskguard sync --github
# → Creates Issue #100 for backend-042 (branch: feature/A)

# Branch B tries to sync same ID
taskguard sync --github
# → "⚠️ Task backend-042 already synced from branch 'feature/A' (Issue #100)"
# → Skip creation, optionally adopt into local mapping
```

✅ **Conflict visibility:**
- User sees which branch owns the task
- Can decide: adopt existing, rename local task, or ignore

✅ **True ID collision (different tasks, same ID):**
```bash
# Branch A: backend-042 = "func foo fix"
# Branch B: backend-042 = "func bar fix" (DIFFERENT task!)

taskguard sync --github
# → "⚠️ Task ID CONFLICT: backend-042"
#    Local:  "func bar fix" (branch: feature/B)
#    GitHub: "func foo fix" (branch: feature/A, Issue #100)
#
#    ❌ Title mismatch - these are DIFFERENT tasks!
#    Options:
#      1. Rename local task to avoid conflict
#      2. Skip sync for this task
#      3. Force adopt (not recommended)
```
- Collision detection via file hash:
  - Same ID + same hash → same task, safe to adopt
  - Same ID + different hash → TRUE COLLISION, must rename

## Technical Notes
- Get current branch: `git rev-parse --abbrev-ref HEAD`
- Search GitHub issues: query for `**TaskGuard ID:** {id}` in body
- Parse branch from existing issue body with regex
- Current body format (sync.rs:616-618):
  ```rust
  let body = format!(
      "**TaskGuard ID:** {}\n\n## Description\n\n{}\n\n---\n*Synced from TaskGuard*",
      task.id, description
  );
  ```
- New format:
  ```rust
  let hash = hash_task_file(&task);  // short hash of .md content
  let body = format!(
      "**TaskGuard ID:** {}\n**Source Branch:** {}\n**Hash:** {}\n\n## Description\n\n{}\n\n---\n*Synced from TaskGuard*",
      task.id, branch_name, hash, description
  );
  ```

## Testing
- [ ] Unit test: branch name extraction from git
- [ ] Unit test: task file hashing
- [ ] Unit test: TaskGuard ID search in GitHub issues
- [ ] Integration test: same task detection (hash match)
- [ ] Integration test: true ID collision (hash mismatch)
- [ ] Edge case: detached HEAD, no branch name

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
- 2025-12-09: Task created

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