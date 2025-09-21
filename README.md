# TaskGuard

> AI-ready local task management with Git integration

TaskGuard is a local-first, Git-native task management system built in Rust. It provides intelligent task management with dependency blocking while keeping developers in complete control.

## ✨ Features

- **🏠 Local-first**: No external services, works completely offline
- **📂 Git-native**: Tasks stored as Markdown files with YAML front-matter
- **🔗 Dependency blocking**: Tasks automatically block until prerequisites are complete
- **📊 Multi-area organization**: Organize tasks by backend, frontend, auth, etc.
- **⚡ Fast & reliable**: Built in Rust for performance and safety
- **🤖 AI-ready**: Designed for natural language integration with Claude Code
- **🕰️ Git analysis**: Intelligent status suggestions based on commit history
- **🔒 Security-audited**: Comprehensive security testing with 17 security-focused tests

## 🚀 Quick Start

### Installation

```bash
# Clone and build
git clone [repository-url]
cd TaskGuard
cargo build --release

# The binary is available at target/release/taskguard
```

### Initialize a Project

```bash
# Navigate to your project
cd my-project

# Initialize TaskGuard
./taskguard init
```

### Create Your First Tasks

```bash
# Create a setup task
./taskguard create --title "Setup development environment" --area setup --priority high

# Create a backend task that depends on setup
./taskguard create --title "Implement user auth" --area backend --priority medium
```

### Add Dependencies

Edit the backend task to depend on setup:

```bash
vim tasks/backend/backend-001.md
```

Add to the YAML front-matter:
```yaml
dependencies: [setup-001]
```

### See What's Ready to Work On

```bash
# See all tasks
./taskguard list

# Check dependencies
./taskguard validate
```

## 🎯 Core Concept: Dependency Blocking

TaskGuard's key innovation is **dependency blocking** - tasks automatically become unavailable until their prerequisites are completed. This ensures work happens in the right order without manual tracking.

**Example workflow:**
1. Create foundation tasks (setup, architecture decisions)
2. Create implementation tasks that depend on foundations
3. Create testing tasks that depend on implementations
4. Use `taskguard validate` to see what's ready to work on
5. Tasks automatically become available as dependencies complete

## 📋 Task Format

Tasks are stored as Markdown files with YAML front-matter:

```markdown
---
id: auth-001
title: "Implement JWT Authentication"
status: todo
priority: high
area: backend
dependencies: [setup-001]
---

# Implement JWT Authentication

## Context
Brief description of what needs to be done and why.

## Tasks
- [ ] Install JWT library
- [ ] Create auth middleware
- [ ] Add login endpoint
- [ ] Write tests

## Acceptance Criteria
✅ **Security**: All endpoints properly authenticated
✅ **Testing**: 100% test coverage for auth flows
```

## 🔧 Commands

| Command | Description |
|---------|-------------|
| `taskguard init` | Initialize TaskGuard in a project |
| `taskguard list [--area AREA] [--status STATUS]` | List tasks with optional filters |
| `taskguard create --title TITLE [OPTIONS]` | Create a new task |
| `taskguard validate` | Check dependencies and show available tasks |
| `taskguard sync [--verbose]` | Analyze Git history for intelligent status suggestions |

## 🏗️ Project Organization

```
my-project/
├── .taskguard/
│   ├── config.toml          # Project configuration
│   └── state/               # Local state (gitignored)
├── tasks/
│   ├── setup/
│   │   ├── setup-001.md
│   │   └── setup-002.md
│   ├── backend/
│   │   └── backend-001.md
│   ├── frontend/
│   │   └── frontend-001.md
│   └── testing/
│       └── testing-001.md
└── README.md
```

## 🧠 Philosophy: "Developer is the Captain"

TaskGuard provides information and suggestions but never makes decisions for you:

- ✅ Shows which tasks are blocked and why
- ✅ Detects dependency issues and conflicts
- ✅ Suggests what to work on next
- ❌ Never automatically resolves conflicts
- ❌ Never modifies your tasks without permission
- ❌ Never hides information from you

## 🔄 Implementation Status

**✅ Phase 1 (COMPLETED): Core Foundation**
- Basic CLI with dependency validation
- Task creation, listing, and organization
- YAML + Markdown task format
- Multi-area project structure

**✅ Phase 2A (COMPLETED): Intelligence Layer**
- ✅ Git history analysis with commit correlation
- ✅ Smart status suggestions based on commit patterns
- ✅ Comprehensive security audit (17 security tests)
- ✅ `taskguard sync` command for workflow intelligence

**⏳ Phase 2B (IN PROGRESS): Enhanced Analysis**
- Task complexity analysis and linting
- Advanced conflict detection helpers

**⏳ Phase 3 (READY): Claude Code Integration**
- Natural language task management
- Context-aware suggestions
- Intelligent workflow automation

## 🤝 Contributing

TaskGuard is in active development. See [CLAUDE.md](CLAUDE.md) for detailed technical documentation.

## 📄 License

MIT License - see LICENSE file for details.

---

Built with ❤️ in Rust for developers who want to stay in control of their workflow.

## 🔒 Security

TaskGuard has undergone comprehensive security auditing with 17 security-focused tests covering:
- Regular Expression Denial of Service (ReDoS) protection
- Path traversal attack prevention
- Memory exhaustion protection
- Git repository access validation
- Input sanitization and validation

See `security-report.md` for detailed security analysis and mitigation strategies.