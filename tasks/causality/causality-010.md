---
id: causality-010
title: "Build and install locally for verification"
status: todo
priority: high
tags: [causality, v0.4.0, build, install]
dependencies: [causality-009]
assignee: developer
created: 2025-12-21T12:00:00Z
estimate: 30m
complexity: 2
area: causality
---

# Build and install locally for verification

> **AI Agent Instructions:**
> 1. Build release binary
> 2. Install to local project
> 3. Verify new features work correctly
> 4. Test the full workflow

## Build Steps

### 1. Build Release Binary
```bash
cargo build --release
```

### 2. Verify Binary
```bash
./target/release/taskguard --version
# Expected: taskguard 0.4.0
```

### 3. Test New Features

#### Test create without dependencies (should fail)
```bash
./target/release/taskguard create --title "Test" --area testing
# Expected: CAUTION message, exit 1
```

#### Test create with --allow-orphan-task
```bash
./target/release/taskguard create --title "Test Orphan" --area testing --allow-orphan-task
# Expected: Success with note
```

#### Test create with dependencies
```bash
./target/release/taskguard create --title "Test Linked" --area testing --dependencies "setup-001"
# Expected: Success
```

#### Test validate --orphans
```bash
./target/release/taskguard validate --orphans
# Expected: Shows orphan tasks (if any)
```

### 4. Clean Up Test Tasks
```bash
rm tasks/testing/testing-*.md 2>/dev/null || true
```

## Acceptance Criteria

- [ ] Release binary builds successfully
- [ ] Version shows 0.4.0
- [ ] Create without deps shows CAUTION and fails
- [ ] Create with --allow-orphan-task succeeds
- [ ] Create with --dependencies succeeds
- [ ] validate --orphans works correctly
- [ ] Test tasks cleaned up
