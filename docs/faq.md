# Frequently Asked Questions

Common questions about TaskGuard.

---

## General

### What is TaskGuard?
TaskGuard is a local-first, Git-native task management system built in Rust.

### Why use TaskGuard?
- ✅ Local-first (no cloud)
- ✅ Git-native (version controlled)
- ✅ AI-ready (structured format)
- ✅ Dependency tracking
- ✅ Simple Markdown files

### Is it free?
Yes, TaskGuard is open source (MIT license).

---

## Technical

### What platforms are supported?
Linux, macOS, Windows, and WSL.

### Can I use it with other tools?
Yes! Tasks are plain Markdown files. Edit with any text editor, process with scripts, integrate with CI/CD.

### How do I backup my tasks?
Tasks are in Git. Every commit is a backup. Use `git tag` for milestones.

---

## Usage

### How do I share tasks with my team?
Use Git: `git push` to share, `git pull` to sync.

### Can I customize areas/statuses?
Yes, edit `.taskguard/config.toml`.

### How do dependencies work?
Tasks with dependencies are blocked until all dependencies are `done`. Use `taskguard validate` to check.

---

## Next Steps

- [Getting Started](getting-started/installation.md)
- [Core Concepts](core-concepts/task-structure.md)
