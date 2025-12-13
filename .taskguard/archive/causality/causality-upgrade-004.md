---
id: causality-upgrade-004
title: Update Documentation - Add Causality-Aware System Guide to AGENTIC_AI_TASKGUARD_GUIDE.md
status: done
priority: medium
tags:
- causality
- upgrade
- documentation
dependencies:
- causality-upgrade-001
- causality-upgrade-002
- causality-upgrade-003
assignee: developer
created: 2025-10-30T15:00:00Z
estimate: 2h
complexity: 4
area: causality
---

# Update Documentation - Add Causality-Aware System Guide

> **⚠️ CRITICAL WORKFLOW NOTICE:**
>
> **This task MUST be completed in ONE dedicated session.**
>
> When this task is marked `done`, the AI agent completing it MUST:
> 1. Fill the "Session Handoff" section below with complete implementation details
> 2. Document what was changed, what runtime behavior to expect, and what dependencies were affected
> 3. Create a clear handoff for the developer/next AI agent working on `causality-upgrade-005`
>
> **The next task (`causality-upgrade-005`) will be handled in a NEW session** and depends on this handoff for context.

## Intent
Document the causality-aware task system in AGENTIC_AI_TASKGUARD_GUIDE.md so AI agents understand how to use the enhanced format effectively. This enables AI agents to naturally fill in causality sections and preserve temporal context across sessions.

## Pre-Implementation Exploration
**Before coding, AI agent must explore:**
- [ ] **Horizontal scan**: Review AGENTIC_AI_TASKGUARD_GUIDE.md structure and style
- [ ] **Vertical scan**: Understand documentation flow (Installation → Basics → Advanced)
- [ ] **Git context**: Check recent documentation updates
- [ ] **Complexity check**: Will this fit in one session context?
  - ✅ YES - Single documentation file update
  - Causality: Read current docs → Add new section → Examples → Best practices

## Implementation Context
**Files/functions this touches:**
- Primary: `AGENTIC_AI_TASKGUARD_GUIDE.md` - Add causality section
- Reference: CAUSALITY_AWARE_UPGRADE.md for specification
- Style: Match existing guide tone and format

**Expected changes:**
- Add "Causality-Aware Task System" section
- Document new template sections
- Provide examples of good causality documentation
- Add upgrade workflow instructions
- Include best practices for AI agents

## Expected Causality Chain
**What should happen when this works:**

1. AI agent reads AGENTIC_AI_TASKGUARD_GUIDE.md → Understands causality system
2. AI creates new task → Uses causality template naturally
3. AI works on task → Fills in causality sections during work
4. AI completes task → Documents session handoff
5. Next AI session → Reads handoff → Has temporal context
6. Temporal drift prevented → Better code quality

**Failure modes:**
- If documentation unclear → AI agents ignore sections → Temporal drift continues
- If examples poor → AI agents copy bad patterns
- If too technical → Reduces adoption

## Current State
- [ ] Read current AGENTIC_AI_TASKGUARD_GUIDE.md
- [ ] Identify insertion point for causality section
- [ ] Write causality-aware system section
- [ ] Add examples and best practices
- [ ] Add upgrade workflow
- [ ] Review for clarity and completeness

## Documentation Structure

### New Section: "Causality-Aware Task System"

**Placement:** After "Task Management Basics" section

**Contents:**
1. **Overview**: Why causality matters for AI agents
2. **Template Sections**: Explanation of each causality section
3. **Workflow**: How to use causality-aware tasks
4. **Examples**: Good vs bad causality documentation
5. **Best Practices**: Tips for effective causality preservation
6. **Upgrade Guide**: How to migrate legacy tasks

## Section Content Outline

### 1. Overview - Why Causality Matters
- Problem: Temporal drift in vibe coding
- Solution: Causality preservation in task format
- Benefits: Better context, fewer mistakes, faster work

### 2. Template Sections Explained

**Intent Section:**
- What: Document the "why" not just the "what"
- When: Always fill this when creating/starting task
- Example: Good vs vague intent

**Pre-Implementation Exploration:**
- What: Checklist to prevent jumping into coding
- When: Complete before writing first line of code
- Why: Catches complexity and causality issues early

**Implementation Context:**
- What: File paths, entry points, git context
- When: As you explore the codebase
- Why: Next session knows where to look

**Expected Causality Chain:**
- What: Document the event flow and failure modes
- When: After exploration, before implementation
- Why: Defines success criteria and testing approach

