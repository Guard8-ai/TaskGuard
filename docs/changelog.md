# Changelog

TaskGuard version history and release notes.

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
