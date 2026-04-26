# Quality Analysis

TaskGuard's `lint` command analyzes your task files for complexity and quality, helping you write tasks that are clear, actionable, and well-structured.

---

## Running the Linter

```bash
taskguard lint                  # Lint all tasks
taskguard lint --area backend   # Lint a single area
taskguard lint --verbose        # Show all tasks, including clean ones
```

---

## Complexity Score (1–10)

Complexity is computed from the YAML front-matter `complexity` field you set when creating a task. The linter interprets it on this scale:

| Score | Label | Meaning |
|-------|-------|---------|
| 1–2 | Simple | Trivial change, well-understood |
| 3–4 | Low | Standard implementation work |
| 5–6 | Medium | Requires design decisions |
| 7–8 | High | Significant effort, multiple moving parts |
| 9–10 | Very High | Architecture-level, high risk |

**Recommendation:** Tasks with complexity ≥ 8 are flagged — consider breaking them into smaller tasks with explicit dependencies.

---

## Quality Score (0–10)

The quality score is computed by `TaskAnalyzer` from the task's content. A higher score means the task is more actionable and complete.

### Scoring Factors

| Factor | Points | Notes |
|--------|--------|-------|
| Has a title | +1 | Basic requirement |
| Title is specific (>20 chars, not vague) | +1 | Penalizes titles like "Fix stuff" |
| Has a time estimate | +1 | Any non-empty `estimate` field |
| Has dependencies set | +1 | At least one entry in `dependencies` |
| Has checklist items | +2 | One or more `- [ ]` items in the body |
| Has acceptance criteria | +2 | `## Acceptance Criteria` section present |
| Has context section | +1 | `## Context` section present |
| Reasonable complexity set | +1 | Complexity between 1–7 (not default 3 AND not ≥8) |

A score of **8–10** is considered high quality. Below **5** generates a Warning.

---

## Issue Severity Levels

### Error
Structural problems that could cause `taskguard` commands to fail:
- YAML parse error
- Missing required fields (`id`, `title`, `status`, `area`)
- Invalid status or priority value

### Warning
Quality problems that make the task harder to execute:
- Quality score below 5
- No checklist items (nothing actionable)
- No estimate
- Complexity set to the default value of 3 without being changed
- Title under 10 characters

### Info
Improvement suggestions:
- No `## Context` section (hard for AI agents to understand why the task exists)
- No `## Acceptance Criteria` (unclear definition of done)
- More than 10 checklist items (consider splitting the task)

---

## Example Output

```
📋 Task Quality Report

backend-001  "Implement JWT authentication middleware"
  Complexity: 6/10  Quality: 9/10  ✅ No issues

backend-002  "Fix bug"
  Complexity: 3/10  Quality: 3/10
  ⚠️  Warning: Title is too vague or short
  ⚠️  Warning: No checklist items — nothing actionable
  ℹ️  Info: Add a ## Context section explaining why this task exists
  ℹ️  Info: Add ## Acceptance Criteria for a clear definition of done

api-003  "Refactor everything"
  Complexity: 9/10  Quality: 5/10
  ⚠️  Warning: High complexity (9) — consider splitting into smaller tasks
  ⚠️  Warning: Quality score below threshold (5/10)

SUMMARY
  Analyzed: 3 tasks
  Errors:   0
  Warnings: 4
  Info:     2
  High quality (≥8): 1 task
```

---

## Tips for High-Quality Tasks

1. **Write specific titles** — "Add JWT token validation to auth middleware" beats "Fix auth".
2. **Always add checklist items** — break the work into concrete steps.
3. **Write acceptance criteria** — define what "done" looks like before you start.
4. **Add a Context section** — explain *why* the task exists, not just what to do. This is critical for AI agents picking up the task in a new session.
5. **Set a realistic complexity** — don't leave the default `3` on everything.
6. **Add a time estimate** — even a rough one (`"half day"`) helps planning.
