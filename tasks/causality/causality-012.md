---
id: causality-012
title: "Merge to main and tag v0.4.0 release"
status: todo
priority: critical
tags: [causality, v0.4.0, release, merge]
dependencies: [causality-011]
assignee: developer
created: 2025-12-21T12:00:00Z
estimate: 30m
complexity: 3
area: causality
---

# Merge to main and tag v0.4.0 release

> **AI Agent Instructions:**
> 1. Create PR from feature branch
> 2. Merge to main (after CI passes)
> 3. Create v0.4.0 tag
> 4. Push tag to trigger release workflow

## Steps

### 1. Create Pull Request
```bash
gh pr create --title "feat(v0.4.0): Causality Tracking" --body "$(cat <<'EOF'
## Summary
- Enforce task dependencies at creation time
- Add `--allow-orphan-task` flag for edge cases
- Add `validate --orphans` to detect orphan tasks
- Update all documentation and AI guides

## Changes
- causality-002: CAUTION on create without dependencies
- causality-003: validate --orphans flag
- causality-004: Archive protection improvements
- causality-005: import-md orphan detection
- causality-006: AI_IMPORT_MD_GUIDE.md updates
- causality-007: AGENTIC_AI_TASKGUARD_GUIDE.md updates

## Test Plan
- [x] All tests pass
- [x] Clippy clean
- [x] Manual verification of new features
- [x] CI passes on all platforms

🤖 Generated with [Claude Code](https://claude.ai/claude-code)
EOF
)"
```

### 2. Merge PR (after CI passes)
```bash
gh pr merge --squash --delete-branch
```

### 3. Pull Latest Main
```bash
git checkout master
git pull origin master
```

### 4. Create and Push Tag
```bash
git tag -a v0.4.0 -m "v0.4.0: Causality Tracking"
git push origin v0.4.0
```

### 5. Monitor Release Workflow
```bash
gh run list --workflow=release.yml
gh run watch  # Watch release build
```

## Acceptance Criteria

- [ ] PR created and reviewed
- [ ] PR merged to main
- [ ] v0.4.0 tag created
- [ ] Release workflow triggered
- [ ] Release artifacts published
