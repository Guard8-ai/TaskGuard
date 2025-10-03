# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

TaskGuard is a local-first, Git-native task management system built in Rust. It provides AI-ready task management with automatic Claude Code integration, zero-setup agent collaboration, and intelligent dependency blocking while keeping developers in control.

## Build and Installation

### Prerequisites
- Rust 1.70+ (install from https://rustup.rs/)
- Git (for version control integration)

### Building from Source
```bash
# Clone the repository
git clone [repository-url]
cd TaskGuard

# Build the project
cargo build --release

# The binary will be available at target/release/taskguard
```

### Installation
```bash
# Install globally (requires Rust/Cargo)
# This installs to ~/.cargo/bin/ which is automatically added to PATH
cargo install --path . --locked

# Or use the binary directly
./target/release/taskguard --help
```

## Quick Start

Get started with TaskGuard in under 5 minutes:

### 1. Initialize a Project
```bash
# Navigate to your project directory
cd my-project

# Initialize TaskGuard
taskguard init
```

### 2. Create Your First Task
```bash
# Create a setup task
taskguard create --title "Setup development environment" --area setup --priority high

# Create a backend task
taskguard create --title "Implement user authentication" --area backend --priority medium
```

### 3. View Your Tasks
```bash
# See all tasks
taskguard list

# Filter by area
taskguard list --area setup
```

### 4. Create Dependencies
Edit a task file to add dependencies:
```bash
# Edit the auth task to depend on setup
vim tasks/backend/backend-001.md
```

Add to the YAML front-matter:
```yaml
dependencies: [setup-001]
```

### 5. Validate Dependencies
```bash
# Check which tasks are available to work on
taskguard validate
```

### 6. Start Working
- Tasks without dependencies are immediately available
- Dependent tasks are blocked until prerequisites are complete
- Edit task files directly to update content and track progress

## Core Philosophy

TaskGuard follows the "Developer is the Captain" philosophy - it provides information, suggests actions, and automates boring tasks, but never makes decisions for you. When conflicts arise, it surfaces them clearly with options, not automated "fixes."

## Architecture

### Technology Stack
- **Rust**: Single binary, fast, reliable, excellent Git integration
- **Git**: Natural persistence layer, collaboration, history tracking
- **Markdown**: Human-readable task format with YAML front-matter
- **Claude Code Commands**: Natural language task management

### File Structure
```
project/
├── .taskguard/
│   ├── config.toml          # Project configuration
│   ├── templates/           # Task templates
│   └── state/               # Local state (gitignored)
├── tasks/
│   ├── setup/
│   ├── auth/
│   ├── api/
│   └── [other areas]/
└── README.md
```

## Task File Format

Tasks are stored as Markdown files with YAML front-matter in the `tasks/` directory, organized by area.

### Required Fields
```yaml
---
id: backend-001              # Auto-generated: area-number format
title: "Task Title"          # Human-readable task name
area: backend                # Area/category (backend, frontend, auth, etc.)
---
```

### Optional Fields
```yaml
---
status: todo                 # todo, doing, review, done, blocked (default: todo)
priority: medium             # low, medium, high, critical (default: medium)
tags: [backend, api]         # List of tags for categorization
dependencies: [setup-001]    # List of task IDs this task depends on
assignee: developer          # Who is working on this task
created: 2025-01-15T10:00:00Z # Auto-generated timestamp
estimate: 4h                 # Time estimate
complexity: 6                # Complexity rating (1-10)
---
```

### Complete Example
```markdown
---
id: auth-001
title: "Implement JWT Authentication"
status: todo
priority: high
tags: [backend, security, auth]
dependencies: [setup-001]
assignee: developer
created: 2025-01-15T10:00:00Z
estimate: 4h
complexity: 6
area: auth
---

# Implement JWT Authentication

## Context
The application needs secure authentication using JWT tokens for API access.
Current state: Basic Express server is running, need to add auth layer.

## Objectives
- Implement JWT token generation and validation
- Create secure login endpoint
- Add middleware for protected routes
- Ensure proper error handling

## Tasks
- [ ] Install jsonwebtoken and bcrypt packages
- [ ] Create auth middleware function
- [ ] Implement POST /auth/login endpoint
- [ ] Add token validation middleware
- [ ] Write unit tests for auth functions

## Acceptance Criteria
✅ **Login Success:**
- User can login with valid email/password
- Server returns valid JWT token
- Token includes user ID and role

✅ **Security:**
- Passwords are hashed with bcrypt
- JWT tokens expire after 24 hours
- Invalid tokens are rejected with 401

## Technical Notes
- Use RS256 algorithm for JWT signing
- Store JWT secret in environment variables
- Consider refresh token strategy for production

## Updates
- 2025-01-15: Task created
- 2025-01-16: Started implementation
- 2025-01-17: Completed login endpoint, testing in progress
```

### File Organization
```
tasks/
├── setup/
│   ├── setup-001.md
│   └── setup-002.md
├── backend/
│   ├── backend-001.md
│   └── backend-002.md
├── frontend/
│   └── frontend-001.md
└── auth/
    └── auth-001.md
```

## Dependency Management

TaskGuard provides powerful dependency blocking to ensure tasks are completed in the correct order.

### Basic Dependencies

Add dependencies to the YAML front-matter:

```yaml
---
id: api-001
title: "User API Endpoints"
dependencies: [auth-001]  # This task depends on auth-001 being completed
area: api
---
```

### Multiple Dependencies

```yaml
---
id: integration-tests
title: "End-to-End Integration Tests"
dependencies: [api-001, auth-001, frontend-001]  # Depends on multiple tasks
area: testing
---
```

### Dependency Validation

Use `taskguard validate` to see dependency status:

```bash
$ taskguard validate

🚦 TASK STATUS
   ✅ Available tasks (dependencies satisfied):
      ⭕ auth-001 - Implement JWT Authentication
      ⭕ setup-001 - Project Setup

   🚫 Blocked tasks:
      ❌ api-001 - User API Endpoints (waiting for: auth-001)
      ❌ integration-tests - End-to-End Tests (waiting for: api-001, auth-001, frontend-001)

✅ VALIDATION PASSED
   No issues found in 4 tasks

📊 SUMMARY
   Total tasks: 4
   Available: 2
   Blocked: 2
```

### Dependency Workflow

1. **Create foundation tasks** without dependencies
2. **Add dependent tasks** that build on completed work
3. **Use `taskguard validate`** to see what's ready to work on
4. **Complete tasks in dependency order** - blocked tasks automatically become available

### Common Dependency Patterns

**Setup → Implementation → Testing:**
```yaml
# setup-001.md
dependencies: []  # No dependencies

# backend-001.md
dependencies: [setup-001]  # Depends on setup

# testing-001.md
dependencies: [backend-001]  # Depends on implementation
```

**Parallel Development:**
```yaml
# frontend-001.md
dependencies: [setup-001]  # Can work in parallel with backend

# backend-001.md
dependencies: [setup-001]  # Can work in parallel with frontend

# integration-001.md
dependencies: [frontend-001, backend-001]  # Needs both complete
```

### Error Detection

TaskGuard detects common dependency issues:

- **Missing dependencies**: References to non-existent tasks
- **Circular dependencies**: Task A depends on B, B depends on A
- **Broken references**: Dependencies on deleted or renamed tasks

## Development Commands

### Basic Commands (Implemented)

#### `taskguard init`
Initialize TaskGuard in a project directory.

```bash
$ taskguard init
🚀 Initializing TaskGuard...
📝 Created example task: tasks/setup/001-project-setup.md
✅ TaskGuard initialized successfully!

📁 Created directories:
   .taskguard/         # Configuration and state
   tasks/              # Task files organized by area
   tasks/setup/
   tasks/backend/
   tasks/frontend/
   tasks/api/
   tasks/auth/
   tasks/testing/
```

#### `taskguard list [OPTIONS]`
List all tasks with optional filtering.

```bash
# List all tasks
$ taskguard list

📁 BACKEND
   ───────────
   ⭕ 🟠 api-001 User API Endpoints
      └── Depends on: simple-001
   ⭕ 🟡 backend-001 Test Task

📁 FRONTEND
   ────────────
   ⭕ 🔴 frontend-001 Frontend Login

📊 SUMMARY
   Total tasks: 3
   todo: 3

# Filter by area
$ taskguard list --area backend

# Filter by status
$ taskguard list --status todo
```

**Options:**
- `--area AREA` - Filter tasks by area (backend, frontend, auth, etc.)
- `--status STATUS` - Filter tasks by status (todo, doing, review, done, blocked)

#### `taskguard create`
Create a new task.

```bash
$ taskguard create --title "Implement JWT Auth" --area backend --priority high
✅ Created task: backend/backend-002.md
   ID: backend-002
   Title: Implement JWT Auth
   Area: backend
   Priority: high

📝 Next steps:
   taskguard show backend-002  # View task details
   Edit the file to add more details
```

**Options:**
- `--title TITLE` - Task title (required)
- `--area AREA` - Task area (default: setup)
- `--priority PRIORITY` - Priority level (low, medium, high, critical)

#### `taskguard validate`
Check for dependency issues and conflicts.

```bash
$ taskguard validate

🚦 TASK STATUS
   ✅ Available tasks (dependencies satisfied):
      ⭕ simple-001 - Simple Test Task
      ⭕ backend-001 - Test Task

   🚫 Blocked tasks:
      ❌ api-001 - User API Endpoints (waiting for: simple-001)

✅ VALIDATION PASSED
   No issues found in 3 tasks

📊 SUMMARY
   Total tasks: 3
   Available: 2
   Blocked: 1
   Parse errors: 0
   Dependency issues: 0
```

### Commands (To Be Implemented)
- `taskguard show <task-id>` - Show detailed task information
- `taskguard edit <task-id>` - Edit an existing task
- `taskguard status` - Show project task status overview
- `taskguard lint` - Analyze task complexity and suggest improvements
- `taskguard sync` - Analyze git activity for status suggestions

## Configuration

TaskGuard configuration is stored in `.taskguard/config.toml`. The configuration is automatically created with sensible defaults when you run `taskguard init`.

### Default Configuration

```toml
[project]
name = "My Project"
version = "0.2.2"
areas = ["setup", "backend", "frontend", "api", "auth", "testing", "deployment"]

[settings]
statuses = ["todo", "doing", "review", "done", "blocked"]
priorities = ["low", "medium", "high", "critical"]
complexity_scale = "1-10"
default_estimate_unit = "hours"

[git]
auto_add_tasks = true
auto_commit_on_status_change = false
commit_message_template = "Task {{id}}: {{action}} - {{title}}"

[ai]
enabled = true
claude_code_integration = true
auto_suggestions = true
complexity_analysis = true
```

### Configuration Options

#### `[project]` Section
- **`name`**: Project name for display purposes
- **`version`**: Project version
- **`areas`**: List of available task areas for organization

#### `[settings]` Section
- **`statuses`**: Available task statuses (used for validation)
- **`priorities`**: Available priority levels
- **`complexity_scale`**: Scale for task complexity (e.g., "1-10")
- **`default_estimate_unit`**: Default unit for time estimates

#### `[git]` Section (Future Features)
- **`auto_add_tasks`**: Automatically stage task files when modified
- **`auto_commit_on_status_change`**: Auto-commit when task status changes
- **`commit_message_template`**: Template for automatic commit messages

#### `[ai]` Section (Future Features)
- **`enabled`**: Enable AI-powered features
- **`claude_code_integration`**: Enable Claude Code natural language integration
- **`auto_suggestions`**: Enable automatic task suggestions
- **`complexity_analysis`**: Enable task complexity analysis

### Customizing Areas

You can customize the available areas for your project:

```toml
[project]
areas = ["planning", "design", "backend", "frontend", "mobile", "testing", "deployment", "documentation"]
```

Areas determine:
- Available subdirectories in `tasks/`
- Options for the `--area` flag in `taskguard create`
- Organization in `taskguard list` output

### Customizing Statuses and Priorities

```toml
[settings]
statuses = ["backlog", "todo", "in-progress", "review", "testing", "done"]
priorities = ["p0", "p1", "p2", "p3"]
```

**Note**: Changing these requires updating existing task files to use the new values.

## Claude Code Integration

TaskGuard v0.2.2 provides zero-setup AI agent integration with automatic guide distribution:

### 🤖 Automatic AI Integration (v0.2.2)
**TaskGuard now handles AI setup automatically:**
1. ✅ **Auto-copies** `AGENTIC_AI_TASKGUARD_GUIDE.md` to your project root during `taskguard init`
2. ✅ **Auto-creates** `AI_AGENT_SETUP_NOTIFICATION.md` with tool integration instructions
3. ✅ **Built-in notification system** prompts you to update AI tool instruction files
4. ✅ **Zero manual setup** required for AI agent collaboration

**Simply run `taskguard init` and follow the notification prompts!**

### 📖 Complete AI Collaboration Guide
**For optimal AI collaboration, see: `AGENTIC_AI_TASKGUARD_GUIDE.md`** (automatically distributed)

### Natural Language Commands
Instead of CLI syntax, use conversational commands:
- "Create a new high-priority task for setting up the database connection"
- "Show me everything that's ready to work on"
- "I just finished the authentication work, what should I work on next?"

### Context-Aware Features
- Git history analysis for status suggestions
- Dependency validation and conflict detection
- Task complexity analysis and breakdown suggestions
- Smart task creation from natural descriptions

## Configuration

Configuration is stored in `.taskguard/config.toml`:

```toml
[project]
name = "Project Name"
areas = ["setup", "backend", "frontend", "api", "auth", "testing"]

[settings]
statuses = ["todo", "doing", "review", "done", "blocked"]
priorities = ["low", "medium", "high", "critical"]

[ai]
enabled = true
claude_code_integration = true
auto_suggestions = true
```

## Implementation Status

**Phase 1 (COMPLETED)**: Core Foundation
- ✅ Basic CLI with clap
- ✅ Task file parsing (YAML + Markdown)
- ✅ Project initialization
- ✅ Task creation and listing
- ✅ Dependency validation and blocking
- ✅ Multi-area task organization

**Phase 2 (COMPLETED)**: Intelligence Layer
- ✅ Git analysis and smart suggestions
- ✅ Task complexity analysis
- ✅ Conflict detection and resolution

**Phase 3 (COMPLETED)**: AI Integration
- ✅ Automatic AI guide distribution
- ✅ Zero-setup agent collaboration
- ✅ Built-in notification system for tool integration
- ✅ Context-aware suggestions

**Phase 4 (COMPLETED)**: Distribution & Polish
- ✅ Cross-platform installation scripts
- ✅ Security audit (17 security tests)
- ✅ Comprehensive documentation

## Key Design Principles

- **Local-first**: No external service dependencies
- **Git-native**: Built around Git workflows
- **Developer control**: Present options, never make automatic decisions
- **Conflict transparency**: Surface conflicts clearly with resolution options
- **Performance**: Fast operations on hundreds of task files
- **Reliability**: Graceful error handling, never lose data

## Security Considerations

TaskGuard has been security-audited with comprehensive testing for defensive security posture. Key security features and considerations:

### Git Repository Security

- **Path Validation**: Repository access is restricted to prevent path traversal attacks
- **Safe Git Operations**: All Git operations use the secure `git2` crate with proper error handling
- **Repository Isolation**: TaskGuard operates only within the current project scope

### Input Processing Security (v0.2.2+ Security Improvements)

- **Commit Message Safety**: Commit messages are processed securely without executing embedded commands
- **ReDoS Protection**: Enhanced regex patterns with timeout protection and bounded processing to prevent Regular Expression Denial of Service attacks
- **Memory Limits**: Task ID extraction limited to 100 IDs per message with 1MB message size limit to prevent memory exhaustion
- **Input Validation**: Comprehensive size limits, character validation, and false positive detection for version strings
- **Path Traversal Protection**: Repository access validation prevents malicious path traversal attacks

### Memory and Performance Security (Enhanced v0.2.2)

- **Bounded Processing**: Task ID extraction and commit processing have strict limits (100 task IDs, 1MB messages)
- **Performance Optimization**: UTF-8 safe context analysis with fast byte-level operations for large messages
- **Algorithmic Complexity Protection**: All operations tested against complexity attacks with sub-100ms processing guarantees
- **Concurrent Safety**: Thread-safe operations for multiple concurrent Git access with comprehensive testing

### Error Handling Security

- **Information Disclosure Protection**: Error messages are sanitized to prevent leaking sensitive paths
- **Graceful Failure**: Proper error handling prevents crashes and unexpected behavior
- **Security Logging**: Important security events are properly logged for monitoring

### Production Deployment Guidelines

When using TaskGuard in production environments:

1. **Repository Access**: Ensure TaskGuard runs with minimal necessary file system permissions
2. **Input Sources**: Validate any external input sources (if integrating with other tools)
3. **Monitoring**: Monitor for unusual patterns in commit processing or task analysis
4. **Updates**: Keep dependencies updated and run security audits regularly

### Security Testing (Enhanced Coverage v0.2.2)

TaskGuard includes comprehensive security tests covering:
- **ReDoS Prevention**: Regex patterns tested against catastrophic backtracking with processing time limits
- **Memory Exhaustion Protection**: Task ID extraction bounds testing with 100K+ malicious input patterns
- **Path Traversal Prevention**: Repository access validation against system directory attacks
- **Malicious Input Handling**: Commit message injection testing with embedded commands and control characters
- **Concurrent Access Safety**: Multi-threaded Git operations testing for race conditions
- **Unicode Safety**: Proper UTF-8 handling with multi-byte character boundary protection
- **Confidence Score Integrity**: Status suggestion bounds checking to prevent overflow conditions

**Security Test Results**: 17/17 tests passing ✅

Run security tests with:
```bash
cargo test security_tests -- --nocapture
```

**Git Analysis Test Results**: 22/22 tests passing ✅

Run git analysis tests with:
```bash
cargo test --test git_analysis_tests -- --nocapture
```

**Security Audit**: TaskGuard has undergone comprehensive security auditing with **ALL vulnerabilities (High, Medium, and Low priority) fully resolved**.

**Final Security Status (v0.2.2+)**:
- ✅ **EXCELLENT Security Posture** - All identified vulnerabilities resolved
- ✅ **17/17 security tests passing** - Comprehensive attack scenario coverage
- ✅ **22/22 git analysis tests passing** - Robust functionality validation
- ✅ **Path Traversal Protection** - Strict validation prevents malicious access
- ✅ **Memory Exhaustion Prevention** - Streaming with 10MB memory limits and 1000 commit bounds
- ✅ **Input Validation** - Unicode normalization and control character sanitization
- ✅ **ReDoS Protection** - Safe regex patterns with timeout protection (sub-100ms processing)
- ✅ **Confidence Score Integrity** - Proper bounds checking prevents overflow
- ✅ **Concurrent Access Safety** - Thread-safe Git operations with comprehensive testing

See `security-report.md` for detailed findings and complete remediation status.

## Troubleshooting

### Common Issues

#### "Not in a TaskGuard project" Error
```bash
❌ Error: Not in a TaskGuard project. Run 'taskguard init' first.
```
**Solution**: You're not in a directory with TaskGuard initialized. Run `taskguard init` or navigate to a directory that has a `.taskguard/` folder.

#### Task Files Not Parsing
```bash
⚠️  Skipping /path/to/task.md: Failed to parse YAML front-matter
```
**Common causes**:
- Missing required fields (`id`, `title`, `area`)
- Invalid YAML syntax
- Missing `---` delimiters around front-matter

**Solution**: Check your YAML format:
```yaml
---
id: task-001
title: "Task Title"
area: backend
---
```

#### Tasks Not Showing in List
- Ensure task files are in the `tasks/` directory
- Check that files have `.md` extension
- Verify YAML front-matter is valid
- Use `taskguard validate` to see parse errors

#### Dependencies Not Working
```bash
❌ api-001: Depends on missing task 'nonexistent-task'
```
**Solution**:
- Check dependency task IDs exist
- Ensure task IDs match exactly (case-sensitive)
- Use `taskguard validate` to find broken references

#### Build Errors
```bash
error: failed to resolve: use of unresolved module
```
**Solution**:
- Ensure Rust 1.70+ is installed
- Run `cargo clean && cargo build`
- Check all dependencies in Cargo.toml

### Performance Issues

#### Slow Task Loading
- Large numbers of task files (>1000) may slow operations
- Consider organizing into more specific areas
- Use `--area` filters to limit scope

#### Large Task Files
- Keep task files focused and under 200 lines
- Break complex tasks into smaller subtasks
- Use separate documentation files for detailed specs

### Debugging

#### Enable Verbose Output
```bash
RUST_LOG=debug taskguard list
```

#### Check File Permissions
Ensure TaskGuard can read/write in your project directory:
```bash
ls -la .taskguard/
ls -la tasks/
```

#### Validate Configuration
```bash
cat .taskguard/config.toml
```