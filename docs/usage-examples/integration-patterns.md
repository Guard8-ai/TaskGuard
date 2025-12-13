# Integration Patterns

Integrate TaskGuard with other tools and workflows.

---

## Git Workflow Integration

```bash
# Commit with task IDs
git commit -m "backend-001: Implement authentication"

# Sync from git
taskguard sync

# Auto-suggest status updates
```

---

## CI/CD Integration

```yaml
# .github/workflows/taskguard.yml
name: Validate Tasks
on: [push]
jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo install taskguard
      - run: taskguard validate
```

---

## Next Steps

See [Troubleshooting](troubleshooting.md) for common issues.
