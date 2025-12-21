---
id: causality-001
title: v0.4.0 Causality Tracking - Parent Feature Task
status: done
priority: high
tags:
- causality
- v0.4.0
- feature
- parent
dependencies: []
assignee: developer
created: 2025-12-21T12:00:00Z
estimate: ~
complexity: 8
area: causality
---

# v0.4.0 Causality Tracking - Parent Feature Task

> **AI Agent Instructions:**
> This is the PARENT task for the v0.4.0 Causality Tracking feature.
> Complete subtasks in order: causality-002 through causality-007.
> Each subtask depends on the previous one.

## Overview

Version 0.4.0 introduces **strict causality tracking** for task dependencies. Every new task must have an explicit relationship to the task graph - either depending on another task or being depended upon. This grounds AI coding agents on critical paths by ensuring no task exists in isolation.

## Problem Statement

Orphan tasks - tasks with no dependencies and nothing depending on them - create:
1. **Lost context**: No traceability to why a task exists
2. **AI agent drift**: Agents work on disconnected tasks without understanding impact
3. **Post-mortem gaps**: Bug fixes can't be traced to originating features
4. **Priority ambiguity**: No dependency chain means unclear execution order

## Solution

Enforce explicit task relationships at creation time. Every task must answer: "Where did this come from?" or "What does this enable?"

## Key Design Decisions

### 1. `setup-001` as Universal Root
- Like Java's `Object` - the primordial ancestor
- Every project starts with `setup-001`
- All new tasks should trace back to it (directly or transitively)
- It's the **only legitimate orphan** - the root of the DAG

### 2. Keep Existing `--dependencies` Flag
- Backward compatible
- Comma-separated task IDs
- No new `--depends-on` or `--blocks` flags (confusing)

### 3. CAUTION Not WARNING
- AI models pay heightened attention to "CAUTION"
- Signals potential harm or irreversible actions
- "Warning" is often treated as informational noise

### 4. Soft vs Hard Enforcement
| Command | Orphan Behavior | Exit Code |
|---------|-----------------|-----------|
| `create` (no deps) | CAUTION + FAIL | 1 |
| `create --allow-orphan-task` | Creates with note | 0 |
| `import-md` (creates orphans) | CAUTION + SUCCEED | 0 |
| `validate --orphans` | Lists orphans | 0 |
| `archive` (has dependents) | FAIL | 1 |

### 5. Orphan Definition
```
ORPHAN =
  task.dependencies.is_empty()
  AND NOT any_task_depends_on(task.id)
  AND task.id != "setup-001"  // Root is exempt
```

## Subtasks

- [ ] causality-002: Add `--allow-orphan-task` flag and CAUTION to create command
- [ ] causality-003: Add `--orphans` flag to validate command
- [ ] causality-004: Strengthen archive command reverse dependency check
- [ ] causality-005: Update import-md with orphan detection and CAUTION output
- [ ] causality-006: Update AI_IMPORT_MD_GUIDE.md with dependency guidance
- [ ] causality-007: Update AGENTIC_AI_TASKGUARD_GUIDE.md with causality workflow

## Success Criteria

- [ ] `taskguard create` without dependencies shows CAUTION and exits 1
- [ ] `taskguard create --allow-orphan-task` creates task with acknowledgment
- [ ] `taskguard validate --orphans` lists all orphan tasks (excluding setup-001)
- [ ] `taskguard archive` blocks archiving tasks that have active dependents
- [ ] `taskguard import-md` outputs CAUTION for orphan tasks but succeeds
- [ ] AI guides updated with causality tracking workflow
- [ ] All existing tests pass
- [ ] New tests cover causality scenarios

## References

- Feature Spec: `/data/eliran/Downloads/FEATURE_0.4.0_CAUSALITY_TRACKING.md`
- Anthropic Best Practices: https://www.anthropic.com/engineering/claude-code-best-practices
- Long-Running Agents: https://www.anthropic.com/engineering/effective-harnesses-for-long-running-agents