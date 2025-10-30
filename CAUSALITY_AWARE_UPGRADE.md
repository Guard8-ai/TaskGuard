# TaskGuard Causality-Aware System Upgrade

## Mission
Upgrade TaskGuard to preserve causality chains by default, solving vibe coding's temporal drift problem through enhanced task format—no new tools needed.

## Core Discovery
**The Problem**: LLMs skip steps, miss dependencies, and lose temporal context between sessions because task descriptions don't capture causality chains.

**The Solution**: Make causality preservation built into TaskGuard's task format by default, not an afterthought.

## Required Changes

### 1. Enhanced Task Template (Static File)

Update `taskguard create` to generate this causality-preserving template:

```markdown
---
id: area-001
title: "[Descriptive title: WHAT + WHY + scope/constraint]"
status: todo
priority: medium
area: backend
dependencies: []
created: 2025-01-15T10:00:00Z
---

# Task: [Descriptive title: WHAT + WHY + scope/constraint]

## Intent
What architectural/feature goal this serves and why it matters

## Pre-Implementation Exploration
**Before coding, AI agent must explore:**
- [ ] **Horizontal scan**: Review similar patterns, related features, parallel implementations
- [ ] **Vertical scan**: Trace dependency chain (UI → API → Service → DB)
- [ ] **Git context**: Check recent commits affecting related code (`git log --oneline -20`)
- [ ] **Complexity check**: Will this fit in one session context?
  - If NO → STOP, break into subtasks first
  - If causality unclear → STOP, document expected event chains first

## Implementation Context
**Files/functions this touches:**
- Existing code: [list files and entry points]
- Last related commits: [git log context]
- Current dependencies: [what exists, what's needed]

## Expected Causality Chain
**What should happen when this works:**
1. Event A triggers → Event B executes → Event C completes
2. [Be specific: function calls, state changes, async flows]
3. [Include failure modes: what breaks if X fails]

## Current State
- [x] Completed steps
- [ ] Blocked/in-progress steps (explain why blocked)
- [ ] Known issues/risks

## Task Dependencies
- **Blocks**: [task IDs that depend on this]
- **Blocked by**: [task IDs this depends on]
- **Related**: [contextually connected tasks]

## Complexity Assessment
- **Estimate**: [time estimate]
- **Complexity**: [1-10 scale]
- **Risk factors**: [what could go wrong]

## Session Notes
- **Created**: [session/date]
- **Last worked**: [session/date]
- **Next session should**: [what to load/check first]

## Session Handoff (AI: Complete this when marking task done)
**Causality preservation for next session:**
- What causal chains were created/modified
- What runtime behavior to expect
- What dependencies changed
- What the next task should know
- Verification steps: Does X still call Y? Are async chains valid?
```

### 2. Title Requirements

Enforce informative titles through validation:

**Bad Examples:**
- ❌ "Fix bug"
- ❌ "Update API"
- ❌ "Add feature"

**Good Examples:**
- ✅ "Fix file upload state sync - component unmounts before setState completes"
- ✅ "Implement JWT auth middleware - add RS256 token validation to protected routes"
- ✅ "Extract payment processor - isolate Stripe logic from checkout flow for testing"

**Pattern**: `[Action] [Component/Area] [Problem/Goal] - [Key constraint or context]`

### 3. New Command: `taskguard upgrade`

Migrate legacy tasks to causality-aware format:

```bash
taskguard upgrade [task-id] [--all] [--dry-run]
```

**Behavior:**
- Reads task in old format
- Adds missing causality sections (Intent, Pre-Implementation, Expected Causality, etc.)
- **AI agents may add their own notes during upgrade—this is encouraged**
- Preserves existing content
- Validates completeness
- Git commits the upgrade

**Options:**
- `[task-id]` - Upgrade specific task
- `--all` - Upgrade all legacy format tasks
- `--dry-run` - Show what would change without modifying files

