# Command Reference

Complete reference for all TaskGuard CLI commands.

---

## `taskguard init`

Initialize TaskGuard in the current directory.

```bash
taskguard init
```

**Creates:**
- `.taskguard/config.toml` — project configuration
- `.taskguard/templates/` — custom template directory
- `.taskguard/state/` — local state (auto-appended to `.gitignore`)
- `tasks/` with default area subdirectories: `setup`, `backend`, `frontend`, `api`, `auth`, `testing`
- `tasks/setup/setup-001.md` — the root task (required ancestor for all future tasks)
- `AGENTIC_AI_TASKGUARD_GUIDE.md` — AI agent reference guide copied to project root

---

## `taskguard create`

Create a new task. **Dependencies are required** (v0.4.0+).

```bash
taskguard create --title "Task name" --area backend --dependencies "setup-001"
```

| Flag | Short | Type | Default | Description |
|------|-------|------|---------|-------------|
| `--title` | `-t` | String | — | Task title **(required)** |
| `--area` | `-a` | String | `setup` | Area / subdirectory |
| `--priority` | `-p` | String | `medium` | `low` / `medium` / `high` / `critical` |
| `--complexity` | — | Integer | `3` | Complexity score 1–10 |
| `--tags` | — | String | — | Comma-separated tags (area is auto-added) |
| `--dependencies` | `-d` | String | — | Comma-separated task IDs **(required unless `--allow-orphan-task`)** |
| `--assignee` | — | String | `developer` | Assignee name |
| `--estimate` | `-e` | String | — | Time estimate, e.g. `"4h"`, `"2d"` |
| `--allow-orphan-task` | — | Flag | false | Bypass dependency requirement (for spikes/research) |

**Causality enforcement:**
- If `--dependencies` is omitted and `--allow-orphan-task` is not set, the command exits with a `CAUTION` message and usage example.
- IDs are zero-padded three digits: `{area}-{NNN}` (e.g. `backend-007`).
- ID generation scans both active `tasks/{area}/` and `.taskguard/archive/{area}/` to prevent reuse of archived IDs.
- New areas not in `config.toml` are automatically added.

---

## `taskguard list`

List tasks with optional filters.

```bash
taskguard list
taskguard list --area backend
taskguard list --status doing
taskguard list --include-archive
taskguard list items <task-id>
```

| Flag | Short | Description |
|------|-------|-------------|
| `--area` | `-a` | Filter to a single area |
| `--status` | `-s` | Filter by status value |
| `--include-archive` | — | Include archived tasks from `.taskguard/archive/` |

**Subcommand `list items <task-id>`:** Lists markdown checklist items (`- [ ]` / `- [x]`) with 1-based index and completion percentage.

Output is grouped by area, sorted alphabetically. Each task shows status icon, priority icon, ID, title, and dependency list. Summary counts appear at the end.

**Status icons:** ⭕ todo · 🔄 doing · 👁 review · ✅ done · 🚫 blocked  
**Priority icons:** 🟢 low · 🟡 medium · 🟠 high · 🔴 critical

---

## `taskguard show <task-id>`

Show detailed information for a task.

```bash
taskguard show backend-001
```

> Note: Currently prints a summary header. Full detail view is planned.

---

## `taskguard validate`

Validate task graph integrity and show available/blocked tasks.

```bash
taskguard validate
taskguard validate --orphans
taskguard validate --sync-areas
```

| Flag | Description |
|------|-------------|
| `--orphans` | Show detailed list of orphan tasks |
| `--sync-areas` | Sync `config.toml` areas array with actual `tasks/` subdirectories |

**Checks performed:**
- YAML parse errors in task files
- Missing dependencies (referenced task IDs that do not exist)
- Circular dependencies (DFS cycle detection across all non-done tasks)
- Orphan tasks (no dependencies and nothing depends on them — `setup-001` is exempt)
- GitHub sync consistency (orphaned mappings, archived synced tasks)

**Output sections:** Task availability (available / blocked with reasons), summary counts, optional GitHub section.

Archived task dependencies are shown with a `📦` suffix and are still considered valid.

---

## `taskguard update`

Update a task's metadata field.

```bash
taskguard update <field> <task-id> <value>
```

| Field | Valid values | Notes |
|-------|-------------|-------|
| `status` | `todo` `doing` `review` `done` `blocked` | Warns on `todo→done` (skipped steps) and `done→anything` (reopening) |
| `priority` | `low` `medium` `high` `critical` | |
| `dependencies` | Comma-separated task IDs | `none` / `null` / empty string clears the list |
| `assignee` | Any string | `none` / `null` / empty string clears the field |

**Examples:**

```bash
taskguard update status backend-001 doing
taskguard update status backend-001 done
taskguard update priority api-003 critical
taskguard update dependencies frontend-002 "backend-001,api-002"
taskguard update assignee backend-001 alice
```

---

## `taskguard task update`

Mark a checklist item inside a task as done or todo.

```bash
taskguard task update <task-id> <item-number> <status>
```

- `item-number` is 1-based (use `taskguard list items <task-id>` to see numbers).
- `status`: `done` / `completed` / `true` or `todo` / `incomplete` / `false`.

```bash
taskguard task update backend-001 2 done
taskguard task update backend-001 2 todo
```

---

## `taskguard sync`

Two modes depending on flags.

### Git history analysis (default)

Reads recent git commits and suggests status changes for tasks referenced in commit messages.

