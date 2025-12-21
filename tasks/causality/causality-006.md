---
id: causality-006
title: Update AI_IMPORT_MD_GUIDE.md with causality tracking guidance
status: done
priority: medium
tags:
- causality
- v0.4.0
- docs
- import-md
dependencies:
- causality-005
assignee: developer
created: 2025-12-21T12:00:00Z
estimate: 1h
complexity: 3
area: causality
---

# Update AI_IMPORT_MD_GUIDE.md with causality tracking guidance

> **AI Agent Instructions:**
> 1. Read the current AI_IMPORT_MD_GUIDE.md
> 2. Add new section on causality tracking and orphan avoidance
> 3. Update workflow sections to include orphan handling
> 4. Emphasize CAUTION output and how to fix orphans

## Context

The `AI_IMPORT_MD_GUIDE.md` needs updates to reflect the new causality tracking behavior in v0.4.0. AI agents need to understand:
1. Why dependencies matter
2. How to declare dependencies in markdown
3. What to do when CAUTION appears after import
4. How to fix orphan tasks

## Changes Required

### 1. Add Causality Tracking Section

Add after "Dependencies Best Practices" section:

```markdown
## Causality Tracking (v0.4.0+)

TaskGuard enforces **causality tracking** - every task must connect to the task graph.

### Why Causality Matters

- **Traceability:** Know where each task came from
- **AI Agent Focus:** Stay on critical paths, avoid drift
- **Post-mortem:** Trace bugs back to originating features
- **Priority:** Dependency chains determine execution order

### The Root Task: setup-001

Every TaskGuard project has `setup-001` as the root task (like Java's Object).
New tasks should trace back to it directly or transitively:

```
setup-001 (root)
    ├── backend-001
    │   └── api-001
    ├── frontend-001
    │   └── ui-001
    └── testing-001
```

### Declaring Dependencies in Markdown

Always specify dependencies when authoring import markdown:

```markdown
### Fix #1: Setup Database
**Priority:** HIGH
**Area:** backend
**Dependencies:** [setup-001]

Database configuration and setup.

### Fix #2: Implement API
**Priority:** HIGH
**Area:** backend
**Dependencies:** [backend-001]

REST API implementation.
```

### What Happens Without Dependencies

If tasks are imported without dependencies, you'll see:

```
⚠️  CAUTION: 2 orphan tasks created (no dependencies, nothing depends on them):
   - docs-001: API Documentation
   - testing-001: Unit Test Setup

   Orphan tasks break causality tracking. Add dependencies with:
     taskguard update dependencies docs-001 "api-001"
     taskguard update dependencies testing-001 "setup-001"
```

**IMPORTANT:** The import SUCCEEDS but you must fix orphans!
```

### 2. Update Workflow Summary

Update the "Workflow Summary" section:

```markdown
## Workflow Summary

### Standard Workflow (Local-only)

1. **Author** markdown file with section headers and metadata
   - **Always include Dependencies** for each task
   - First task should depend on `setup-001` or existing task
2. **Import** with `taskguard import-md your-file.md`
3. **Check for CAUTION** - if orphans detected, fix them:
   ```bash
   taskguard update dependencies <task-id> "parent-task-id"
   ```
4. **Validate** with `taskguard validate` (CRITICAL)
5. **Work** on tasks using `taskguard list` and dependency blocking
```

### 3. Update Tips for AI Agents

Add to "Tips for AI Agents" section:

```markdown
- **Always declare dependencies:** Every task needs a parent in the graph
- **Start from setup-001:** First tasks in your analysis should depend on setup-001
- **Chain dependencies:** Task #2 should depend on Task #1 if order matters
- **Fix orphans immediately:** When CAUTION appears, add dependencies before proceeding
- **Use validate --orphans:** Check for orphan tasks with `taskguard validate --orphans`
```

### 4. Update Complete Example

Modify the example to include proper dependency chains:

```markdown
## Complete Example

```markdown
# Backend API Analysis

## Task Breakdown

### Fix #1: Database Connection Pool
**Priority:** CRITICAL
**Effort:** 6 hours
**Area:** backend
**Dependencies:** [setup-001]  ← REQUIRED: Links to project root
**Tags:** [database, performance]

Implement connection pooling for PostgreSQL.

### Issue #2: API Authentication Middleware
**Priority:** HIGH
**Effort:** 4 hours
**Area:** backend
**Dependencies:** [backend-001]  ← Chains from previous task
**Tags:** [security, auth]

Create middleware for JWT token validation.

### Issue #3: API Documentation
**Priority:** MEDIUM
**Effort:** 2 hours
**Area:** docs
**Dependencies:** [backend-002]  ← Depends on auth being done
**Tags:** [docs]

Document the authentication endpoints.
```
```

### 5. Update Version

Change:
```markdown
**Version:** 0.4.0
**Last Updated:** 2025-12-21
```

## Files to Modify

- [ ] `AI_IMPORT_MD_GUIDE.md` - Add causality section, update workflows

## Acceptance Criteria

- [ ] New "Causality Tracking" section added
- [ ] All examples include proper dependencies
- [ ] Workflow sections mention orphan handling
- [ ] Tips section includes dependency guidance
- [ ] CAUTION output example included
- [ ] Version updated to 0.4.0