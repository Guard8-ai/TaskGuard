# TaskGuard Usage Guide for Agentic AI Agents

## Critical: Respect the Tool's Design Patterns

TaskGuard is a sophisticated task management system that requires **tool hygiene** and patience. Agentic AI agents must follow its intended workflow rather than trying to bulldoze through with rapid commands.

## Core Problems to Avoid

### ❌ Poor Area Distribution
**Problem**: Cramming everything into `backend` or `api` areas
**Solution**: Use the full spectrum of available areas

### ❌ No Validation Between Operations
**Problem**: Creating tasks without checking current state
**Solution**: Use `taskguard validate` and `taskguard list` frequently

### ❌ Ignoring Dependencies
**Problem**: Creating tasks without proper dependency chains
**Solution**: Edit task files immediately after creation to add dependencies

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

### Step 4: Edit Task Files After Creation
**CRITICAL**: TaskGuard creates tasks with template content - AI agents MUST edit them immediately!

```bash
# TaskGuard creates skeleton tasks with placeholder content
# Example after: taskguard create --title "Setup API endpoints" --area api --priority high

# The file contains generic template content:
# ## Context
# Brief description of what needs to be done and why.
# ## Tasks
# - [ ] Break down the work into specific tasks

# AI agents must immediately edit to add real content:
vim tasks/api/api-001.md
```

**Required Edits:**
1. **Replace template content** with actual requirements
2. **Add dependencies** in YAML front-matter: `dependencies: [setup-001, backend-001]`
3. **Update context** with real project details
4. **Specify concrete tasks** instead of placeholder text
5. **Define measurable acceptance criteria**

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

## Understanding TaskGuard's Create Process

**Important**: TaskGuard's `create` command generates **template tasks**, not AI-generated content.

### What TaskGuard Creates Automatically:
- **YAML metadata**: ID, status (todo), priority, tags, timestamps
- **Template structure**: Standard sections (Context, Objectives, Tasks, etc.)
- **Placeholder content**: Generic text like "Brief description of what needs to be done"

### What AI Agents Must Do:
1. **Immediate editing**: Replace all template content with real requirements
2. **Add dependencies**: Specify which tasks must complete first
3. **Define concrete deliverables**: Replace generic bullet points with specific work items
4. **Validate workflow**: Use `taskguard validate` after each edit

### Template vs. Real Content Example:
```markdown
# TEMPLATE (what TaskGuard creates):
## Context
Brief description of what needs to be done and why.

## Tasks
- [ ] Break down the work into specific tasks

# REAL CONTENT (what AI agents must add):
## Context
The current API lacks user authentication. Need to implement JWT-based auth
system to secure /api/users and /api/data endpoints before frontend integration.

## Tasks
- [ ] Install and configure jsonwebtoken package
- [ ] Create auth middleware for protected routes
- [ ] Implement POST /auth/login endpoint with bcrypt
- [ ] Add token validation to existing user endpoints
- [ ] Write integration tests for auth flow
```

## AI Agent Best Practices

### 🔄 Check State Frequently
```bash
# Use these commands between operations
taskguard list --area backend    # Check specific area
taskguard validate              # See dependency status
taskguard list                  # See full overview
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

# 3. IMMEDIATELY edit the task with real content
# Replace template content in tasks/setup/setup-001.md:
# - Add actual verification steps
# - Define specific endpoints to check
# - Set measurable success criteria

# 4. Validate after editing
taskguard validate

# 5. Create next task
taskguard create --title "Extract data processing patterns" --area data --priority medium

# 6. IMMEDIATELY edit and add dependency
# Edit tasks/data/data-001.md:
# - Add: dependencies: [setup-001]
# - Replace template with real extraction requirements

# 7. Validate dependency chain
taskguard validate
# Should show setup-001 available, data-001 blocked

# 8. Continue create-edit-validate pattern
taskguard create --title "Implement strategy execution endpoint" --area api --priority medium
# Edit tasks/api/api-001.md with dependencies: [setup-001, data-001]
taskguard validate

# 9. Result: Clear dependency chain with real, actionable tasks
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
# Check YAML syntax in task files
cat tasks/api/api-001.md

# Ensure dependency IDs exist and are correct
taskguard validate
```

## Key Success Metrics

A successful TaskGuard session should show:

1. **Clean task distribution**: Tasks spread across multiple areas
2. **Clear dependency chains**: `taskguard validate` shows logical blocking
3. **No parse errors**: All tasks validate successfully
4. **Actionable queue**: Clear list of available tasks to work on
5. **No template content**: All tasks have real requirements, not generic placeholders
6. **Concrete deliverables**: Each task has specific, measurable work items

## Remember: TaskGuard is the Manager

Let TaskGuard guide the workflow:
- It tells you which tasks are ready to work on
- It validates your dependency logic
- It organizes work by area and priority
- It prevents you from working on blocked tasks

**The AI agent's job**: Create well-structured tasks and let TaskGuard manage the execution flow.