```bash
taskguard sync
taskguard sync --limit 100 --verbose
taskguard sync --remote
```

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--limit` | `-l` | `50` | Number of commits to analyze (max 1000) |
| `--verbose` | `-v` | false | Show commit details and repo stats |
| `--remote` | `-r` | false | Fetch and analyze remote branch changes |

Detects task IDs via pattern `\b([a-zA-Z]{1,20})-(\d{3})\b` in commit messages. Suggestions are printed only — no changes are applied automatically.

### GitHub Issues / Projects v2 sync

```bash
taskguard sync --github
taskguard sync --github --dry-run
taskguard sync --github --backfill-project
```

| Flag | Description |
|------|-------------|
| `--github` | Enable GitHub sync mode |
| `--dry-run` | Preview actions without applying |
| `--backfill-project` | Add already-mapped issues to the Projects v2 board |

Requires `.taskguard/github.toml`:

```toml
owner = "your-org"
repo = "your-repo"
project_number = 1
```

**Behaviour:** Creates GitHub Issues for unmapped tasks, adds them to the configured Projects v2 board, maps status → column with fuzzy name matching (`doing` → "In Progress" / "In progress" / "Doing" / "Working"). Detects cross-branch duplicates by inspecting issue body for matching task ID and content hash.

---

## `taskguard lint`

Analyze task complexity and quality with actionable suggestions.

```bash
taskguard lint
taskguard lint --area backend
taskguard lint --verbose
```

| Flag | Short | Description |
|------|-------|-------------|
| `--area` | `-a` | Filter to a single area |
| `--verbose` | `-v` | Show all tasks, including those with no issues |

Reports per-task complexity score (0–10), quality score (0–10), and issues at three severity levels:
- **Error** — missing required fields, broken structure
- **Warning** — vague title, no estimate, no checklist items
- **Info** — improvement suggestions

See [Quality Analysis](../features/quality-analysis.md) for full scoring details.

---

## `taskguard ai "<query>"`

Natural language interface for task management. Runs fully in-process — no external API calls.

```bash
taskguard ai "what should I work on next?"
taskguard ai "I just finished backend-003"
taskguard ai "show me blocked tasks"
taskguard ai "analyze dependencies for api-001"
taskguard ai "how complex is the frontend area?"
taskguard ai "create a task for adding rate limiting"
```

Handles:
- **Next task recommendation** — sorted by priority then complexity (lower first)
- **Completion announcements** — extracts task reference, prints next steps
- **Status queries** — available / blocked / doing counts
- **Dependency analysis** — traces dependency chain for a task
- **Complexity analysis** — scores and explains per area
- **Creation suggestions** — outputs the exact `taskguard create` command to run (does not create)

---

## `taskguard import-md <file>`

Import tasks in bulk from a structured markdown file.

```bash
taskguard import-md analysis.md --area backend --dry-run
taskguard import-md fixes.md --area testing --prefix fix
```

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--area` | `-a` | `setup` | Default area for all imported tasks |
| `--prefix` | `-p` | Area name | Override the task ID prefix |
| `--dry-run` | — | false | Preview without creating files |
| `--start-number` | — | Auto | Override starting ID number |
| `--tags` | `-t` | — | Additional comma-separated tags for all tasks |
| `--priority` | — | Inferred | Override inferred priority for all tasks |

**Input format** — markdown sections with this structure:

```markdown
### Fix #1: Descriptive task title

**Priority:** high
**Effort:** 2h
**Area:** backend
**Status:** todo
**Dependencies:** setup-001
**Tags:** performance, api

Body text becomes the task description.
```

Supported section prefixes: `Fix #N`, `Issue #N`, `Task #N`, `Feature #N`, `Bug #N`.

Metadata fields (`**Priority:**`, `**Effort:**`, etc.) are extracted automatically. Skips files that already exist. After import, prints a `CAUTION` warning if any orphan tasks were created.

See [AI_IMPORT_MD_GUIDE.md](../../AI_IMPORT_MD_GUIDE.md) for the full bulk-import workflow.

---

## `taskguard archive`

Move all `done` tasks to `.taskguard/archive/`.

```bash
taskguard archive
taskguard archive --dry-run
```

| Flag | Description |
|------|-------------|
| `--dry-run` | Preview which tasks would be archived |

**Causality protection:** A done task that is still referenced in an active (non-done) task's `dependencies` list cannot be archived.

If GitHub sync is enabled, the corresponding GitHub Issue is closed and `is_archived: true` is set in `github-mapping.json`.

Creates a git commit: `Archive completed tasks: id1, id2, ...`

---

## `taskguard restore <task-id>`

Restore an archived task back to `tasks/{area}/`.

```bash
taskguard restore backend-003
taskguard restore backend-003 --dry-run
```

| Flag | Description |
|------|-------------|
| `--dry-run` | Preview without moving files |

If GitHub sync is enabled and the task was archived, the corresponding GitHub Issue is reopened.

Creates a git commit: `Restore archived task: {id}`

---

## `taskguard clean`

Permanently delete `done` tasks (not moved to archive — deleted).

```bash
taskguard clean
taskguard clean --dry-run
```

| Flag | Description |
|------|-------------|
| `--dry-run` | Preview which tasks would be deleted |

Respects the same causality protection as `archive` — tasks with active dependents cannot be deleted.

> **Use `archive` instead** if you want to preserve history. `clean` is irreversible.

---

## `taskguard compact`

Remove excessive blank lines from task markdown files (max one consecutive blank line).

```bash
taskguard compact
taskguard compact --dry-run
```

| Flag | Description |
|------|-------------|
| `--dry-run` | Show which files would be modified |

YAML front-matter is not affected.

---

## `taskguard stats`

Show storage statistics for the task directory.

```bash
taskguard stats
```

Reports: total storage size, file count and size by area, tasks by status, top 10 largest task files.

---

## `taskguard status`

Show a high-level project overview.

```bash
taskguard status
```

> Note: Currently prints a status header. Full overview is planned.
