---
id: causality-009
title: Update all documentation for v0.4.0
status: done
priority: high
tags:
- causality
- v0.4.0
- docs
dependencies:
- causality-008
assignee: developer
created: 2025-12-21T12:00:00Z
estimate: 2h
complexity: 5
area: causality
---

# Update all documentation for v0.4.0

> **AI Agent Instructions:**
> 1. Update README.md with causality tracking section
> 2. Update CHANGELOG.md with v0.4.0 changes
> 3. Update docs/ for ReadTheDocs
> 4. Ensure all examples show --dependencies usage

## Files to Update

### 1. README.md
- Add "Causality Tracking" section
- Update Quick Start to show `--dependencies`
- Add `--allow-orphan-task` to CLI reference
- Add `validate --orphans` to commands

### 2. CHANGELOG.md
```markdown
## [0.4.0] - 2025-12-21

### Added
- **Causality Tracking**: Enforce task dependencies at creation time
- `--allow-orphan-task` flag for edge cases (spikes, experiments)
- `validate --orphans` to detect orphan tasks
- CAUTION message when creating tasks without dependencies
- Soft enforcement for `import-md` (warns but succeeds)

### Changed
- `create` command now requires `--dependencies` or `--allow-orphan-task`
- Archive command shows which tasks depend on blocked tasks
- Updated AI guides with causality workflow

### Fixed
- Archive protection now shows specific dependent task IDs
```

### 3. docs/ (ReadTheDocs)
- `docs/getting-started/first-task.md` - Add dependencies to examples
- `docs/core-concepts/execution-model.md` - Add causality section
- `docs/features/task-management.md` - Document new flags
- `docs/usage-examples/common-workflows.md` - Update workflows

### 4. Version Bump
- Update `Cargo.toml` version to `0.4.0`
- Update version references in docs

## Acceptance Criteria

- [x] README.md updated with causality tracking
- [x] CHANGELOG.md has v0.4.0 entry
- [x] All docs/ examples use `--dependencies`
- [x] Version bumped to 0.4.0 in Cargo.toml
- [x] No broken links in documentation

## Session Handoff

**Completed:** 2025-12-21

**What was done:**
- Updated README.md with Causality Tracking section (Phase 6)
- Updated CHANGELOG.md with full v0.4.0 entry
- Updated Cargo.toml version to 0.4.0
- Updated docs/index.md, docs/api-reference/commands.md
- Updated docs/features/dependencies.md with full causality docs
- Updated docs/features/task-management.md
- Updated docs/getting-started/first-task.md (all examples)
- Updated docs/usage-examples/common-workflows.md (all 3 workflows)
- Updated docs/features/ai-integration.md
- Updated docs/changelog.md
- Updated INSTALL.md test command

**Files changed:** 12 documentation files updated with v0.4.0 content