# Changelog

All notable changes to TaskGuard will be documented in this file.

## [0.4.0] - 2025-12-21

### Added

- **Causality tracking** - Every task must have dependencies to maintain semantic cause-effect chains
- **CAUTION enforcement** - `taskguard create` fails without `--dependencies` flag, shows clear guidance
- **Orphan detection** - `taskguard validate --orphans` identifies tasks with no dependencies or dependents
- **`--allow-orphan-task` flag** - Escape hatch for spikes/research tasks that truly have no dependencies
- **Archive protection messaging** - Clear feedback showing which tasks depend on blocked archive targets
- **Orphan detection in import** - `taskguard import-md` warns about orphan tasks after import

### Changed

- **Create command** - Now requires `--dependencies` flag or `--allow-orphan-task` for explicit control
- **Validate command** - Added `--orphans` flag to show orphan task details
- **Archive messaging** - Improved feedback when tasks can't be archived due to dependents
- **AI guides updated** - AGENTIC_AI_TASKGUARD_GUIDE.md and AI_IMPORT_MD_GUIDE.md include causality workflow

### Philosophy

TaskGuard v0.4.0 enforces causality tracking to improve AI agent workflows:
- **Semantic chains**: Tasks form cause-effect relationships, not isolated items
- **Root task**: `setup-001` is auto-created by `taskguard init` as the universal root
- **CAUTION keyword**: AI agents pay attention to CAUTION more than warnings
- **Soft enforcement**: Import warns but doesn't fail; create fails without deps

## [0.3.1] - 2025-12-15

### Added

- **Dependency context workflow** - Pre-flight checks now prompt to read dependency task files for Session Handoff context
- **Session Handoff emphasis** - Workflow updated to emphasize filling Session Handoff on task completion

### Changed

- **Template pre-flight** - Added "Read dependency task files" as first pre-flight check
- **Workflow steps** - Added "Start" step for reading dependencies, emphasized Session Handoff in "Complete" step

## [0.3.0] - 2025-12-13

### Added

- **Domain-specific task templates** - Each area (api, auth, backend, etc.) now has tailored causation chain prompts and pre-flight verification commands
- **Custom template support** - Override templates via `.taskguard/templates/{area}.md`
- **GitHub cross-branch sync** - Detects duplicate tasks across branches, adds branch/hash to issue body
- **Context section extraction** - GitHub issue descriptions now use task's Context section
- **CI/CD workflows** - Added GitHub Actions for CI testing and release automation
- **Create command flags** - `--complexity`, `--tags`, `--dependencies`, `--assignee`, `--estimate`
- **Restore command** - Restore archived tasks with automatic GitHub issue reopening
- **Archive command** - Archive completed tasks with automatic GitHub issue closing
- **GitHub-aware validation** - `taskguard validate` shows sync status and archived tasks
- **Auto-sync config areas** - Areas auto-added to config when creating tasks in new areas
- **Bulk import** - `taskguard import-md` to create tasks from markdown files

### Changed

- **Concise templates** - Reduced from ~120 to ~25 lines for better AI agent readability
- **Concise AI guide** - Reduced AGENTIC_AI_TASKGUARD_GUIDE.md from ~450 to ~88 lines
- **Removed AI notification file** - Replaced with terminal prompts for memory file updates
- **Task ID generation** - Prevents ID reuse when archived tasks exist

### Fixed

- Premature commit behavior in AI agents
- Unused imports and compiler warnings
- Test suite achieving 100% pass rate

## [0.2.2] - Previous Release

- Core CLI with dependency validation
- Task creation, listing, and organization
- YAML + Markdown task format
- Git history analysis
- Security audit (17 tests)
