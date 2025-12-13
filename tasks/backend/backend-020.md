---
id: backend-020
title: Remove AI_AGENT_SETUP_NOTIFICATION.md creation and add memory file update prompt
status: done
priority: high
tags:
- backend
dependencies: []
assignee: developer
created: 2025-11-13T13:33:39.795074080Z
estimate: ~
complexity: 3
area: backend
---

# Remove AI_AGENT_SETUP_NOTIFICATION.md creation and add memory file update prompt

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
`AI_AGENT_SETUP_NOTIFICATION.md` creates a temporary file that gets deleted after first read. This is inefficient and creates clutter. Instead, we should print a clear message to the terminal that prompts AI agents to update their memory files (like CLAUDE.md or equivalent) with TaskGuard integration details.

## Objectives
- Remove `create_ai_notification()` function and its call from init.rs
- Remove all references to `AI_AGENT_SETUP_NOTIFICATION.md` from the codebase
- Add new terminal output that prompts AI agents to update memory files
- Message should guide both local project memory files AND suggest global memory files
- Keep the output concise and actionable

## Tasks
- [ ] Remove `create_ai_notification()` function from src/commands/init.rs (lines 172-251)
- [ ] Remove call to `create_ai_notification(&current_dir)?;` (line 72)
- [ ] Remove `AI_AGENT_SETUP_NOTIFICATION.md` references from terminal output (lines 85, 93)
- [ ] Add new terminal output with memory file update prompt
- [ ] Update CLAUDE.md to remove AI_AGENT_SETUP_NOTIFICATION.md reference (line 660)
- [ ] Update README.md to remove AI_AGENT_SETUP_NOTIFICATION.md reference if present
- [ ] Search docs/ for any other references and clean them up

## Acceptance Criteria
✅ **No Notification File:**
- `create_ai_notification()` function completely removed
- `taskguard init` does NOT create `AI_AGENT_SETUP_NOTIFICATION.md`
- No errors or warnings about missing notification creation

✅ **AI Memory Prompt:**
- Clear terminal message prompts AI agents to update memory files
- Message distinguishes between project-level and global memory files
- Message is concise but actionable (max 10-15 lines)
- Includes brief example or guidance for memory file integration

✅ **Clean Codebase:**
- No references to `AI_AGENT_SETUP_NOTIFICATION.md` anywhere
- `cargo build --release` succeeds with no warnings
- `taskguard init` in test directory shows new messaging correctly

## Technical Notes

### Suggested Terminal Message Format
```
🤖 AI Agent Integration:
   ✅ AGENTIC_AI_TASKGUARD_GUIDE.md copied to project root

   📝 AI Agents: Update your memory file for TaskGuard workflow

   Project memory:
   Add reference: "TaskGuard active. See AGENTIC_AI_TASKGUARD_GUIDE.md"

   💡 Optional - Global memory:
   Consider adding TaskGuard detection for all your projects
```

### Files to Modify
- `src/commands/init.rs` - Remove function, update output
- `CLAUDE.md` - Remove notification file reference
- `README.md` - Remove notification file reference if present
- `docs/` - Clean up any references

### Implementation Approach
1. Remove notification function and its call
2. Test that init still works without it
3. Add new print statements for memory file prompt
4. Clean up all documentation references
5. Rebuild and test in clean directory

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