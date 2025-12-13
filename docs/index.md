# TaskGuard Documentation

**AI-ready local task management with Git integration**

---

## 🎯 What is TaskGuard?

TaskGuard is a **local-first, Git-native task management system** built in Rust that provides:

- 📋 **Simple task management** - Markdown files with YAML metadata
- 🔗 **Dependency tracking** - Automatic blocking based on task dependencies
- 🤖 **AI collaboration** - Zero-setup integration with Claude Code and other LLMs
- 🔒 **Git-native** - All tasks stored in version control
- ⚡ **Fast & secure** - Rust-powered with comprehensive security testing

---

## ✨ Key Features

<div class="grid cards" markdown>

- 🚀 **Quick Setup**

    Initialize in seconds with `taskguard init`. No configuration required.

- 📦 **Local-First**

    All data stays on your machine. No cloud dependencies.

- 🔄 **Git Integration**

    Analyze commit history and suggest task status updates automatically.

- 🤖 **AI-Ready**

    Structured format perfect for LLM consumption and automation.

- 🔐 **Security Tested**

    17/17 security tests passing. Production-ready.

- ⚡ **Zero Dependencies**

    Single binary. No runtime requirements besides Git.

</div>

---

## 🚀 Quick Start

Get started with TaskGuard in under 5 minutes:

### 1. Installation

=== "macOS"

    ```bash
    git clone git@github.com:Guard8-ai/TaskGuard.git
    cd TaskGuard
    ./scripts/install-macos.sh
    ```

=== "Linux"

    ```bash
    git clone git@github.com:Guard8-ai/TaskGuard.git
    cd TaskGuard
    ./scripts/install-linux.sh
    ```

=== "Windows"

    ```powershell
    git clone git@github.com:Guard8-ai/TaskGuard.git
    cd TaskGuard
    .\scripts\install-windows.ps1
    ```

### 2. Initialize Project

```bash
cd ~/my-project
taskguard init
```

### 3. Create Your First Task

```bash
taskguard create --title "Setup database" --area backend --priority high
```

### 4. View Tasks

```bash
taskguard list
```

**Output:**
```
📁 BACKEND
   ───────────
   ⭕ 🟠 backend-001 Setup database

📊 SUMMARY
   Total tasks: 1
   todo: 1
```

---

## 📖 Navigation Guide

<div class="grid cards" markdown>

- **Getting Started**

    New to TaskGuard? Start here for installation and your first task.

    [:octicons-arrow-right-24: Get Started](getting-started/prerequisites.md)

- **Core Concepts**

    Learn about task structure, dependencies, and state management.

    [:octicons-arrow-right-24: Core Concepts](core-concepts/task-structure.md)

- **Features**

    Explore TaskGuard's powerful features and capabilities.

    [:octicons-arrow-right-24: Features](features/task-management.md)

- **Usage Examples**

    See real-world workflows and integration patterns.

    [:octicons-arrow-right-24: Examples](usage-examples/common-workflows.md)

- **API Reference**

    Complete command reference and configuration documentation.

    [:octicons-arrow-right-24: API Docs](api-reference/commands.md)

- **Contributing**

    Help improve TaskGuard. Development setup and guidelines.

    [:octicons-arrow-right-24: Contribute](contributing/development-setup.md)

</div>

---

## 🎯 Why TaskGuard?

### Local-First Philosophy

Your tasks stay on your machine. No cloud sync, no vendor lock-in, complete control.

### Git-Native Design

Tasks are just Markdown files in a Git repo. Version control, collaboration, and history tracking built-in.

### AI Collaboration

Structured YAML + Markdown format makes tasks readable by both humans and LLMs. Zero-setup integration with Claude Code.

### Developer Control

TaskGuard suggests, never decides. You're always in control of your workflow.

---

## 📊 At a Glance

| Feature | Status |
|---------|--------|
| **Task Management** | ✅ Create, list, update tasks |
| **Dependencies** | ✅ Automatic blocking & validation |
| **Git Sync** | ✅ Commit analysis & suggestions |
| **Quality Analysis** | ✅ Complexity scoring & linting |
| **AI Integration** | ✅ Claude Code, natural language |
| **Security** | ✅ 17/17 tests passing |
| **Platforms** | ✅ Linux, macOS, Windows, WSL |

---

## 🔗 Quick Links

- **GitHub:** [Guard8-ai/TaskGuard](https://github.com/Guard8-ai/TaskGuard)
- **Issues:** [Report a bug](https://github.com/Guard8-ai/TaskGuard/issues)
- **License:** MIT
- **Version:** 0.2.2

---

## 💡 Example Use Cases

### Solo Developer
```bash
# Track personal project tasks
taskguard create --title "Build API endpoint" --area backend
taskguard validate  # See what's ready to work on
```

### Team Collaboration
```bash
# Tasks in Git - share via pull requests
git add tasks/
git commit -m "Add authentication tasks"
git push
```

### AI-Assisted Development
```bash
# Let AI help manage tasks
taskguard ai "create tasks for user authentication feature"
taskguard sync  # AI analyzes git commits
```

---

## 🚀 Next Steps

Ready to dive in?

1. [Install TaskGuard](getting-started/installation.md)
2. [Complete the 5-minute tutorial](getting-started/first-task.md)
3. [Learn core concepts](core-concepts/task-structure.md)
4. [Explore features](features/task-management.md)

---

**Questions?** Check the [FAQ](faq.md) or [open an issue](https://github.com/Guard8-ai/TaskGuard/issues).
