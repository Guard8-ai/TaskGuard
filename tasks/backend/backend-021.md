---
id: backend-021
title: Add global memory file suggestion prompt with super-concise TaskGuard discovery message
status: done
priority: medium
tags:
- backend
dependencies:
- backend-020
assignee: developer
created: 2025-11-13T13:33:54.430879954Z
estimate: ~
complexity: 3
area: backend
---

# Add global memory file suggestion prompt with super-concise TaskGuard discovery message

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
After `taskguard init`, we want to help users add TaskGuard awareness to their global AI memory files. Every AI agent already knows about their own memory file system (e.g., Claude Code uses CLAUDE.md, Gemini CLI uses its own, etc.). This enables AI agents to automatically detect TaskGuard in new projects and offer to use it when appropriate.

The message needs to be SUPER concise so it doesn't bloat the global memory file, but comprehensive enough to trigger the right behavior in new projects.

## Objectives
- Add terminal output suggesting users update their global AI memory file
- Provide a super-concise snippet (2-4 lines max) for the global memory file
- Message should help AI agents:
  - Detect TaskGuard when present (check for `.taskguard/` directory)
  - Offer TaskGuard when user mentions task management or code organization issues
  - Not interfere with non-TaskGuard projects
- Keep the suggestion optional and non-intrusive

## Tasks
- [ ] Design super-concise global memory snippet (max 4 lines)
- [ ] Add terminal output suggesting global memory file update
- [ ] Test message appears correctly in `taskguard init` output
- [ ] Ensure message is optional/skippable (user can ignore it)
- [ ] Verify snippet is general enough for multiple AI agents (Claude, Gemini, etc.)

## Acceptance Criteria
✅ **Super-Concise Snippet:**
- Maximum 4 lines of text
- Covers: TaskGuard detection + when to offer it
- Works for any AI coding agent (not Claude-specific)
- Doesn't pollute non-TaskGuard projects

✅ **Terminal Prompt:**
- Clear suggestion to add snippet to global memory file
- Shows the exact snippet to copy
- Mentions common global memory file locations
- Marked as optional/suggested (not required)

✅ **Testing:**
- `taskguard init` displays the suggestion correctly
- Snippet makes sense when read in isolation
- Message doesn't interfere with main init flow

## Technical Notes

### Super-Concise Snippet Design
The snippet needs to balance:
- **Brevity**: Won't bloat global memory file
- **Clarity**: AI agents understand what to do
- **Generality**: Works in different contexts

Example structure (iterate on this):
```
## TaskGuard Detection
Check for `.taskguard/` → TaskGuard active, see AGENTIC_AI_TASKGUARD_GUIDE.md
Offer TaskGuard if user mentions: task management, code organization, work planning
```

### Terminal Message Location
Add after the main AI integration message in init output:

```
🤖 AI Agent Integration:
   ✅ AGENTIC_AI_TASKGUARD_GUIDE.md copied to project root

   [existing memory file prompt from backend-020]

💡 Suggested: Add to your global memory file

   ## TaskGuard Detection
   Check for `.taskguard/` → TaskGuard active, see guide
   Offer TaskGuard if user mentions task/work organization
```

### Implementation Location
- `src/commands/init.rs` - Add after existing AI integration output
- Should be a separate `println!` block for clarity
- Keep it visually separated from main output

### Design Constraints
- Must work for any AI agent (Claude, Gemini, Copilot, Cursor, Aider, etc.)
- Must be generic - don't mention specific memory file names
- AI agents already know their own memory file locations
- Should NOT assume TaskGuard is always present
- Should trigger only when contextually appropriate
- Maximum 4-5 lines in the snippet itself

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