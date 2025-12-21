---
id: causality-007
title: Update AGENTIC_AI_TASKGUARD_GUIDE.md with causality workflow
status: done
priority: medium
tags:
- causality
- v0.4.0
- docs
- agentic
dependencies:
- causality-006
assignee: developer
created: 2025-12-21T12:00:00Z
estimate: 1h
complexity: 3
area: causality
---

# Update AGENTIC_AI_TASKGUARD_GUIDE.md with causality workflow

> **AI Agent Instructions:**
> 1. Read the current AGENTIC_AI_TASKGUARD_GUIDE.md
> 2. Add causality tracking section
> 3. Update Quick Reference with new flags
> 4. Emphasize --dependencies is REQUIRED
> 5. Add --allow-orphan-task for edge cases

## Context

The `AGENTIC_AI_TASKGUARD_GUIDE.md` is the primary reference for AI coding agents. It must be updated to reflect causality tracking requirements in v0.4.0.

## Changes Required

### 1. Update Quick Reference

Add `--allow-orphan-task` and emphasize `--dependencies`:

```bash
# Core commands
taskguard init                                    # Initialize
taskguard create --title "Task" --area backend \
  --dependencies "setup-001"                      # ← REQUIRED
taskguard list                                    # List tasks
taskguard validate                                # Check dependencies
taskguard validate --orphans                      # Check for orphan tasks

# Full create (ALWAYS use --dependencies)
taskguard create --title "Task" --area backend --priority high \
  --complexity 7 --dependencies "setup-001" --estimate "4h"

# Edge case: orphan task (not recommended)
taskguard create --title "Spike" --area testing \
  --allow-orphan-task                             # Only for experiments
```

### 2. Add Causality Tracking Section

Add new section after "Workflow":

```markdown
## Causality Tracking (v0.4.0+)

**Every task MUST have dependencies.** TaskGuard enforces this.

### The Root: setup-001

All tasks trace back to `setup-001` (like Java's Object):

```
setup-001 (root)
    ├── backend-001 (depends on setup-001)
    │   └── api-001 (depends on backend-001)
    └── frontend-001 (depends on setup-001)
```

### Creating Tasks

```bash
# ✅ CORRECT: Always specify dependencies
taskguard create --title "API endpoint" --area api --dependencies "backend-001"

# ❌ WRONG: This will FAIL with CAUTION
taskguard create --title "API endpoint" --area api
# ⚠️  CAUTION: Task has no dependencies.
#    Orphan tasks break causality tracking...

# ⚠️ ESCAPE HATCH: Only for experiments/spikes
taskguard create --title "Spike" --area testing --allow-orphan-task
```

### Fixing Orphans

If you created orphan tasks:

```bash
# Check for orphans
taskguard validate --orphans

# Fix by adding dependencies
taskguard update dependencies docs-001 "api-001"
```

### Why This Matters

| Without Causality | With Causality |
|-------------------|----------------|
| Tasks float disconnected | Clear execution path |
| AI drifts off-topic | AI stays on critical path |
| Can't trace bug origins | Full traceability |
| Unclear priority | Dependency chain = priority |
```

### 3. Update Common Mistakes Table

```markdown
## Common Mistakes

| Wrong | Right |
|-------|-------|
| All tasks in one area | Spread across areas |
| Manual YAML editing | Use CLI commands |
| No validation | `taskguard validate` frequently |
| No dependencies | **ALWAYS use `--dependencies`** |
| Ignore CAUTION | Fix orphans immediately |
| Skip setup-001 | Chain from setup-001 |
```

### 4. Update Workflow Section

```markdown
## Workflow

1. **Init**: `taskguard init && taskguard validate`
2. **Create**: One task per area, **set dependencies at creation**
   - First task: `--dependencies "setup-001"`
   - Later tasks: `--dependencies "previous-task-id"`
3. **Start**: Read dependency task files first (check Session Handoff for context)
4. **Validate**: `taskguard validate` after each change
5. **Check Orphans**: `taskguard validate --orphans` if CAUTION appeared
6. **Update**: Use CLI commands, not manual file editing
7. **Complete**: `taskguard update status <id> done` + fill Session Handoff
8. **Commit**: `git add -A && git commit -m "feat(area): description"`
```

### 5. Add CAUTION Response Section

```markdown
## Responding to CAUTION

If you see:
```
⚠️  CAUTION: Task has no dependencies.
   Orphan tasks break causality tracking and reduce AI agent effectiveness.
```

**Immediately fix:**
```bash
# Option 1: Add dependencies to the task
taskguard update dependencies <new-task-id> "parent-task-id"

# Option 2: If truly orphan (spike/experiment), recreate with flag
taskguard create --title "..." --area ... --allow-orphan-task
```
```

## Files to Modify

- [ ] `AGENTIC_AI_TASKGUARD_GUIDE.md` - Full update with causality guidance

## Acceptance Criteria

- [ ] Quick Reference shows `--dependencies` as required
- [ ] New "Causality Tracking" section added
- [ ] Common Mistakes updated with orphan guidance
- [ ] Workflow section emphasizes dependencies
- [ ] CAUTION response guidance included
- [ ] `--allow-orphan-task` documented as escape hatch
- [ ] `validate --orphans` command documented