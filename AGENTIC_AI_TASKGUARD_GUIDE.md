# TaskGuard for AI Agents

## Quick Reference

```bash
# Core commands
taskguard init                                    # Initialize
taskguard create --title "Task" --area backend    # Create task
taskguard list                                    # List tasks
taskguard validate                                # Check dependencies
taskguard update status <id> doing                # Update status

# Full create (recommended)
taskguard create --title "Task" --area backend --priority high \
  --complexity 7 --dependencies "setup-001" --estimate "4h"

# Update commands (format: update <field> <id> <value>)
taskguard update status <id> doing
taskguard update status <id> done
taskguard update priority <id> high
taskguard update dependencies <id> "dep1,dep2"
taskguard update assignee <id> "name"

# Checklist items
taskguard list items <id>                         # View checklist
taskguard task update <id> <item#> done           # Mark item done

# GitHub sync
taskguard sync --github                           # Sync to GitHub
taskguard archive                                 # Archive done tasks
taskguard restore <id>                            # Restore archived
```

## Workflow

1. **Init**: `taskguard init && taskguard validate`
2. **Create**: One task per area, set dependencies at creation
3. **Validate**: `taskguard validate` after each change
4. **Update**: Use CLI commands, not manual file editing
5. **Complete**: `taskguard update status <id> done`
6. **Commit**: `git add -A && git commit -m "feat(area): description"`

## Areas

`setup` | `backend` | `api` | `frontend` | `auth` | `data` | `testing` | `deployment` | `docs` | `integration`

## Priority

`critical` > `high` > `medium` > `low`

## Status Flow

`todo` → `doing` → `review` → `done` (or `blocked`)

## Dependency Chain Example

```
setup-001 → backend-001 → api-001 → testing-001
         ↘ frontend-001 → integration-001
```

## Common Mistakes

| Wrong | Right |
|-------|-------|
| All tasks in one area | Spread across areas |
| Manual YAML editing | Use CLI commands |
| No validation | `taskguard validate` frequently |
| No dependencies | Set with `--dependencies` flag |

## Troubleshooting

```bash
taskguard validate          # See parse errors & blocked tasks
taskguard list --area X     # Filter by area
gh auth status              # Check GitHub auth (for sync)
```

## GitHub Sync

```bash
# Setup: create .taskguard/github.toml
owner = "username"
repo = "repo"
project_number = 1

# Sync workflow
taskguard sync --github     # Push to GitHub
taskguard archive           # Archive done (closes issues)
taskguard restore <id>      # Restore (reopens issue)
```

---
**Rule**: Use CLI for all metadata. TaskGuard manages flow, you execute tasks.
