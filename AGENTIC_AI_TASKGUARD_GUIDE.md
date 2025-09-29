# TaskGuard Usage Guide for Agentic AI Agents

## Critical: Use CLI-First Approach

TaskGuard is a sophisticated task management system designed for **deterministic, programmatic operations**. Agentic AI agents should use the CLI update commands for atomic task modifications instead of manual file editing.

## Core Problems to Avoid

### ❌ Poor Area Distribution
**Problem**: Cramming everything into `backend` or `api` areas
**Solution**: Use the full spectrum of available areas

### ❌ No Validation Between Operations
**Problem**: Creating tasks without checking current state
**Solution**: Use `taskguard validate` and `taskguard list` frequently

### ❌ Ignoring Dependencies
**Problem**: Creating tasks without proper dependency chains
**Solution**: Use `taskguard update dependencies <task-id> <deps>` immediately after creation

## Correct TaskGuard Workflow for AI Agents

### Step 1: Initialize and Assess
```bash
# Always start here
taskguard init

# Check current state
taskguard list
taskguard validate
```

### Step 2: Strategic Task Distribution
Create **ONE task per area initially** to avoid ID conflicts:

```bash
# Foundation layer (no dependencies)
taskguard create --title "Verify existing system status" --area setup --priority high
taskguard create --title "Analyze project requirements" --area docs --priority high

# Implementation layer (will depend on foundation)
taskguard create --title "Extract core patterns from legacy code" --area backend --priority medium
taskguard create --title "Implement primary API endpoints" --area api --priority medium
taskguard create --title "Create UI components" --area frontend --priority medium

# Validation layer (will depend on implementation)
taskguard create --title "Create integration test suite" --area testing --priority medium
taskguard create --title "Validate end-to-end workflows" --area integration --priority low
```

### Step 3: Validate After Each Creation
```bash
# After each task creation, check the state
taskguard list
taskguard validate
```

### Step 4: Update Task Metadata with CLI Commands
**CRITICAL**: TaskGuard creates tasks with template content - use CLI commands for atomic updates!

```bash
# After creating a task, use CLI commands for deterministic updates
# Example after: taskguard create --title "Setup API endpoints" --area api --priority high

# Update task dependencies immediately
taskguard update dependencies api-001 "setup-001,backend-001"

# Adjust priority if needed
taskguard update priority api-001 critical

# Assign ownership
taskguard update assignee api-001 "team-lead"

# Update status as work progresses
taskguard update status api-001 doing
```

**CLI Update Commands Available:**
1. **Dependencies**: `taskguard update dependencies <task-id> <dep1,dep2,dep3>`
2. **Status**: `taskguard update status <task-id> <todo|doing|review|done|blocked>`
3. **Priority**: `taskguard update priority <task-id> <low|medium|high|critical>`
4. **Assignee**: `taskguard update assignee <task-id> <name>`
5. **Task Items**: `taskguard list items <task-id>` - List checklist items
6. **Task Items**: `taskguard task update <task-id> <item-index> <done|todo>` - Update specific items

**Still Required (Manual Editing):**
- Replace template content with actual requirements
- Update context with real project details
- Specify concrete tasks instead of placeholder text
- Define measurable acceptance criteria

### Step 5: Verify Dependency Chain
```bash
taskguard validate
# Should show clear dependency blocking and available tasks
```

## Available Areas for Task Distribution

Use these areas strategically to avoid ID conflicts:

- **setup**: Environment verification, prerequisites, project initialization
- **docs**: Documentation, requirements analysis, planning
- **backend**: Core server-side implementation
- **api**: Endpoint development, REST/GraphQL APIs
- **frontend**: UI/UX components, client-side logic
- **auth**: Authentication, authorization, security
- **data**: Data processing, extraction, database work
- **testing**: Unit tests, integration tests, validation
- **integration**: System integration, connecting components
- **deployment**: CI/CD, infrastructure, production setup

## CLI Update Commands Reference

TaskGuard provides deterministic CLI commands for atomic task updates:

### Status Updates
```bash
taskguard update status <task-id> <new-status>
# Valid statuses: todo, doing, review, done, blocked

# Examples:
taskguard update status api-001 doing
taskguard update status backend-002 done
taskguard update status frontend-001 blocked
```

### Priority Updates
```bash
taskguard update priority <task-id> <new-priority>
# Valid priorities: low, medium, high, critical

# Examples:
taskguard update priority setup-001 critical
taskguard update priority docs-001 low
```

### Assignee Updates
```bash
taskguard update assignee <task-id> <assignee-name>
# Use "" or "none" to clear assignee

# Examples:
taskguard update assignee api-001 "backend-team"
taskguard update assignee frontend-001 "alice"
taskguard update assignee testing-001 ""  # Clear assignee
```

