---
id: deployment-003
title: Improve TaskGuard installation to use consistent directory
status: done
priority: medium
tags:
- deployment
dependencies: []
assignee: developer
created: 2025-09-26T13:13:01.256136659Z
estimate: ~
complexity: 3
area: deployment
---

# Improve TaskGuard installation to use consistent directory

## Context
During the v0.1.1 update, discovered that `cargo install --path .` installs to `~/.cargo/bin/` but user's system has TaskGuard in `~/.local/bin/` (which is first in PATH). This creates version conflicts where the old version is found first.

**Current Problem:**
- `cargo install` → `/data/eliran/.cargo/bin/taskguard`
- User's PATH prioritizes → `/data/eliran/.local/bin/taskguard`
- Manual copy required to update system installation

## Objectives
- Streamline TaskGuard installation process
- Eliminate manual copy steps for updates
- Ensure consistent installation directory
- Improve user experience for system-wide installation

## Tasks
- [ ] Research cargo install target directory options
- [ ] Add installation script or Makefile for consistent deployment
- [ ] Consider adding `--install-dir` option or detection logic
- [ ] Update installation documentation in CLAUDE.md
- [ ] Test installation process on different systems
- [ ] Verify PATH priority handling

## Acceptance Criteria
✅ **Consistent Installation:**
- TaskGuard installs to user's preferred bin directory automatically
- No manual copy steps required for updates
- Version conflicts eliminated

✅ **User Experience:**
- Single command updates system installation
- Clear feedback about installation location
- Works regardless of user's PATH configuration

## Technical Notes
**Current Installation Paths:**
- `cargo install --path .` → `~/.cargo/bin/taskguard`
- User system expectation → `~/.local/bin/taskguard`
- PATH priority: `.local/bin` comes before `.cargo/bin`

**Potential Solutions:**
1. Installation script that detects existing TaskGuard location
2. Makefile with `make install` target
3. Cargo configuration to specify install directory
4. Documentation update with clear installation instructions

## Updates
- 2025-09-26: Task created after v0.1.1 installation issue discovery