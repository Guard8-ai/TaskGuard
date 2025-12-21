# First Task Tutorial

Create and manage your first task with TaskGuard in under 5 minutes.

---

## Step 1: Initialize Project

Navigate to your project directory and initialize TaskGuard:

```bash
cd ~/my-project
taskguard init
```

**Output:**
```
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

🤖 AI Agent Integration:
   ✅ AGENTIC_AI_TASKGUARD_GUIDE.md copied to project root
```

**What happened:**
- Created `.taskguard/` configuration directory
- Created `tasks/` directory with area subdirectories
- Generated example task
- Set up AI integration files
- Updated `.gitignore`

---

## Step 2: Create Your First Task

Create a high-priority task for setting up a database. **v0.4.0+: Dependencies are required.**

```bash
# setup-001 was auto-created by init - use it as the root dependency
taskguard create --title "Setup PostgreSQL database" --area backend --priority high --dependencies "setup-001"
```

**Output:**
```
✅ Created task: backend/backend-001.md
   ID: backend-001
   Title: Setup PostgreSQL database
   Area: backend
   Priority: high

📝 Next steps:
   taskguard show backend-001  # View task details
   Edit the file to add more details
```

**What was created:**
- File: `tasks/backend/backend-001.md`
- Unique ID: `backend-001` (area-number format)
- Status: `todo` (default)
- Priority: `high`
- Timestamp: Auto-generated

---

## Step 3: View Your Tasks

List all tasks:

```bash
taskguard list
```

**Output:**
```
📁 BACKEND
   ───────────
   ⭕ 🟠 backend-001 Setup PostgreSQL database

📁 SETUP
   ─────────
   ⭕ 🟠 setup-001 Project Setup and Dependencies

📊 SUMMARY
   Total tasks: 2
   todo: 2
```

**Legend:**
- ⭕ = Task status (open)
- 🔴 = Critical priority
- 🟠 = High priority
- 🟡 = Medium priority
- ⚪ = Low priority

---

## Step 4: Edit Task Details

Open the task file in your editor:

```bash
vim tasks/backend/backend-001.md
# Or use your preferred editor
```

**File structure:**
```yaml
---
id: backend-001
title: Setup PostgreSQL database
status: todo
priority: high
tags:
- backend
dependencies: []
assignee: developer
created: 2025-10-05T10:00:00Z
estimate: ~
complexity: 3
area: backend
---

# Setup PostgreSQL database

## Context
Brief description of what needs to be done and why.

## Objectives
- Clear, actionable objectives
- Measurable outcomes
- Success criteria

## Tasks
- [ ] Install PostgreSQL
- [ ] Create database schema
- [ ] Set up connection pooling
- [ ] Configure authentication

## Acceptance Criteria
✅ **Database Running:**
- PostgreSQL installed and running
- Database schema created
- Connection successful

## Technical Notes
- Use PostgreSQL 15+
- Connection string in .env
- Pool size: 20 connections
```

**Edit the file to add:**
- Specific objectives
- Checklist items
- Acceptance criteria
- Technical notes

---

## Step 5: Create Dependent Tasks

Create a task that depends on the database setup (specify dependency at creation):

```bash
taskguard create --title "Create user authentication API" --area api --priority high --dependencies "backend-001"
```

**The API task is automatically blocked until backend-001 is complete.**

!!! tip "Updating Dependencies"
    You can also update dependencies later:
    ```bash
    taskguard update dependencies api-001 "backend-001,auth-001"
    ```

---

## Step 6: Validate Dependencies

Check which tasks are ready to work on:

```bash
taskguard validate
```

**Output:**
```
🚦 TASK STATUS
   ✅ Available tasks (dependencies satisfied):
      ⭕ backend-001 - Setup PostgreSQL database
      ⭕ setup-001 - Project Setup and Dependencies

   🚫 Blocked tasks:
      ❌ api-001 - Create user authentication API (waiting for: backend-001)

✅ VALIDATION PASSED
   No issues found in 3 tasks

📊 SUMMARY
   Total tasks: 3
   Available: 2
   Blocked: 1
   Parse errors: 0
   Dependency issues: 0
```