### Dependency Updates
```bash
taskguard update dependencies <task-id> <comma-separated-deps>
# Use "" or "none" to clear dependencies

# Examples:
taskguard update dependencies api-001 "setup-001,backend-001"
taskguard update dependencies integration-001 "api-001,frontend-001,auth-001"
taskguard update dependencies testing-001 ""  # Clear dependencies
```

### Granular Task Item Management
**NEW**: TaskGuard now supports programmatic management of individual checklist items within tasks, enabling precise progress tracking for agentic AI systems.

#### List Task Items
```bash
taskguard list items <task-id>
# Shows numbered checklist items with status and progress summary

# Example output:
# 📋 Checklist items for task backend-001:
#    Implement user authentication system
#
#    1. ✅ [done] Install and configure JWT dependencies
#    2. ⭕ [todo] Create authentication middleware
#    3. ⭕ [todo] Implement login endpoint
#    4. ✅ [done] Add password hashing with bcrypt
#
# 📊 SUMMARY
#    Total items: 4
#    Completed: 2 (50.0%)
#    Remaining: 2
```

#### Update Individual Task Items
```bash
taskguard task update <task-id> <item-index> <status>
# item-index: 1-based index from list output
# status: done, todo (or completed, incomplete)

# Examples:
taskguard task update backend-001 2 done    # Mark 2nd item as completed
taskguard task update api-001 3 todo        # Mark 3rd item as incomplete
taskguard task update auth-001 1 done       # Mark 1st item as completed
```

#### Benefits for Agentic AI
- **Atomic Operations**: Each item update is a single, deterministic operation
- **Progress Tracking**: Get precise completion percentages and remaining work
- **Error Handling**: Clear validation for invalid indexes and task IDs
- **Preserved Formatting**: Markdown structure remains intact during updates
- **Consistent Indexing**: 1-based indexing matches human-readable output

### CLI Command Benefits for AI Agents
- **Atomic operations**: Updates are all-or-nothing
- **Consistent exit codes**: 0 for success, 1 for errors
- **Machine-parseable errors**: Clear error messages for automation
- **Idempotent**: Safe to run multiple times
- **Immediate validation**: Invalid values are rejected with helpful messages

## Understanding TaskGuard's Create Process

**Important**: TaskGuard's `create` command generates **template tasks**, not AI-generated content.

### What TaskGuard Creates Automatically:
- **YAML metadata**: ID, status (todo), priority, tags, timestamps
- **Template structure**: Standard sections (Context, Objectives, Tasks, Testing, Version Control, etc.)
- **Placeholder content**: Generic text like "Brief description of what needs to be done"
- **Development workflow reminders**: Testing and Version Control sections with best practices

### What AI Agents Must Do:
1. **Immediate editing**: Replace all template content with real requirements
2. **Add dependencies**: Specify which tasks must complete first
3. **Define concrete deliverables**: Replace generic bullet points with specific work items
4. **Customize testing approach**: Replace generic testing reminders with project-specific test plans
5. **Adapt version control workflow**: Customize commit and branching strategy for the task
6. **Validate workflow**: Use `taskguard validate` after each edit

### Template vs. Real Content Example:
```markdown
# TEMPLATE (what TaskGuard creates):
## Context
Brief description of what needs to be done and why.

## Tasks
- [ ] Break down the work into specific tasks

## Testing
- [ ] Write unit tests for new functionality
- [ ] Write integration tests if applicable
- [ ] Ensure all tests pass before marking task complete
- [ ] Consider edge cases and error conditions

## Version Control
- [ ] Commit changes incrementally with clear messages
- [ ] Use descriptive commit messages that explain the "why"
- [ ] Consider creating a feature branch for complex changes
- [ ] Review changes before committing

# REAL CONTENT (what AI agents must add):
## Context
The current API lacks user authentication. Need to implement JWT-based auth
system to secure /api/users and /api/data endpoints before frontend integration.

## Tasks
- [ ] Install and configure jsonwebtoken package
- [ ] Create auth middleware for protected routes
- [ ] Implement POST /auth/login endpoint with bcrypt
- [ ] Add token validation to existing user endpoints

## Testing
- [ ] Write unit tests for auth middleware with valid/invalid tokens
- [ ] Create integration tests for login flow with real JWT tokens
- [ ] Test protected route access with and without authentication
- [ ] Verify error handling for malformed tokens and expired sessions

## Version Control
- [ ] Commit auth middleware implementation separately
- [ ] Commit login endpoint with clear description of JWT flow
- [ ] Create feature branch for auth implementation
- [ ] Review security implications before merging to main
```

## AI Agent Best Practices

### 🔄 Check State Frequently
```bash
# Use these commands between operations
taskguard list --area backend    # Check specific area
taskguard validate              # See dependency status
taskguard list                  # See full overview
```

