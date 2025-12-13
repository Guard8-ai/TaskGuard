---
id: causality-upgrade-001
title: Enhanced Task Template - Add Causality-Preserving Sections
status: done
priority: high
tags:
- causality
- upgrade
- template
dependencies:
- fix-1
- fix-2
- fix-3
- fix-4
- fix-5
- github-fix-1
- github-fix-2
- github-fix-3
- github-fix-4
- github-fix-5
- github-fix-6
assignee: developer
created: 2025-10-30T15:00:00Z
estimate: 4h
complexity: 7
area: causality
---

# Enhanced Task Template - Add Causality-Preserving Sections

> **⚠️ CRITICAL WORKFLOW NOTICE:**
>
> **This task MUST be completed in ONE dedicated session.**
>
> When this task is marked `done`, the AI agent completing it MUST:
> 1. Fill the "Session Handoff" section below with complete implementation details
> 2. Document what was changed, what runtime behavior to expect, and what dependencies were affected
> 3. Create a clear handoff for the developer/next AI agent working on `causality-upgrade-002`
>
> **The next task (`causality-upgrade-002`) will be handled in a NEW session** and depends on this handoff for context.

## Intent
Upgrade TaskGuard's default task template to preserve causality chains by default, solving vibe coding's temporal drift problem. This makes causality preservation built into TaskGuard's task format, not an afterthought.

## Pre-Implementation Exploration
**Before coding, AI agent must explore:**
- [x] Review CAUSALITY_AWARE_UPGRADE.md specification
- [ ] **Horizontal scan**: Review existing task templates in codebase
- [ ] **Vertical scan**: Trace create command flow (CLI → Template → File Write)
- [ ] **Git context**: Check recent commits affecting create.rs (`git log --oneline -20 src/commands/create.rs`)
- [ ] **Complexity check**: Will this fit in one session context?
  - ✅ YES - Single file modification with template update
  - Causality is clear: User creates task → Template applied → File written

## Implementation Context
**Files/functions this touches:**
- Primary: `src/commands/create.rs` - Update default task template
- Related: Task template generation logic
- Last related commits: [Need to check git log]
- Current dependencies: Standard Rust file I/O

**Expected changes:**
- Add new sections to markdown template string
- Preserve existing YAML frontmatter structure
- Add causality-aware markdown sections

## Expected Causality Chain
**What should happen when this works:**
1. User runs `taskguard create` → Enhanced template applied → File written with causality sections
2. AI agent reads task → Sees pre-implementation checklist → Performs exploration before coding
3. AI completes task → Fills session handoff → Next session has causality context
4. Dependency chains preserved across sessions → Temporal drift prevented

**Failure modes:**
- If template syntax wrong → Task file parsing breaks → Validate command fails
- If sections unclear → AI agents ignore them → Temporal drift continues
- If too verbose → Cognitive overload → Reduced adoption

## Current State
- [ ] Read existing template in create.rs
- [ ] Design new template with causality sections
- [ ] Implement template update
- [ ] Test task creation with new template
- [ ] Verify YAML parsing still works
- [ ] Test with validate command

## Task Dependencies
- **Blocks**: causality-upgrade-002 (upgrade command needs new template format)
- **Blocked by**:
  - fix-1 through fix-5 (causality chain fixes must work first)
  - github-fix-1 through github-fix-6 (GitHub compatibility must work)
- **Related**: All causality-aware upgrade tasks

## Complexity Assessment
- **Estimate**: 4 hours
- **Complexity**: 7/10
- **Risk factors**:
  - Template too verbose → Reduced adoption
  - Markdown formatting breaks parsing
  - YAML frontmatter compatibility issues
  - Need to balance guidance vs. burden

## New Template Structure

```markdown
---
id: {area}-{number}
title: "{title} - [WHAT + WHY + constraint]"
status: todo
priority: {priority}
area: {area}
dependencies: []
created: {timestamp}
---

# Task: {title}

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
- [ ] Task breakdown
- [ ] Implementation steps
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
- **Created**: {timestamp}
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

## Title Requirements Enforcement

**Bad titles to detect:**
- ❌ "Fix bug"
- ❌ "Update API"
- ❌ "Add feature"

**Good title pattern:**
`[Action] [Component/Area] [Problem/Goal] - [Key constraint or context]`

**Examples:**
- ✅ "Fix file upload state sync - component unmounts before setState completes"
- ✅ "Implement JWT auth middleware - add RS256 token validation to protected routes"
- ✅ "Extract payment processor - isolate Stripe logic from checkout flow for testing"

## Implementation Steps

1. **Locate template in create.rs:**
   ```rust
   // Find the template string definition
   let template = format!("...");
   ```

2. **Update template with new sections:**
   - Keep YAML frontmatter intact
   - Add Intent section
   - Add Pre-Implementation Exploration checklist
   - Add Implementation Context section
   - Add Expected Causality Chain section
   - Add Session Handoff section

3. **Add title validation:**
   - Check title length (minimum informative)
   - Suggest better patterns if generic detected
   - Don't block creation, just warn

4. **Test template generation:**
   - Create test task
   - Verify markdown parsing
   - Verify YAML parsing
   - Check validate command works

## Session Notes
- **Created**: 2025-10-30
- **Next session should**: Read create.rs first, understand current template structure

## Session Handoff
To be filled when task is complete.