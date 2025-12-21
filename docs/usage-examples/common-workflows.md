# Common Workflows

Real-world TaskGuard usage patterns.

---

## Workflow 1: Solo Development

```bash
# 1. Initialize (creates setup-001 as root)
cd ~/my-project
taskguard init

# 2. Create feature tasks with dependencies (v0.4.0+)
taskguard create --title "Build API" --area backend --priority high --dependencies "setup-001"
taskguard create --title "Build UI" --area frontend --priority high --dependencies "setup-001"

# 3. Validate and check orphans
taskguard validate --orphans

# 4. Work through tasks (setup-001 first)
taskguard update status setup-001 doing
# ... complete setup ...
taskguard update status setup-001 done

# 5. Backend/Frontend now unblocked
taskguard update status backend-001 doing
```

---

## Workflow 2: Team Collaboration

```bash
# Alice: Create tasks with dependencies
taskguard create --title "Database schema" --area backend --dependencies "setup-001"
git add tasks/
git commit -m "Add backend tasks"
git push

# Bob: Pull and work
git pull
taskguard list
taskguard update status backend-001 doing
git commit -am "Start database work"
git push

# Alice: Sync
git pull
taskguard list --status doing
```

---

## Workflow 3: Feature Branch Development

```bash
# Create feature branch
git checkout -b feature/authentication

# Add tasks with dependencies
taskguard create --title "JWT implementation" --area auth --dependencies "setup-001"
taskguard create --title "Login endpoint" --area api --dependencies "auth-001"

# Work and commit
taskguard update status auth-001 doing
# ... work ...
git add tasks/
git commit -m "Add auth tasks and implementation"

# Merge to main
git checkout main
git merge feature/authentication
```

---

## Next Steps

- [Integration Patterns](integration-patterns.md)
- [Troubleshooting](troubleshooting.md)
