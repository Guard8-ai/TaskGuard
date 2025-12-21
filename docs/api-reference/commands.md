# Command Reference

Complete reference for all TaskGuard CLI commands.

---

## Core Commands

### `taskguard init`
Initialize TaskGuard in current directory.

```bash
taskguard init
```

Creates `.taskguard/` config and `tasks/` directories.

---

### `taskguard create`
Create a new task. **Dependencies are required** (v0.4.0+).

```bash
taskguard create --title "Task name" --area backend --dependencies "setup-001"
```

| Flag | Short | Description |
|------|-------|-------------|
| `--title` | `-t` | Task title (required) |
| `--area` | `-a` | Task area (default: setup) |
| `--priority` | `-p` | low, medium, high, critical |
| `--complexity` | | 1-10 scale |
| `--tags` | | Comma-separated tags |
| `--dependencies` | `-d` | Comma-separated task IDs (required unless `--allow-orphan-task`) |
| `--assignee` | | Task assignee |
| `--estimate` | `-e` | Time estimate (e.g., "4h") |
| `--allow-orphan-task` | | Allow task without dependencies (for spikes/research) |

**Causality Tracking (v0.4.0+):**
- Every task must have dependencies to maintain semantic chains
- Use `--allow-orphan-task` for root tasks or research spikes
- Shows CAUTION if no dependencies specified

---

### `taskguard list`
List tasks with optional filters.

```bash
taskguard list [--area AREA] [--status STATUS]
taskguard list items <task-id>    # List checklist items
```

---

### `taskguard validate`
Check dependencies and show available tasks.

```bash
taskguard validate [--orphans]
```

| Flag | Description |
|------|-------------|
| `--orphans` | Show orphan tasks (no dependencies and no dependents) |

Shows: available tasks, blocked tasks, parse errors, GitHub sync status.

**Orphan Detection (v0.4.0+):**
- Orphan = task with no dependencies AND nothing depends on it
- `setup-001` is exempt (universal root)
- Use `--orphans` to see detailed orphan task list

---

### `taskguard update`
Update task fields.

```bash
taskguard update <field> <task-id> <value>
```

| Field | Values |
|-------|--------|
| `status` | todo, doing, review, done, blocked |
| `priority` | low, medium, high, critical |
| `dependencies` | Comma-separated task IDs |
| `assignee` | Assignee name |

---

### `taskguard task update`
Update checklist items within a task.

```bash
taskguard task update <task-id> <item-number> <status>
```

Example: `taskguard task update backend-001 1 done`

---

## GitHub Integration

### `taskguard sync --github`
Sync tasks with GitHub Issues and Projects v2.

```bash
taskguard sync --github [--dry-run] [--backfill-project]
```

Requires `.taskguard/github.toml`:
```toml
owner = "username"
repo = "repo"
project_number = 1
```

---

### `taskguard archive`
Archive completed tasks (closes GitHub issues if synced).

```bash
taskguard archive [--dry-run]
```

---

### `taskguard restore`
Restore archived task (reopens GitHub issue if synced).

```bash
taskguard restore <task-id>
```

---

## Analysis Commands

### `taskguard sync`
Analyze Git history for status suggestions.

```bash
taskguard sync [--verbose] [--limit N]
```

---

### `taskguard lint`
Analyze task complexity and quality.

```bash
taskguard lint [--area AREA] [--verbose]
```

---

### `taskguard ai`
Natural language task management.

```bash
taskguard ai "what should I work on next?"
```

---

## Utility Commands

### `taskguard import-md`
Import tasks from markdown file.

```bash
taskguard import-md FILE --area AREA --prefix PREFIX [--dry-run]
```

---

### `taskguard show`
Show detailed task information.

```bash
taskguard show <task-id>
```

---

### `taskguard stats`
Show storage statistics.

```bash
taskguard stats
```

---

### `taskguard clean`
Clean old completed tasks.

```bash
taskguard clean [--dry-run]
```
