# TaskGuard Usage Guide for Agentic AI Agents

## 🚀 Quick Command Reference

```bash
# Essential Commands (use every session)
taskguard init                                    # Initialize project
taskguard create --title "Task" --area backend    # Create task
taskguard list                                    # View all tasks
taskguard validate                                # Check dependencies
taskguard update status <task-id> doing           # Update status

# Frequent Commands
taskguard update dependencies <task-id> "dep1,dep2"  # Set dependencies
taskguard update priority <task-id> high             # Change priority
taskguard list items <task-id>                       # View checklist items
taskguard task update <task-id> 1 done              # Mark item complete
```

## ⚡ CLI-First Approach

**CRITICAL**: TaskGuard is designed for **deterministic, programmatic operations**. Use CLI update commands for atomic task modifications instead of manual file editing.

## 🎯 Core 5-Step Workflow for AI Agents

### Step 1: Initialize and Assess (30 seconds)
```bash
taskguard init
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
taskguard create --title "Extract core patterns" --area backend --priority medium
taskguard create --title "Implement API endpoints" --area api --priority medium
taskguard create --title "Create UI components" --area frontend --priority medium

# Validation layer (will depend on implementation)
taskguard create --title "Create integration tests" --area testing --priority medium
```

### Step 3: Validate After Each Creation
```bash
taskguard list
taskguard validate
```

### Step 4: Update with CLI Commands
```bash
# Update dependencies immediately after creation
taskguard update dependencies api-001 "setup-001,backend-001"

# Adjust priority and ownership
taskguard update priority api-001 critical
taskguard update assignee api-001 "team-lead"

# Track progress
taskguard update status api-001 doing
```

### Step 5: Verify Dependency Chain
```bash
taskguard validate
# Should show clear dependency blocking and available tasks
```

## 📋 Available Areas for Task Distribution

Use these strategically to avoid ID conflicts:

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

## 🔧 CLI Update Commands

### Status Management
```bash
taskguard update status <task-id> <new-status>
# Valid: todo, doing, review, done, blocked
```

### Priority & Assignment
```bash
taskguard update priority <task-id> <priority>     # low, medium, high, critical
taskguard update assignee <task-id> <name>         # Assign ownership
```

### Dependencies
```bash
taskguard update dependencies <task-id> "dep1,dep2,dep3"  # Set dependencies
taskguard update dependencies <task-id> ""                # Clear dependencies
```

### Granular Task Items (NEW)
```bash
taskguard list items <task-id>                    # View numbered checklist
taskguard task update <task-id> <item-index> done  # Mark specific item complete
taskguard task update <task-id> <item-index> todo  # Mark item incomplete
```

## ⚠️ Critical Problems to Avoid

### ❌ Poor Area Distribution
**Problem**: Cramming everything into `backend` or `api` areas
**Solution**: Use the full spectrum of available areas

### ❌ No Validation Between Operations
**Problem**: Creating tasks without checking current state
**Solution**: Use `taskguard validate` and `taskguard list` frequently

### ❌ Ignoring Dependencies
**Problem**: Creating tasks without proper dependency chains
**Solution**: Use `taskguard update dependencies` immediately after creation

### ❌ Manual File Editing
**Problem**: Editing YAML metadata manually instead of using CLI
**Solution**: Use CLI commands for all metadata updates

## 🔄 State Management Best Practices

### Check State Frequently
```bash
taskguard list --area backend    # Check specific area
taskguard validate              # See dependency status
taskguard list                  # Full overview
```

### Think in Dependency Chains
```
setup-001 → backend-001 → api-001 → testing-001
         → frontend-001 → integration-001
```

### Priority Guidelines
- **high**: Critical path items, blockers, foundation work
- **medium**: Core implementation, dependent features
- **low**: Nice-to-have, documentation, optimization

## ✅ Success Metrics

A successful TaskGuard session shows:

1. **Clean task distribution**: Tasks spread across multiple areas
2. **Clear dependency chains**: `taskguard validate` shows logical blocking
3. **No parse errors**: All tasks validate successfully
4. **Actionable queue**: Clear list of available tasks
5. **Deterministic operations**: All metadata updates via CLI commands
6. **No template content**: All tasks have real requirements
7. **Granular progress tracking**: Individual items managed via CLI

## 🚨 Quick Troubleshooting

### Tasks Not Showing
```bash
taskguard validate  # Check for parse errors
ls -la tasks/*/     # Verify file structure
```

### Dependencies Not Working
```bash
taskguard update dependencies api-001 "setup-001,backend-001"  # Use CLI instead of manual editing
taskguard validate  # Verify dependency chain
```

### CLI Commands Failing
```bash
taskguard list | grep task-id  # Check if task exists
echo $?                        # Check exit code (0=success, 1=error)
```

## 🎬 Complete Example Workflow

```bash
# 1. Initialize
taskguard init

# 2. Create foundation
taskguard create --title "Verify API endpoints" --area setup --priority high
taskguard update status setup-001 doing

# 3. Create dependent tasks
taskguard create --title "Extract data patterns" --area data --priority medium
taskguard update dependencies data-001 "setup-001"

# 4. Validate chain
taskguard validate
# Shows: setup-001 doing, data-001 blocked

# 5. Complete setup
taskguard update status setup-001 done

# 6. Validate again
taskguard validate
# Shows: data-001 now available

# 7. Track granular progress
taskguard list items data-001
taskguard task update data-001 1 done
taskguard task update data-001 2 done
```

## 🔗 Advanced Features

For complex workflows, see detailed documentation:
- **Remote team collaboration**: `taskguard sync --remote`
- **Template customization**: `.taskguard/templates/`
- **Complex debugging**: Comprehensive error analysis
- **Batch operations**: Multi-task management strategies

---

**Remember**: TaskGuard is the manager - it tells you which tasks are ready, validates dependencies, and organizes work by priority. Your job: create well-structured tasks and let TaskGuard manage execution flow.