**Interpretation:**
- `backend-001` and `setup-001` are ready to work on (no dependencies)
- `api-001` is blocked waiting for `backend-001` to complete

---

## Step 7: Update Task Status

Start working on the backend task:

```bash
taskguard update status backend-001 doing
```

**Output:**
```
✅ Updated task backend-001
   status: todo → doing
```

**Check status:**
```bash
taskguard list --status doing
```

**Output:**
```
📁 BACKEND
   ───────────
   ▶️ 🟠 backend-001 Setup PostgreSQL database

📊 SUMMARY
   Total tasks: 1
   doing: 1
```

---

## Step 8: Complete the Task

When work is finished, mark the task as done:

```bash
taskguard update status backend-001 done
```

**Verify:**
```bash
taskguard validate
```

**Output:**
```
🚦 TASK STATUS
   ✅ Available tasks (dependencies satisfied):
      ⭕ api-001 - Create user authentication API  ← Now unblocked!
      ⭕ setup-001 - Project Setup and Dependencies

   🚫 Blocked tasks:
      (none)

✅ VALIDATION PASSED
```

**Notice:** `api-001` is now available because `backend-001` is complete!

---

## Step 9: Filter Tasks

View tasks by area:

```bash
taskguard list --area backend
```

View tasks by status:

```bash
taskguard list --status done
```

---

## Step 10: Git Integration (Optional)

TaskGuard can analyze Git commits to suggest status updates:

```bash
# Make some commits mentioning task IDs
git commit -m "backend-001: Install PostgreSQL and create schema"

# Sync from git
taskguard sync

# TaskGuard suggests status updates based on commits
```

---

## Common Workflows

### Create → Work → Complete

```bash
# 1. Create task (with dependencies - required in v0.4.0+)
taskguard create --title "Implement feature X" --area backend --dependencies "setup-001"

# 2. Start work
taskguard update status backend-002 doing

# 3. Make changes
git add .
git commit -m "backend-002: Implement core logic"

# 4. Complete
taskguard update status backend-002 done
```

### Create Task Chain

```bash
# setup-001 is auto-created by init as root

# Create dependent tasks with --dependencies
taskguard create --title "Backend API" --area backend --dependencies "setup-001"
taskguard create --title "Frontend UI" --area frontend --dependencies "setup-001"
taskguard create --title "Integration Tests" --area testing --dependencies "backend-001,frontend-001"

# Validate and check for orphans
taskguard validate --orphans
```

---

## Checklist Updates

Update individual checklist items within a task:

```bash
# Mark first checklist item as done
taskguard task update backend-001 1 done

# Mark second item as done
taskguard task update backend-001 2 done
```

---

## What You've Learned

✅ Initialize TaskGuard in a project
✅ Create tasks with metadata
✅ Edit task files (YAML + Markdown)
✅ Set up dependencies
✅ Validate dependency chains
✅ Update task status
✅ Filter and list tasks
✅ Git integration basics

---

## Next Steps

Now that you understand the basics:

- **[Task Structure](../core-concepts/task-structure.md)** - Deep dive into task format
- **[Dependencies](../features/dependencies.md)** - Master dependency management
- **[Git Sync](../features/git-sync.md)** - Automate with Git integration
- **[Common Workflows](../usage-examples/common-workflows.md)** - Real-world patterns

---

## Quick Reference

```bash
# Initialize (creates setup-001 as root)
taskguard init

# Create (dependencies required in v0.4.0+)
taskguard create --title "Task" --area <area> --dependencies "setup-001"

# Create without dependencies (escape hatch)
taskguard create --title "Spike" --area <area> --allow-orphan-task

# List
taskguard list [--area <area>] [--status <status>]

# Update
taskguard update status <task-id> <status>
taskguard update dependencies <task-id> "dep1,dep2"

# Validate
taskguard validate
taskguard validate --orphans

# Git sync
taskguard sync
```

**Areas:** setup, backend, frontend, api, auth, testing
**Statuses:** todo, doing, review, done, blocked
**Priorities:** low, medium, high, critical
