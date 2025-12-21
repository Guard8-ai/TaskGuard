---
id: causality-013
title: Install TaskGuard globally on system
status: done
priority: high
tags:
- causality
- v0.4.0
- install
- global
dependencies:
- causality-012
assignee: developer
created: 2025-12-21T12:00:00Z
estimate: 15m
complexity: 2
area: causality
---

# Install TaskGuard globally on system

> **AI Agent Instructions:**
> 1. Install release binary to system PATH
> 2. Verify global installation works
> 3. Test from any directory

## Installation Options

### Option 1: cargo install (from crates.io after publish)
```bash
cargo install taskguard
```

### Option 2: Manual install (from release binary)
```bash
# Copy to /usr/local/bin or ~/.local/bin
sudo cp ./target/release/taskguard /usr/local/bin/taskguard
# OR for user-only install
cp ./target/release/taskguard ~/.local/bin/taskguard
```

### Option 3: cargo install from local
```bash
cargo install --path .
```

## Verification

### 1. Check Installation
```bash
which taskguard
taskguard --version
# Expected: taskguard 0.4.0
```

### 2. Test from Different Directory
```bash
cd /tmp
taskguard --help
# Expected: Help output works
```

### 3. Test in Another Project
```bash
cd /tmp
mkdir test-project && cd test-project
taskguard init
taskguard list
# Expected: Initializes and shows setup-001
```

### 4. Clean Up
```bash
rm -rf /tmp/test-project
```

## Acceptance Criteria

- [x] taskguard available in PATH
- [x] `taskguard --version` shows 0.4.0
- [x] Works from any directory
- [x] Can initialize new projects

## Session Handoff

**Completed:** 2025-12-21

**What was done:**
- Copied release binary to ~/.cargo/bin/taskguard
- Verified `which taskguard` returns correct path
- Verified `taskguard --version` shows 0.4.0
- Tested in /tmp/tg-final-test - works correctly

**Installation path:** ~/.cargo/bin/taskguard (user PATH)