# Task Management

Core task management commands and workflows.

---

## Commands Overview

| Command | Purpose |
|---------|---------|
| `taskguard create` | Create new task |
| `taskguard list` | List tasks |
| `taskguard update` | Update task fields |
| `taskguard task update` | Update checklist items |
| `taskguard validate` | Check dependencies |

---

## Creating Tasks

```bash
# Basic creation
taskguard create --title "Task title" --area backend

# With priority
taskguard create --title "Critical fix" --area backend --priority critical

# Different areas
taskguard create --title "UI component" --area frontend --priority high
```

**Auto-generated:**
- Unique ID (`backend-001`, `frontend-001`, etc.)
- Timestamp
- Default status (`todo`)
- Task file in `tasks/{area}/{id}.md`

---

## Listing Tasks

```bash
# All tasks
taskguard list

# Filter by area
taskguard list --area backend

# Filter by status
taskguard list --status doing

# Combined filters
taskguard list --area frontend --status todo
```

---

## Updating Tasks

### Status
```bash
taskguard update status backend-001 doing
taskguard update status backend-001 review
taskguard update status backend-001 done
```

### Priority
```bash
taskguard update priority backend-001 critical
```

### Dependencies
```bash
taskguard update dependencies backend-001 "setup-001,config-001"
```

### Assignee
```bash
taskguard update assignee backend-001 "alice@example.com"
```

---

## Checklist Management

```bash
# Mark first item done
taskguard task update backend-001 1 done

# Mark second item done
taskguard task update backend-001 2 done

# Mark item as todo
taskguard task update backend-001 3 todo
```

---

## Next Steps

- [Dependencies](dependencies.md) - Manage task dependencies
- [Git Sync](git-sync.md) - Git integration
