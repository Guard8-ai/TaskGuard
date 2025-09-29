# TaskGuard - AI-Ready Local Task Management

## Vision Statement
TaskGuard is a local-first, Git-native task management system that keeps developers in control while providing intelligent assistance. Built in Rust for reliability and performance, with zero-setup AI agent integration and automatic guide distribution for Claude Code workflows.

## Core Philosophy: "Developer is the Captain"

TaskGuard provides information, suggests actions, and automates boring tasks - but never makes decisions for you. When conflicts arise, we surface them clearly with options, not automated "fixes."

---

## Architecture Overview

### Technology Stack
- **Rust**: Single binary, fast, reliable, excellent Git integration
- **Git**: Natural persistence layer, collaboration, history tracking  
- **Markdown**: Human-readable task format with YAML front-matter
- **Claude Code Commands**: Natural language task management

### File Structure
```
my-project/
├── .taskguard/
│   ├── config.toml          # Project configuration
│   ├── templates/           # Task templates
│   └── state/               # Local state (gitignored)
├── tasks/
│   ├── setup/
│   │   ├── 001-project-init.md
│   │   └── 002-dependencies.md
│   ├── auth/
│   │   ├── 001-jwt-setup.md
│   │   └── 002-login-endpoint.md
│   └── api/
│       └── 001-user-endpoints.md
├── .gitignore               # Includes .taskguard/state/
└── README.md
```

---

## Task File Format

```markdown
---
id: auth-jwt-001
title: "Implement JWT Authentication"
status: todo
priority: high
tags: [backend, security, auth]
dependencies: [setup-001, setup-002]
assignee: developer
created: 2025-01-15T10:00:00Z
estimate: 4h
complexity: 6
area: auth
---

# JWT Authentication Implementation

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
- [ ] Create user model/schema
- [ ] Write unit tests for auth functions
- [ ] Write integration tests for endpoints
- [ ] Update API documentation

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

---

## Problems & Solutions Framework

### 🔴 Problem 1: Git Conflicts in Tasks
**Scenario:** Two developers modify the same task file simultaneously.

**TaskGuard Response:**
```bash
$ taskguard status
⚠️  Conflict detected in tasks/auth/001-jwt.md
├── Your changes: status todo → doing
├── Remote changes: added acceptance criteria  
├── Last common version: 2 hours ago
└── Actions:
    [1] Open merge tool (code --diff)
    [2] Show detailed diff
    [3] Keep my version
    [4] Keep remote version
    [5] Manual resolve (opens editor)
    [6] Mark as conflicted (for later)

Choice [1-6]: _
```

**Philosophy:** Present clear options, let developer decide. No automatic merging.

### 🔴 Problem 2: Broken Dependencies  
**Scenario:** Task A depends on Task B, but B was deleted or renamed.

**TaskGuard Response:**
```bash
$ taskguard validate
❌ Dependency issues found:

auth-001.md:
├── Depends on: setup-003 (❌ missing)
├── Depends on: setup-001 (✅ exists)
└── Blocking: api-002, api-003

api-002.md:  
├── Depends on: auth-999 (❌ invalid ID)
└── Suggested fix: auth-001?

