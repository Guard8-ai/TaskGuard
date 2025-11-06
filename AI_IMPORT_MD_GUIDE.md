# AI Agent Guide: Import-MD Markdown Authoring

Quick reference for creating markdown files that work seamlessly with `taskguard import-md`.

## Basic Format

```markdown
# Project Analysis: Component Name

## Task Breakdown

### Fix #1: Task Title Here
**Priority:** HIGH
**Effort:** 4 hours
**Area:** backend

Task description with details about what needs to be done.

#### Acceptance Criteria
- [ ] Criterion 1
- [ ] Criterion 2

### Issue #2: Another Task
**Priority:** MEDIUM
**Dependencies:** [backend-001, setup-003]

Description of this task.
```

## Supported Section Headers

The import-md parser recognizes these patterns (case-insensitive):

- `### Fix #N: Title`
- `### Issue #N: Title`
- `### Task #N: Title`
- `### Feature #N: Title`
- `### Bug #N: Title`

**Note:** Section numbers are ignored. TaskGuard auto-generates task IDs based on area (e.g., `backend-001`, `testing-002`).

## Metadata Fields

Place metadata immediately after the section header:

```markdown
### Fix #1: Authentication Bug
**Priority:** HIGH | CRITICAL | MEDIUM | LOW
**Effort:** 2 hours | 1 day | 3 days
**Area:** backend | frontend | testing | setup | tools
**Status:** todo | doing | review | done
**Dependencies:** [task-id-1, task-id-2]
**Tags:** [security, auth, urgent]
```

### Field Details

- **Priority** (optional): Sets task priority (default: MEDIUM)
- **Effort** (optional): Time estimate (supports hours/days/weeks)
- **Area** (required): Determines file location (tasks/AREA/AREA-NNN.md)
- **Status** (optional): Initial task status (default: todo)
- **Dependencies** (optional): Array of task IDs this task depends on
- **Tags** (optional): Array of tags for categorization

## ID Format Convention

TaskGuard uses **zero-padded 3-digit IDs**:
- ✅ `backend-001`, `backend-010`, `backend-100`
- ❌ `backend-1`, `backend-10` (old format)

Import-md automatically generates IDs in this format.

## Dependencies Best Practices

### Syntax
```markdown
**Dependencies:** [setup-001, auth-002, backend-015]
```

### Critical Post-Import Step
**ALWAYS run `taskguard validate` after import** to:
- Verify all dependency task IDs exist
- Detect circular dependencies
- Check for broken references
- See which tasks are ready to work on

Example:
```bash
taskguard import-md analysis.md
taskguard validate  # ← ESSENTIAL
```

### Validation Output
```
🚦 TASK STATUS
   ✅ Available tasks (dependencies satisfied):
      ⭕ setup-001 - Setup Task

   🚫 Blocked tasks:
      ❌ backend-001 - API Task (waiting for: setup-001)

❌ VALIDATION FAILED
   Found 1 issue:
   - backend-005: Depends on missing task 'setup-999'
```

## Complete Example

```markdown
# Backend API Analysis

## Task Breakdown

### Fix #1: Database Connection Pool
**Priority:** CRITICAL
**Effort:** 6 hours
**Area:** backend
**Tags:** [database, performance]

Implement connection pooling for PostgreSQL to handle concurrent requests.

#### Context
Current implementation creates a new connection per request, causing:
- High connection overhead
- Connection exhaustion under load
- Poor performance with 100+ concurrent users

#### Acceptance Criteria
- [ ] Implement connection pool with configurable size
- [ ] Add connection timeout handling
- [ ] Write unit tests for pool management
- [ ] Update documentation

### Issue #2: API Authentication Middleware
**Priority:** HIGH
**Effort:** 4 hours
**Area:** backend
**Dependencies:** [backend-001]
**Tags:** [security, auth]

Create middleware for JWT token validation on protected routes.

#### Implementation Notes
- Use jsonwebtoken crate
- Validate token signature and expiration
- Extract user ID from claims
- Return 401 for invalid tokens
```

## Workflow Summary

### Standard Workflow (Local-only)

1. **Author** markdown file with section headers and metadata
2. **Import** with `taskguard import-md your-file.md`
3. **Validate** with `taskguard validate` (CRITICAL)
4. **Fix** any dependency issues identified
5. **Work** on tasks using `taskguard list` and dependency blocking

### GitHub-integrated Workflow

1. **Author** markdown file with section headers and metadata
2. **Import** with `taskguard import-md your-file.md`
3. **Validate** with `taskguard validate` (CRITICAL)
4. **Fix** any dependency issues identified
5. **Sync to GitHub** with `taskguard sync --github`
6. **Work** on tasks (updates sync automatically)
7. **Archive when complete** with `taskguard archive` (closes GitHub issues)

## Conflict Handling

Import-md **skips** existing tasks by default:

```bash
$ taskguard import-md analysis.md
⚠️  Skipping task: tasks/backend/backend-001.md already exists
✅ Created task: tasks/backend/backend-002.md
```

To update existing tasks, edit them directly - import-md won't overwrite.

## Tips for AI Agents

- **Always specify Area:** Without it, tasks go to `setup/` by default
- **Use realistic effort estimates:** Helps with planning and prioritization
- **Group related tasks:** Use dependencies to enforce order
- **Run validation:** Catch dependency issues immediately after import
- **Check for conflicts:** Import-md skips existing IDs - plan accordingly
- **Sync after import (GitHub integration):** Run `taskguard sync --github` after importing to create GitHub issues
- **Validate before archiving:** Always run `taskguard validate` before archiving to see sync status
- **Preview archives:** Use `taskguard archive --dry-run` to preview what will be archived and which issues will close

## Example AI Workflow

### Local-only Example

```
Human: "Analyze the authentication system and create tasks"

AI: I'll create an analysis markdown and import it:

1. Writing analysis.md with:
   - Setup task for dependencies
   - Auth implementation tasks
   - Testing tasks with dependencies

2. Running: taskguard import-md analysis.md

3. Running: taskguard validate

4. Checking output:
   - ✅ All dependencies valid
   - 🚫 Found 2 blocked tasks (correct - waiting on setup)

5. Summary: Created 5 tasks, 2 ready to work on
```

### GitHub-integrated Example

```
Human: "Analyze the authentication system, create tasks, and sync to GitHub"

AI: I'll create an analysis markdown, import it, and sync to GitHub:

1. Writing analysis.md with:
   - Setup task for dependencies
   - Auth implementation tasks
   - Testing tasks with dependencies

2. Running: taskguard import-md analysis.md --area auth --prefix auth

3. Running: taskguard validate
   - ✅ All dependencies valid
   - 🚫 Found 2 blocked tasks (correct - waiting on setup)

4. Running: taskguard sync --github
   - ✅ Created GitHub issue #42 for auth-001
   - ✅ Created GitHub issue #43 for auth-002
   - ✅ Added issues to Projects v2 board

5. Summary: Created 5 tasks, synced to GitHub, 2 ready to work on

6. Later, after completion:
   - Running: taskguard archive
   - ✅ Archived auth-001 (closed GitHub issue #42)
   - ✅ Archived auth-002 (closed GitHub issue #43)
```

---

**Version:** 0.3.0
**Last Updated:** 2025-01-05
