# TaskGuard Usage Guide for Agentic AI Agents

## Critical: Respect the Tool's Design Patterns

TaskGuard is a sophisticated task management system that requires **tool hygiene** and patience. Agentic AI agents must follow its intended workflow rather than trying to bulldoze through with rapid commands.

## Core Problems to Avoid

### ❌ Task ID Overwriting
**Problem**: Creating multiple tasks in the same area rapidly causes ID conflicts
**Solution**: Distribute tasks across different areas initially

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

### Step 4: Add Dependencies by Editing Files
```bash
# Edit task files to add proper dependencies
vim tasks/api/api-001.md
# Add to YAML front-matter:
# dependencies: [setup-001, backend-001]

vim tasks/testing/testing-001.md
# Add to YAML front-matter:
# dependencies: [api-001, backend-001, frontend-001]
```

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

# 2. Create strategic foundation tasks
taskguard create --title "Verify existing API endpoints" --area setup --priority high
taskguard create --title "Analyze requirements document" --area docs --priority high

# 3. Check state
taskguard list
taskguard validate

# 4. Create implementation tasks
taskguard create --title "Extract data processing patterns" --area data --priority medium
taskguard create --title "Implement strategy execution endpoint" --area api --priority medium

# 5. Check state again
taskguard validate

# 6. Edit task files to add dependencies
# Edit tasks/api/api-001.md to depend on [setup-001, data-001]
# Edit tasks/data/data-001.md to depend on [docs-001]

# 7. Create validation tasks
taskguard create --title "Create end-to-end test suite" --area testing --priority medium

# 8. Final validation
taskguard validate
# Should show clear dependency chain and available tasks
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

## Remember: TaskGuard is the Manager

Let TaskGuard guide the workflow:
- It tells you which tasks are ready to work on
- It validates your dependency logic
- It organizes work by area and priority
- It prevents you from working on blocked tasks

**The AI agent's job**: Create well-structured tasks and let TaskGuard manage the execution flow.