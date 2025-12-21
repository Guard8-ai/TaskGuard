---
id: causality-011
title: "Verify CI/CD pipeline passes"
status: todo
priority: critical
tags: [causality, v0.4.0, ci, quality]
dependencies: [causality-010]
assignee: developer
created: 2025-12-21T12:00:00Z
estimate: 30m
complexity: 3
area: causality
---

# Verify CI/CD pipeline passes

> **AI Agent Instructions:**
> 1. Push branch to GitHub
> 2. Monitor CI workflow
> 3. Ensure ALL checks pass
> 4. Fix any CI failures before proceeding

## Steps

### 1. Push Feature Branch
```bash
git push -u origin feature/0.4.0-causality-tracking
```

### 2. Monitor CI
```bash
gh run list --branch feature/0.4.0-causality-tracking
gh run watch  # Watch latest run
```

### 3. Required Checks
- [ ] Build (Linux)
- [ ] Build (macOS)
- [ ] Build (Windows)
- [ ] Tests pass
- [ ] Clippy lint passes
- [ ] Format check passes

### 4. Fix Any Failures
If CI fails:
1. Check logs: `gh run view <run-id> --log-failed`
2. Fix issues locally
3. Commit and push
4. Wait for CI to pass

## Acceptance Criteria

- [ ] Feature branch pushed to GitHub
- [ ] All CI checks pass (green)
- [ ] No warnings in build logs
- [ ] Tests pass on all platforms
