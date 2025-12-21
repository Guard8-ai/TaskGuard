---
id: causality-010
title: Build and install locally for verification
status: done
priority: high
tags:
- causality
- v0.4.0
- build
- install
dependencies:
- causality-009
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

- [x] Release binary builds successfully
- [x] Version shows 0.4.0
- [x] Create without deps shows CAUTION and fails
- [x] Create with --allow-orphan-task succeeds
- [x] Create with --dependencies succeeds
- [x] validate --orphans works correctly
- [x] Test tasks cleaned up

## Session Handoff

**Completed:** 2025-12-21

**What was done:**
- Built release binary with `cargo build --release`
- Verified version shows `taskguard 0.4.0`
- Tested all causality features in /tmp/taskguard-test:
  - Create without deps → CAUTION message, exit 1 ✓
  - Create with --allow-orphan-task → Success with note ✓
  - Create with --dependencies "setup-001" → Success ✓
  - validate --orphans → Shows orphan details correctly ✓

**Verification output:**
```
taskguard 0.4.0
⚠️  CAUTION: Task has no dependencies.
```