**Session Handoff:**
- What: Summary for next session/agent
- When: When marking task done
- Why: Prevents temporal causality loss

### 3. Workflow Example

**Show complete workflow from creation to completion:**
```bash
# Create task with causality template
$ taskguard create --title "Implement rate limiting middleware"

# AI agent fills sections as they work:
# 1. Read task → See pre-implementation checklist
# 2. Explore codebase → Fill "Implementation Context"
# 3. Document expected flow → Fill "Expected Causality Chain"
# 4. Implement feature → Update "Current State"
# 5. Complete task → Fill "Session Handoff"

# Mark done
$ taskguard status done backend-001

# Next task can reference the handoff
```

### 4. Examples Section

**Good Causality Documentation:**
```markdown
## Expected Causality Chain
**What should happen when this works:**
1. Request hits rate limiter middleware → Checks Redis for request count
2. Under limit → Increments counter → Proceeds to next middleware
3. Over limit → Returns 429 Too Many Requests → Logs violation

**Failure modes:**
- Redis connection fails → Middleware should fail open (allow request)
- Counter increment race condition → Use atomic INCR operation
- Clock skew between servers → Use UTC timestamps
```

**Bad (Vague) Causality Documentation:**
```markdown
## Expected Causality Chain
The rate limiter should work.
```

### 5. Best Practices for AI Agents

1. **Fill sections during work, not after:**
   - Update "Implementation Context" as you explore
   - Document "Expected Causality Chain" before implementing
   - Complete "Session Handoff" when marking done

2. **Be specific in causality chains:**
   - Name actual functions and files
   - Include failure modes
   - Document async/event flows

3. **Use session handoffs effectively:**
   - What changed (code + runtime behavior)
   - What to verify (testing checklist)
   - What next task should know

4. **Upgrade legacy tasks opportunistically:**
   - When working on legacy task, upgrade it first
   - Fill causality sections based on exploration
   - Help improve system quality

### 6. Upgrade Workflow

**For AI agents working with legacy tasks:**

```bash
# Detect legacy tasks
$ taskguard validate
# Shows: "⚠️ Found 13 legacy format tasks"

# Upgrade specific task you're working on
$ taskguard upgrade backend-001

# Or upgrade all at once
$ taskguard upgrade --all

# Fill in causality sections during implementation
# (sections will have AI prompts: "[AI: Document...]")
```

## Writing Style Guidelines

**Match existing guide style:**
- Clear, conversational tone
- Use examples liberally
- Include both CLI commands and explanations
- Use emoji icons sparingly but effectively (✅ ❌ 💡 ⚠️)
- Code blocks with syntax highlighting
- Real-world scenarios

**Avoid:**
- Overly technical language
- Jargon without explanation
- Walls of text without examples
- Theoretical concepts without practical application

## Implementation Steps

1. **Read current guide:**
   ```bash
   cat AGENTIC_AI_TASKGUARD_GUIDE.md
   ```

2. **Identify insertion point:**
   - After "Task Management Basics"
   - Before "Advanced Features" (if exists)

3. **Write new section:**
   - Follow outline above
   - Use existing guide's style
   - Include all examples

4. **Add table of contents entry:**
   - Update TOC to include new section

5. **Review for clarity:**
   - Read as if you're an AI agent
   - Check examples are clear
   - Ensure workflow is intuitive

## Documentation Checklist

- [ ] Overview explains "why" clearly
- [ ] Each template section explained with purpose
- [ ] Complete workflow example provided
- [ ] Good vs bad examples included
- [ ] Best practices actionable
- [ ] Upgrade workflow documented
- [ ] Style matches existing guide
- [ ] Code examples tested
- [ ] No broken links
- [ ] Table of contents updated

## Task Dependencies
- **Blocks**: None (documentation is final)
- **Blocked by**:
  - causality-upgrade-001 (template must exist)
  - causality-upgrade-002 (upgrade command must exist)
  - causality-upgrade-003 (validate enhancements must exist)
- **Related**: All causality-aware upgrade tasks

## Complexity Assessment
- **Estimate**: 2 hours
- **Complexity**: 4/10 - Straightforward documentation writing
- **Risk factors**:
  - Documentation unclear → Reduced adoption
  - Examples confusing → Bad patterns copied
  - Too verbose → Readers skip it
  - Need concise but comprehensive

## Session Notes
- **Created**: 2025-10-30
- **Next session should**: Read existing guide first, understand tone and structure

## Session Handoff
To be filled when task is complete.