### ⚡ Use Deterministic CLI Operations
```bash
# Prefer CLI commands over manual file editing
taskguard update status backend-001 doing          # Start work
taskguard update priority setup-001 critical       # Adjust priority
taskguard update dependencies api-001 "setup-001,backend-001"  # Set dependencies
taskguard update assignee frontend-001 "ui-team"   # Assign ownership

# NEW: Granular task item management
taskguard list items backend-001                   # View checklist items
taskguard task update backend-001 1 done           # Mark specific items complete
taskguard task update api-001 3 todo              # Revert items if needed

# Commands provide consistent exit codes for automation
echo $?  # 0 for success, 1 for error
```

### 📊 Start Small, Expand Gradually
1. Create 5-7 high-level tasks across different areas
2. Let dependencies guide which tasks are ready
3. Break down tasks into smaller subtasks as needed
4. Use TaskGuard's validation to stay organized

### 🔗 Think in Dependency Chains
```
setup-001 → backend-001 → api-001 → testing-001
         → frontend-001 → integration-001
```

### 🎯 Priority Guidelines
- **high**: Critical path items, blockers, foundation work
- **medium**: Core implementation, dependent features
- **low**: Nice-to-have, documentation, optimization

## Example: Complete AI Agent Workflow

```bash
# 1. Initialize
taskguard init

# 2. Create foundation task
taskguard create --title "Verify existing API endpoints" --area setup --priority high

# 3. Start work immediately with CLI commands
taskguard update status setup-001 doing
taskguard validate

# 4. Create next task and set dependencies
taskguard create --title "Extract data processing patterns" --area data --priority medium
taskguard update dependencies data-001 "setup-001"

# 5. Validate dependency chain
taskguard validate
# Should show setup-001 doing, data-001 blocked

# 6. Create API task with multiple dependencies
taskguard create --title "Implement strategy execution endpoint" --area api --priority medium
taskguard update dependencies api-001 "setup-001,data-001"
taskguard update priority api-001 high  # Increase priority

# 7. Complete setup task and see chain reaction
taskguard update status setup-001 done
taskguard validate
# Now data-001 becomes available

# 8. Continue with deterministic operations
taskguard update status data-001 doing
taskguard update assignee data-001 "data-team"

# 9. Track granular progress within tasks
taskguard list items data-001
taskguard task update data-001 1 done  # Mark first checklist item complete
taskguard task update data-001 2 done  # Mark second item complete

# 10. Result: Clean dependency chain with granular progress tracking
```

## Debugging TaskGuard Issues

### When Tasks Aren't Showing Up
```bash
# Check for parsing errors
taskguard validate

# Verify file structure
ls -la tasks/*/
```

### When IDs Are Conflicting
```bash
# Check current tasks by area
taskguard list --area backend
taskguard list --area api

# Ensure you're not creating multiple tasks in same area rapidly
```

### When Dependencies Aren't Working
```bash
# Use CLI commands instead of manual editing
taskguard update dependencies api-001 "setup-001,backend-001"

# Verify dependency chain
taskguard validate

# Check specific task details
taskguard list --area api
```

### When CLI Commands Fail
```bash
# Check if task exists
taskguard list | grep task-id

# Verify valid status values
taskguard update status task-001 invalid-status
# Error: Invalid status 'invalid-status'. Valid statuses: todo, doing, review, done, blocked

# Check exit codes in scripts
taskguard update priority api-001 high
echo $?  # 0 for success, 1 for error

# Test granular item management errors
taskguard list items nonexistent-001
# Error: Task file not found

taskguard task update backend-001 99 done
# Error: Invalid item index 99. Valid range: 1-10

taskguard task update backend-001 1 invalid
# Error: Invalid status 'invalid'. Valid values: done, todo
```

## Key Success Metrics

A successful TaskGuard session should show:

1. **Clean task distribution**: Tasks spread across multiple areas
2. **Clear dependency chains**: `taskguard validate` shows logical blocking
3. **No parse errors**: All tasks validate successfully
4. **Actionable queue**: Clear list of available tasks to work on
5. **Deterministic operations**: All metadata updates done via CLI commands
6. **Consistent exit codes**: CLI commands return 0 for success, 1 for errors
7. **Atomic updates**: Task state changes are atomic and reversible
8. **No template content**: All tasks have real requirements, not generic placeholders
9. **Concrete deliverables**: Each task has specific, measurable work items
10. **Proper status tracking**: Tasks progress through logical status transitions
11. **Granular progress tracking**: Individual task items managed via CLI commands
12. **Precise completion metrics**: Real-time progress percentages for complex tasks

## Remember: TaskGuard is the Manager

Let TaskGuard guide the workflow:
- It tells you which tasks are ready to work on
- It validates your dependency logic
- It organizes work by area and priority
- It prevents you from working on blocked tasks

**The AI agent's job**: Create well-structured tasks and let TaskGuard manage the execution flow.