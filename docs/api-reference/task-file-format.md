# Task File Format

Every TaskGuard task is a Markdown file with a YAML front-matter block.

---

## File Location

```
tasks/{area}/{area}-{NNN}.md
```

- `{area}` — the task's area (e.g. `backend`, `api`, `frontend`)
- `{NNN}` — zero-padded 3-digit sequential number (e.g. `001`, `042`)

**Examples:** `tasks/backend/backend-001.md`, `tasks/api/api-012.md`

Archived tasks are stored at the same relative path under `.taskguard/archive/`:

```
.taskguard/archive/{area}/{area}-{NNN}.md
```

---

## File Structure

```
---
<YAML front-matter>
---

<Markdown body>
```

The first `---` opens the front-matter, the second `---` closes it. Everything after is the markdown body (task description, checklist, notes).

---

## YAML Front-Matter Fields

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `id` | String | Yes | — | Unique task ID, e.g. `backend-001` |
| `title` | String | Yes | — | Short human-readable task title |
| `status` | Enum | Yes | `todo` | Current status (see below) |
| `priority` | Enum | Yes | `medium` | Priority level (see below) |
| `area` | String | Yes | — | Matches the parent directory name |
| `dependencies` | List\<String\> | No | `[]` | Task IDs this task depends on |
| `tags` | List\<String\> | No | `[]` | Free-form labels; area is always first tag |
| `assignee` | String | No | `developer` | Person or agent responsible |
| `created` | DateTime | Yes | Now | ISO 8601 creation timestamp |
| `estimate` | String | No | — | Free-form estimate, e.g. `"4h"`, `"2d"` |
| `complexity` | Integer | No | `3` | Complexity score 1–10 |

### Status Values

| Value | Meaning |
|-------|---------|
| `todo` | Not started |
| `doing` | Actively in progress |
| `review` | Completed, awaiting review |
| `done` | Finished |
| `blocked` | Cannot proceed (waiting on something external) |

### Priority Values

| Value | Icon | Meaning |
|-------|------|---------|
| `low` | 🟢 | Nice-to-have |
| `medium` | 🟡 | Standard work item |
| `high` | 🟠 | Important, do soon |
| `critical` | 🔴 | Blocking or urgent |

---

## Markdown Body

The body is free-form markdown. TaskGuard templates generate a standard structure, but only the `- [ ]` checklist syntax has special meaning (used by `taskguard list items` and `taskguard task update`).

**Standard template sections:**

```markdown
# Task Title

## Causation Chain
[Domain-specific prompt guiding the AI to think causally about this task]

## Pre-flight Checks
- [ ] Read all dependency task files
- [ ] Run: grep -r "keyword" src/

## Context
[Why this task exists and what problem it solves]

## Tasks
- [ ] Subtask one
- [ ] Subtask two
- [x] Subtask three (already done)

## Acceptance Criteria
- [ ] All tests pass
- [ ] Feature works end-to-end

---
**Session Handoff** (fill when done):
- Changed: [files/functions modified]
- Causality: [what triggers what as a result of this task]
- Verify: [how to confirm correctness]
- Next: [context for anyone working on dependent tasks]
```

The **Session Handoff** section is specifically for AI agent continuity — it records what changed and provides context for the next session working on dependent tasks.

### Checklist Items

Items matching `- [ ] text` or `- [x] text` are parsed by:

- `taskguard list items <task-id>` — display with 1-based index and completion %
- `taskguard task update <task-id> <N> done|todo` — toggle by index

---

## Complete Example

```markdown
---
id: backend-001
title: Implement JWT authentication middleware
status: doing
priority: high
tags:
  - backend
  - security
  - auth
dependencies:
  - setup-001
assignee: alice
created: 2025-10-05T10:00:00Z
estimate: 4h
complexity: 6
area: backend
---

# Implement JWT authentication middleware

## Causation Chain
Trace the request lifecycle: HTTP verb → middleware chain → handler.
This middleware intercepts every request before it reaches route handlers.

## Pre-flight Checks
- [ ] Read setup-001 task file for project context
- [ ] Run: grep -r "verify\|validate\|decode" src/

## Context
Legal requires all API endpoints to be authenticated before Q3.
The middleware validates tokens from the Authorization header and
populates the request context with the decoded user payload.

## Tasks
- [x] Add jsonwebtoken dependency to Cargo.toml
- [x] Create src/middleware/auth.rs
- [ ] Write unit tests for invalid/expired tokens
- [ ] Wire middleware into Axum router
- [ ] Document header format in README

## Acceptance Criteria
- [ ] All protected endpoints return 401 for missing/invalid tokens
- [ ] Valid tokens decode to correct user ID
- [ ] Tests cover expiry, wrong signature, malformed input

---
**Session Handoff** (fill when done):
- Changed: src/middleware/auth.rs, src/router.rs, Cargo.toml
- Causality: auth.rs::verify_token() → called by auth_layer in router.rs
- Verify: cargo test middleware::auth
- Next: api-001 needs to import AuthUser extractor from this middleware
```

---

## ID Format

```
{area}-{NNN}
```

- `area` — lowercase alphanumeric, matches directory name
- `NNN` — three digit, zero-padded, unique within the area
- Examples: `setup-001`, `backend-042`, `frontend-007`

IDs are never reused. When generating a new ID, TaskGuard scans both `tasks/{area}/` and `.taskguard/archive/{area}/` to find the current maximum and increments it.

---

## GitHub Mapping

When GitHub sync is enabled, task-to-issue mappings are stored in `.taskguard/github-mapping.json`:

```json
[
  {
    "task_id": "backend-001",
    "issue_number": 42,
    "issue_id": "I_kwDOABC123",
    "project_item_id": "PVTI_lADOABC456",
    "synced_at": "2025-10-05T10:30:00Z",
    "is_archived": false
  }
]
```

This file is committed to Git. `is_archived: true` is set when the task is archived via `taskguard archive`.
