# Dependencies & Causality Tracking

TaskGuard v0.4.0 introduces **causality tracking** - every task must have dependencies to form semantic cause-effect chains.

---

## Key Concepts

| Concept | Description |
|---------|-------------|
| **Root Task** | `setup-001` is auto-created by `taskguard init` |
| **Dependencies** | Required for all tasks (use `--dependencies`) |
| **Orphan Task** | No dependencies AND nothing depends on it |
| **Escape Hatch** | `--allow-orphan-task` for spikes/research |

---

## Dependency Chain Example

```
setup-001 → backend-001 → api-001 → testing-001
         ↘ frontend-001 → integration-001
```

---

## Commands

```bash
# Create with dependencies (required)
taskguard create --title "API endpoints" --area api --dependencies "backend-001"

# Update dependencies
taskguard update dependencies api-001 "backend-001,auth-001"

# Check for orphan tasks
taskguard validate --orphans

# Create orphan (escape hatch)
taskguard create --title "Research" --area backend --allow-orphan-task
```

---

## Validation

```bash
# Standard validation
taskguard validate

# Show orphan details
taskguard validate --orphans
```

**Orphan = task with no dependencies AND nothing depends on it** (except `setup-001`).

---

## Archive Protection

Tasks with active dependents cannot be archived:

```bash
$ taskguard archive
⚠️  Cannot archive backend-001 - active tasks depend on it:
   - api-001
   - testing-001
```

Complete dependent tasks first, then archive.

---

## Next Steps

- [Task Management](task-management.md) - Core commands
- [Git Sync](git-sync.md) - Git integration
