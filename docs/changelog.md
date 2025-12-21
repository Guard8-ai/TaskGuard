# Changelog

TaskGuard version history and release notes.

---

## v0.4.0 (2025-12-21)

### Added
- **Causality tracking** - Every task must have dependencies to maintain semantic cause-effect chains
- **CAUTION enforcement** - `taskguard create` fails without `--dependencies` flag, shows clear guidance
- **Orphan detection** - `taskguard validate --orphans` identifies tasks with no dependencies or dependents
- **`--allow-orphan-task` flag** - Escape hatch for spikes/research tasks that truly have no dependencies
- **Archive protection messaging** - Clear feedback showing which tasks depend on blocked archive targets

### Changed
- **Create command** - Now requires `--dependencies` flag or `--allow-orphan-task` for explicit control
- **Validate command** - Added `--orphans` flag to show orphan task details
- **Root task** - `setup-001` is auto-created by `taskguard init` as the universal root

### Philosophy
TaskGuard v0.4.0 enforces causality tracking to improve AI agent workflows:

- **Semantic chains**: Tasks form cause-effect relationships, not isolated items
- **CAUTION keyword**: AI agents pay attention to CAUTION more than warnings
- **Soft enforcement**: Import warns but doesn't fail; create fails without deps

---

## v0.3.1 (2025-12-15)

### Added
- **Dependency context workflow** - Pre-flight checks prompt to read dependency task files for Session Handoff context
- **Session Handoff emphasis** - Workflow emphasizes filling Session Handoff on task completion

### Changed
- **Template pre-flight** - "Read dependency task files" as first pre-flight check
- **Workflow steps** - Added "Start" step, emphasized Session Handoff in "Complete" step

---

## v0.3.0 (2025-12-13)

### Added
- **Domain-specific task templates** - Each area (api, auth, backend, etc.) has tailored causation chain prompts and pre-flight checks
- **Custom template support** - Override via `.taskguard/templates/{area}.md`
- **GitHub integration** - Bidirectional sync with GitHub Issues and Projects v2
- **Archive/Restore commands** - Archive completed tasks (closes GitHub issues), restore (reopens)
- **Cross-branch sync** - Detects duplicate tasks across branches
- **Create command flags** - `--complexity`, `--tags`, `--dependencies`, `--assignee`, `--estimate`
- **CI/CD workflows** - GitHub Actions for testing and releases
- **Pre-built binaries** - Available for Linux, macOS, Windows, WSL

### Changed
- **Concise templates** - Reduced from ~120 to ~25 lines
- **Concise AI guide** - Reduced from ~450 to ~88 lines
- **Task ID generation** - Prevents ID reuse with archived tasks

### Fixed
- Test suite 100% pass rate
- Compiler warnings eliminated

---

## v0.2.2 (2025-10-05)

### Fixed
- Unicode processing vulnerability
- UTF-8 safe truncation in context analysis
- Proper multi-byte character boundary handling

### Security
- Enhanced security posture maintained
- All 17 security tests passing
- All 22 git analysis tests passing

---

## v0.2.1 (2025-09-30)

### Security Fixes
- **ReDoS Protection:** Bounded regex patterns with timeout protection
- **Memory Exhaustion Prevention:** Strict limits (100 task IDs, 1MB messages)
- **Path Traversal Protection:** Repository access validation
- **Input Validation:** Enhanced Unicode normalization and control character sanitization

### Testing
- ✅ 17/17 security tests passing
- ✅ 22/22 git analysis tests passing

### Improvements
- Performance optimization for large commit messages
- Improved confidence score integrity with bounds checking
- Concurrent access safety for Git operations

---

## v0.2.0 (Initial Release)

### Features
- Task creation and management
- Dependency tracking and validation
- Git integration and sync
- Quality analysis (lint)
- AI integration support
- Multi-platform support (Linux, macOS, Windows, WSL)

---

## Next Steps

See [GitHub Releases](https://github.com/Guard8-ai/TaskGuard/releases) for detailed release notes.
