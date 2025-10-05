# Common Workflows

Real-world TaskGuard usage patterns.

---

## Workflow 1: Solo Development

```bash
# 1. Initialize
cd ~/my-project
taskguard init

# 2. Create foundation tasks
taskguard create --title "Project setup" --area setup --priority critical

# 3. Create feature tasks
taskguard create --title "Build API" --area backend --priority high
taskguard create --title "Build UI" --area frontend --priority high

# 4. Add dependencies (edit files)
vim tasks/backend/backend-001.md
# dependencies: [setup-001]

# 5. Validate
taskguard validate

# 6. Work through tasks
taskguard update status setup-001 doing
# ... complete setup ...
taskguard update status setup-001 done

# 7. Backend unblocked
taskguard update status backend-001 doing
```

---

## Workflow 2: Team Collaboration

```bash
# Alice: Create tasks
taskguard create --title "Database schema" --area backend
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

# Add tasks
taskguard create --title "JWT implementation" --area auth
taskguard create --title "Login endpoint" --area api

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
