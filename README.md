# TaskGuard

> AI-ready local task management with Git integration

[![Documentation](https://readthedocs.org/projects/taskguard/badge/?version=latest)](https://taskguard.readthedocs.io/en/latest/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

TaskGuard is a local-first, Git-native task management system built in Rust. It provides AI-ready task management with automatic agent integration, dependency blocking, and intelligent workflows while keeping developers in complete control.

**[📖 Full Documentation](https://taskguard.readthedocs.io)**

## ✨ Features

- **🏠 Local-first**: No external services, works completely offline
- **📂 Git-native**: Tasks stored as Markdown files with YAML front-matter
- **🔗 Dependency blocking**: Tasks automatically block until prerequisites are complete
- **📊 Multi-area organization**: Organize tasks by backend, frontend, auth, etc.
- **⚡ Fast & reliable**: Built in Rust for performance and safety
- **🤖 AI-ready**: Zero-setup AI agent integration with automatic guide distribution
- **🕰️ Git analysis**: Intelligent status suggestions based on commit history
- **🔒 Security-audited**: Comprehensive security testing with 17 security-focused tests

## 🚀 Quick Start

### Installation

TaskGuard can be installed globally to work with all your projects.

**Global Installation (Recommended):**

```bash
# Clone the repository
git clone https://github.com/Guard8-ai/TaskGuard.git
cd TaskGuard

# Run platform-specific installation script
./scripts/install-linux.sh     # Linux
./scripts/install-macos.sh     # macOS
./scripts/install-wsl.sh       # WSL/WSL2
```

**Windows (PowerShell):**
```powershell
.\scripts\install-windows.ps1
```

**Manual Build:**
```bash
# Clone and build
git clone https://github.com/Guard8-ai/TaskGuard.git
cd TaskGuard
cargo build --release

# The binary is available at target/release/taskguard
```

See [INSTALL.md](INSTALL.md) for detailed installation instructions and troubleshooting.

### Initialize a Project

```bash
# Navigate to your project
cd my-project

# Initialize TaskGuard (works globally after installation)
taskguard init
```

**🤖 Zero-Setup AI Integration**: TaskGuard automatically creates AI collaboration files when initialized:
- `AGENTIC_AI_TASKGUARD_GUIDE.md` - Complete guide for AI agents with best practices
- `AI_AGENT_SETUP_NOTIFICATION.md` - Automatic setup instructions for tool integration

**For AI agents**: TaskGuard automatically distributes integration guides and provides notification system for updating tool instruction files (CLAUDE.md, GEMINI.md, etc.) with zero manual setup required.

### Create Your First Tasks

```bash
# Create a setup task
taskguard create --title "Setup development environment" --area setup --priority high

# Create a backend task that depends on setup
taskguard create --title "Implement user auth" --area backend --priority medium
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
taskguard list

# Check dependencies and see what's available
taskguard validate

# Get AI recommendations
taskguard ai "what should I work on next?"

# Analyze Git activity for status suggestions
taskguard sync --verbose
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

### Core Commands
| Command | Description |
|---------|-------------|
| `taskguard init` | Initialize TaskGuard in a project |
| `taskguard list [--area AREA] [--status STATUS]` | List tasks with optional filters |
| `taskguard create --title TITLE [OPTIONS]` | Create a new task |
| `taskguard validate` | Check dependencies and show available tasks |

### Intelligence Commands
| Command | Description |
|---------|-------------|
| `taskguard sync [--verbose]` | Analyze Git history for intelligent status suggestions |
| `taskguard lint [--verbose]` | Analyze task complexity and quality |
| `taskguard ai "QUERY"` | Natural language task management with AI |

### Installation Commands
| Platform | Command |
|----------|---------|
| Linux | `./scripts/install-linux.sh` |
| macOS | `./scripts/install-macos.sh` |
| Windows | `.\scripts\install-windows.ps1` |
| WSL/WSL2 | `./scripts/install-wsl.sh` |

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

**✅ Phase 2 (COMPLETED): Intelligence Layer**
- ✅ Git history analysis with commit correlation (`taskguard sync`)
- ✅ Smart status suggestions based on commit patterns
- ✅ Task complexity analysis and linting (`taskguard lint`)
- ✅ Comprehensive security audit (17 security tests)

**✅ Phase 3 (COMPLETED): AI Integration**
- ✅ Natural language task management (`taskguard ai`)
- ✅ Context-aware suggestions and recommendations
- ✅ Claude Code integration for intelligent workflow automation

**✅ Phase 4 (COMPLETED): Distribution**
- ✅ Cross-platform installation scripts (Linux, macOS, Windows, WSL)
- ✅ Global installation for multi-project usage
- ✅ Comprehensive documentation and guides

## 🤖 For AI Agents & Automation

TaskGuard is designed to work seamlessly with agentic AI systems like Claude Code. If you're building AI agents that need to manage tasks systematically:

**📖 [Agentic AI TaskGuard Guide](AGENTIC_AI_TASKGUARD_GUIDE.md)**

This comprehensive guide covers:
- Common pitfalls when AI agents use TaskGuard
- Proper task distribution across areas to avoid ID conflicts
- Tool hygiene practices for reliable dependency management
- Step-by-step workflows for AI-driven task breakdown
- Debugging strategies for complex task hierarchies

Key insight: AI agents must **respect TaskGuard's design patterns** rather than trying to bulldoze through with rapid commands.

## 📚 Documentation

Complete documentation is available at **[taskguard.readthedocs.io](https://taskguard.readthedocs.io)**:

- [Getting Started Guide](https://taskguard.readthedocs.io/en/latest/getting-started/installation/)
- [Core Concepts](https://taskguard.readthedocs.io/en/latest/core-concepts/task-structure/)
- [API Reference](https://taskguard.readthedocs.io/en/latest/api-reference/commands/)
- [Contributing Guidelines](https://taskguard.readthedocs.io/en/latest/contributing/development-setup/)

## 🤝 Contributing

TaskGuard is in active development. See [CLAUDE.md](CLAUDE.md) for detailed technical documentation or visit the [contributing section](https://taskguard.readthedocs.io/en/latest/contributing/development-setup/) in our documentation.

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