### 4. Enhanced `taskguard validate`

Detect legacy format tasks:

```bash
$ taskguard validate

⚠️  Found 13 tasks in legacy format (missing causality sections)
Run: taskguard upgrade --all

🚦 TASK STATUS
   ✅ Available tasks: 5
   🚫 Blocked tasks: 8
   📋 Causality-aware tasks: 12
   📋 Legacy format tasks: 13

Recommendations:
- Upgrade legacy tasks to causality-aware format
- 3 tasks have unclear causality chains (document expected events)
```

### 5. Task Completion Behavior

When marking task done, AI agent should add to **Session Handoff** section:

```markdown
## Session Handoff
**Completed**: 2025-01-16 14:30

**Causality chains created:**
- User login → JWT token generation → Protected route access
- Failed login → Error response → UI error display

**Code changes:**
- `/src/auth/middleware.js` - Added JWT validation
- `/src/routes/protected.js` - Integrated auth middleware

**Expected runtime behavior:**
- Valid token in Authorization header → Request proceeds
- Invalid/missing token → 401 Unauthorized response
- Token expiration → 401 with "Token expired" message

**Dependencies affected:**
- Unblocked: `api-001` (User API endpoints can now use auth)
- Related: `testing-001` (needs to test auth flow)

**Next session should:**
- Verify token validation works in all protected routes
- Check async token refresh handling
- Test edge cases: expired tokens, malformed tokens, missing headers

**Known issues to watch:**
- Token expiration handling may need adjustment
- Refresh token strategy not yet implemented
```

## Agent Collaboration Design Principles

**Make TaskGuard feel like a collaborative partner:**

1. **Pre-flight guidance**: Show exploration checklist as helpful suggestions, not blockers
2. **Context overflow detection**: Suggest when to break tasks based on complexity
3. **Auto-generated summaries**: Generate session handoffs, don't force manual entry
4. **Validation feedback**: Report gaps and suggest improvements, don't block operations
5. **Encourage AI notes**: During upgrade, AI agents adding their own insights is valuable

**References** (context only, don't implement from these):
- https://www.anthropic.com/engineering/writing-tools-for-agents
- https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents
- https://www.anthropic.com/engineering/claude-code-best-practices

## Success Criteria

After upgrade:
- ✅ New tasks auto-include causality sections
- ✅ AI naturally fills them (sees value, not burden)
- ✅ Session handoffs preserve temporal context across sessions
- ✅ Large tasks get split before coding starts (caught in pre-implementation phase)
- ✅ Legacy tasks upgradeable with one command
- ✅ Task titles are informative and descriptive
- ✅ Horizontal/vertical exploration happens before implementation

## Implementation Files to Modify

1. **Task template** in `src/cli/commands/create.rs` - Update to new format
2. **New command** `src/cli/commands/upgrade.rs` - Implement format migration
3. **Enhanced validation** in `src/cli/commands/validate.rs` - Detect legacy format
4. **Documentation** in `AGENTIC_AI_TASKGUARD_GUIDE.md` - Add causality-aware section

## Design Constraints

- ✅ Minimal changes to existing commands
- ✅ Keep markdown format (git-trackable, human-readable)
- ✅ Make agents **want** to use TaskGuard (helps them succeed)
- ✅ Don't force unnatural workflows
- ✅ Pre-implementation analysis catches issues early
- ✅ Complexity checks prevent context overflow
- ✅ Session handoffs prevent temporal causality loss

## Why This Solves Vibe Coding Problems

**Temporal Causality Collapse**: Task format now preserves "what happened" → "what should happen next"

**Context Overflow**: Pre-implementation complexity checks catch this before coding starts

**Missing Dependencies**: Expected causality chain forces thinking through event sequences

**Session Drift**: Session handoff section creates explicit causality prompts for next session

**TaskGuard becomes the "attention pill"** that helps AI agents maintain focus and causality across sessions.