Actions:
[1] Show dependency graph
[2] Fix broken links interactively  
[3] Remove all broken dependencies
[4] Export report for review
[5] Ignore (I'll fix manually)
```

### 🔴 Problem 3: Large Unwieldy Tasks
**Scenario:** Single task file becomes too large/complex.

**TaskGuard Response:**
```bash
$ taskguard lint  
⚠️  Task complexity analysis:

tasks/api-refactor.md:
├── Size: 847 lines (⚠️  large)
├── Checkboxes: 47 items (⚠️  too many)  
├── Estimated effort: 2-3 weeks (⚠️  epic-sized)
└── Complexity score: 9/10

Suggestions:
[1] Auto-suggest breakdown points
[2] Convert to epic with subtasks
[3] Move implementation details to separate docs
[4] Show similar tasks for reference
[5] This is fine (mark as epic)
```

### 🔴 Problem 4: Status Desynchronization
**Scenario:** Developer forgets to update task status after completing work.

**TaskGuard Response:**
```bash
$ taskguard sync
🔍 Analyzing git activity...

Detected potential status updates:
├── auth-001.md: Last 3 commits suggest completion
│   └── "Add JWT auth tests", "Fix auth middleware", "Auth docs updated"  
├── api-002.md: Active branch detected (feature/user-api)
│   └── Recent commits in progress
└── setup-001.md: No recent activity (stale?)

Suggestions:
✅ Mark auth-001 as done? [Y/n]: _
⏳ Mark api-002 as in-progress? [Y/n]: _  
❓ Review setup-001 status? [y/N]: _
🚫 Skip auto-suggestions [s]: _
```

---

## Claude Code Integration

### Natural Language Commands
Instead of remembering CLI syntax, use natural language:

```bash
# Traditional CLI:
$ taskguard create --title="Setup database" --area=backend --priority=high

# Claude Code Integration:
> "Create a new high-priority task for setting up the database connection"

# TaskGuard responds with:
✅ Created task: backend-003-database-setup.md
├── Area: backend  
├── Priority: high
├── Template: backend-service
└── Next: Edit task details? [Y/n]
```

### Context-Aware Workflows
```bash
> "I just finished the authentication work, what should I work on next?"

TaskGuard Analysis:
🔍 Checking git history for auth-related work...
├── auth-001: Recent commits detected
├── auth-002: Status still 'doing'  
└── Dependencies now unblocked: api-001, api-002

Recommendations:
[1] Complete auth-002 (in progress)
[2] Start api-001 (now unblocked)  
[3] Review auth-001 for completion
[4] Show full available task list

> "Show me everything that's ready to work on"

✅ Available tasks (dependencies satisfied):
├── api-001: User registration endpoint (high priority)
├── frontend-003: Login page UI (medium priority)  
├── testing-001: Setup test environment (low priority)
└── docs-002: API documentation update (low priority)

Next action suggestions:
💡 api-001 builds on your auth work
💡 frontend-003 complements the backend auth
```

### Smart Task Creation
```bash
> "We need to add email verification to the authentication system"

TaskGuard Analysis:
🔍 Found related tasks: auth-001, auth-002
🔍 Checking current auth implementation...
├── JWT auth: ✅ implemented  
├── Login endpoint: ✅ exists
├── Email service: ❓ not found
└── User model: ✅ has email field

Suggested task structure:
📝 auth-003-email-verification.md
├── Dependencies: auth-001 (JWT), email-service-001 (missing)
├── Subtasks: verification token, email templates, endpoints
└── Integration points: user registration, login flow

Create this task? [Y/n]
```

---

## POC Implementation Roadmap

### Phase 1: Core Foundation
**Duration: 1-2 weeks**

**Features:**
- Basic CLI: `init`, `list`, `show`, `create`, `edit`
- Markdown parsing with YAML front-matter
- Simple Git integration (add/commit task files)
- Dependency validation
- Status management

**Deliverable:** Single Rust binary that manages local task files.

### Phase 2: Intelligence Layer  
**Duration: 2-3 weeks**

**Features:**
- Git history analysis for status suggestions
- Dependency graph visualization
- Task complexity analysis
- Conflict detection and resolution helpers
- Smart lint warnings

**Deliverable:** TaskGuard that provides intelligent insights about project state.

### Phase 3: Claude Code Bridge
**Duration: 1-2 weeks**

**Features:**
- Natural language command parsing  
- Context-aware task suggestions
- Workflow automation ("what's next?")
- Smart task creation from descriptions
- Integration with Claude Code's command system

**Deliverable:** Seamless natural language task management in Claude Code.

### Phase 4: Advanced Features
**Duration: Ongoing**

**Features:**
- Templates and task generation
- Team collaboration features
- Reporting and analytics
- Custom workflows
- IDE extensions

---

## Configuration (.taskguard/config.toml)

```toml
[project]
name = "My Web Application"
version = "0.2.0"
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

[lint]
max_task_lines = 200
max_checkboxes = 20
warn_on_large_tasks = true
suggest_breakdowns = true

[templates]
default = "basic"

[templates.basic]
areas = ["backend", "frontend", "testing"]
default_complexity = 3
include_acceptance_criteria = true

[templates.epic]
areas = ["architecture", "planning"]  
default_complexity = 8
include_breakdown_suggestions = true
```

---

## Rust Implementation Benefits

### 1. Git Integration Excellence
```toml
[dependencies]
git2 = "0.18"              # Native Git operations
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"         # YAML front-matter parsing
clap = { version = "4.0", features = ["derive"] }
walkdir = "2.4"            # Efficient file traversal
chrono = { version = "0.4", features = ["serde"] }
```

### 2. Performance Guarantees
- Parse hundreds of task files in milliseconds
- Git operations complete instantly
- Single binary deployment (~5MB)
- Memory usage <10MB for typical projects

### 3. Error Handling Philosophy
```rust
// TaskGuard never crashes on user data issues:
match parse_task_file(&path) {
    Ok(task) => tasks.push(task),
    Err(ParseError::InvalidYaml(e)) => {
        warnings.push(format!("⚠️  Skipping {}: Invalid YAML - {}", path, e));
        // Continue processing other files
    },
    Err(ParseError::MissingFrontmatter) => {
        warnings.push(format!("⚠️  Skipping {}: No front-matter found", path));
    }
}
```

---

## Success Metrics

### Developer Experience
- ✅ Zero setup friction (single binary)
- ✅ Works offline completely
- ✅ No external service dependencies
- ✅ Natural Git integration
- ✅ Claude Code feels seamless

### Reliability  
- ✅ Never loses task data
- ✅ Graceful error handling
- ✅ Clear conflict resolution
- ✅ Predictable behavior

### Intelligence
- ✅ Accurate status suggestions from Git
- ✅ Helpful dependency analysis
- ✅ Smart task breakdown recommendations
- ✅ Context-aware "what's next" suggestions

---

## Competitive Advantages

| Feature | TaskGuard | Claude Task Master | Traditional Tools |
|---------|-----------|-------------------|------------------|
| **Local-first** | ✅ Complete | ❌ Requires MCP/API | ⚠️ Varies |
| **Git-native** | ✅ Built-in | ❌ Manual files | ❌ Separate sync |
| **Zero setup** | ✅ Single binary | ❌ Complex config | ❌ Multiple tools |
| **Conflict handling** | ✅ Developer choice | ❌ Automatic/hidden | ⚠️ Basic |
| **Claude Code** | ✅ Natural language | ✅ MCP integration | ❌ None |
| **Reliability** | ✅ Rust + Git | ⚠️ JSON corruption | ⚠️ Varies |
| **Team collaboration** | ✅ Git workflows | ❌ Problematic | ✅ Varies |

---

## Philosophy in Action

### TaskGuard Says "YES" to:
- ✅ Providing clear information about current state
- ✅ Offering data-driven suggestions  
- ✅ Automating tedious technical tasks
- ✅ Helping developers make informed decisions
- ✅ Surfacing problems before they compound

### TaskGuard Says "NO" to:
- ❌ Making content decisions automatically
- ❌ Modifying files without explicit permission
- ❌ "Fixing" conflicts behind the scenes
- ❌ Hiding problems from developers
- ❌ Lock-in to specific tools or services

### Example: The Philosophy in Practice
```bash
$ taskguard auto-fix-dependencies
❌ Error: TaskGuard doesn't auto-fix dependencies.

Instead, try:
├── taskguard validate (see what's broken)
├── taskguard deps --interactive (fix with guidance)  
└── taskguard deps --suggest (see recommended fixes)

This keeps you in control of your project structure.
```

**The Result:** TaskGuard is a thoughtful assistant, not an autonomous agent. It amplifies developer decision-making rather than replacing it.

---

This POC framework provides the foundation for a task management system that learns from Claude Task Master's successes while avoiding its pitfalls, keeping developers firmly in control while providing powerful AI-assisted workflows.