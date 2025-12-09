# TaskGuard Usage Guide for Agentic AI Agents

## 🚀 Quick Command Reference

```bash
# Essential Commands (use every session)
taskguard init                                    # Initialize project
taskguard create --title "Task" --area backend    # Create task (basic)
taskguard list                                    # View all tasks
taskguard validate                                # Check dependencies
taskguard update status <task-id> doing           # Update status

# Create with all options (recommended for AI agents)
taskguard create --title "Task" --area backend --priority high \
  --complexity 7 --tags "api,v2" --dependencies "setup-001" \
  --assignee "developer" --estimate "4h"

# Frequent Commands
taskguard update dependencies <task-id> "dep1,dep2"  # Set dependencies
taskguard update priority <task-id> high             # Change priority
taskguard list items <task-id>                       # View checklist items
taskguard task update <task-id> 1 done              # Mark item complete

# Archive & Restore Commands
taskguard archive [--dry-run]                        # Archive completed tasks (closes GitHub issues if synced)
taskguard restore <task-id>                          # Restore archived task (reopens GitHub issue if synced)

# GitHub Integration (requires .taskguard/github.toml)
taskguard sync --github                              # Sync tasks ↔ GitHub Issues & Projects v2
taskguard sync --github --backfill-project           # Add existing issues to Projects v2 board
taskguard sync --github --dry-run                    # Preview sync without changes

# Bulk Import (convert markdown analysis files to tasks)
taskguard import-md file.md --area github --prefix gh [--dry-run]
```

## ⚡ CLI-First Approach

**CRITICAL**: TaskGuard is designed for **deterministic, programmatic operations**. Use CLI update commands for atomic task modifications instead of manual file editing.

## 🎯 Core 6-Step Workflow for AI Agents

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
taskguard create --title "Verify existing system status" --area setup --priority high --complexity 3
taskguard create --title "Analyze project requirements" --area docs --priority high --complexity 4

# Implementation layer (with dependencies set at creation)
taskguard create --title "Extract core patterns" --area backend --priority medium \
  --complexity 6 --dependencies "setup-001" --estimate "4h"
taskguard create --title "Implement API endpoints" --area api --priority medium \
  --complexity 7 --dependencies "setup-001,backend-001" --tags "rest,v2"
taskguard create --title "Create UI components" --area frontend --priority medium \
  --complexity 5 --dependencies "setup-001"

# Validation layer (depends on implementation)
taskguard create --title "Create integration tests" --area testing --priority medium \
  --complexity 6 --dependencies "api-001,frontend-001" --estimate "3h"
```

**Create command options:**
| Flag | Short | Description | Example |
|------|-------|-------------|---------|
| `--title` | `-t` | Task title (required) | `"Implement feature"` |
| `--area` | `-a` | Task area | `backend` |
| `--priority` | `-p` | Priority level | `low`, `medium`, `high`, `critical` |
| `--complexity` | - | Complexity 1-10 | `7` |
| `--tags` | - | Comma-separated tags | `"api,security,v2"` |
| `--dependencies` | `-d` | Comma-separated task IDs | `"setup-001,auth-001"` |
| `--assignee` | - | Task assignee | `"john"` |
| `--estimate` | `-e` | Time estimate | `"4h"`, `"2d"` |

### Step 3: Validate After Each Creation
```bash
taskguard list
taskguard validate
```

### Step 4: Update with CLI Commands
```bash
# Note: dependencies, priority, assignee can be set at creation with flags
# Use update commands for modifications after task exists

# Update dependencies (if not set at creation)
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

### Step 6: Commit Your Work
After completing implementation work, **proactively commit changes** with descriptive messages:

```bash
# Stage and commit with conventional commit format
git add -A
git commit -m "feat(area): Brief description of changes

- Detail 1
- Detail 2

Closes task-id"
```

**Commit conventions:**
- `feat`: New feature or functionality
- `fix`: Bug fix
- `chore`: Maintenance, dependencies, config
- `docs`: Documentation changes
- `refactor`: Code restructuring without behavior change
- `test`: Adding or updating tests

**Best practice**: Commit after each task completion, not in batches. This preserves clear history and enables easy rollback.

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
**Solution**: Use `--dependencies` flag at creation or `taskguard update dependencies` after

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

### GitHub Integration Issues

**Sync Not Working**
```bash
# Check GitHub configuration
cat .taskguard/github.toml

# Verify credentials (GitHub CLI must be authenticated)
gh auth status

# Test sync with dry-run
taskguard sync --github --dry-run
```

**Issues Not Closing on Archive**
```bash
# Verify task is synced before archiving
taskguard validate  # Shows "Synced to GitHub: #123" for synced tasks

# Check task-issue mapping
cat .taskguard/state/task_issue_mapping.json

# Ensure task was synced at least once before archiving
taskguard sync --github  # Sync before archiving
taskguard archive
```

**Restore Not Reopening Issues**
```bash
# Verify task was previously synced
cat .taskguard/state/task_issue_mapping.json | grep task-id

# Check if issue was actually closed
gh issue view <issue-number>

# Restore should automatically reopen
taskguard restore backend-001
```

**Archived Tasks Showing in Validation**
```bash
# This is expected behavior - validation shows archived synced tasks
# Use this information to understand what was archived and synced

taskguard validate
# Example output:
# 📦 ARCHIVED TASKS (GitHub synced):
#    ✅ backend-001 - Feature X (synced to GitHub: #42, archived)
```

## 🎬 Complete Example Workflow

```bash
# 1. Initialize
taskguard init

# 2. Create foundation task
taskguard create --title "Verify API endpoints" --area setup --priority high --complexity 3
taskguard update status setup-001 doing

# 3. Create dependent task with all options at creation
taskguard create --title "Extract data patterns" --area data --priority medium \
  --complexity 6 --dependencies "setup-001" --estimate "4h" --tags "extraction,analysis"

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

### GitHub Integration

TaskGuard provides comprehensive GitHub integration with bidirectional sync and automatic issue lifecycle management.

#### Setup
Create `.taskguard/github.toml`:
```toml
owner = "your-username"
repo = "your-repo"
project_number = 1
```

#### Core GitHub Workflows

**1. Create and Sync Tasks**
```bash
# Create tasks locally (with all metadata)
taskguard create --title "Feature X" --area backend --priority high \
  --complexity 7 --tags "feature,v2" --estimate "8h"

# Sync to GitHub (creates issues and adds to Projects v2 board)
taskguard sync --github

# Preview sync without making changes
taskguard sync --github --dry-run
```

**2. Work and Update Tasks**
```bash
# Update task status locally
taskguard update status backend-001 doing

# Sync status to GitHub (moves issue to "In Progress" column)
taskguard sync --github
```

**3. Archive Completed Work (GitHub-aware)**
```bash
# IMPORTANT: Always validate before archiving to see sync status
taskguard validate

# Preview what will be archived (shows GitHub sync status)
taskguard archive --dry-run

# Archive completed tasks (automatically closes GitHub issues)
taskguard archive

# Features:
# ✅ Archives completed tasks to .taskguard/archive/
# ✅ Closes corresponding GitHub issues automatically
# ✅ Updates task-issue mapping with archived status
# ✅ Preserves task content for future reference
```

**4. Restore Archived Tasks (GitHub-aware)**
```bash
# Restore a previously archived task
taskguard restore backend-001

# Features:
# ✅ Moves task back to active tasks/ directory
# ✅ Reopens corresponding GitHub issue automatically
# ✅ Updates task-issue mapping to remove archived flag
# ✅ Preserves all task metadata and content
```

#### GitHub Sync Features
- **Auto-creates GitHub Issues** from local tasks
- **Adds issues to Projects v2 board** with correct status columns
- **Bidirectional sync** (local ↔ GitHub)
- **Status mapping**: todo→Backlog, doing→In Progress, done→Done
- **Archive lifecycle**: Archiving closes issues, restoring reopens them
- **Mapping persistence**: Tracks sync state and archived status

#### Recommended GitHub Workflow
```bash
# 1. Create and sync tasks (with full metadata)
taskguard create --title "Feature X" --area backend --priority high --complexity 6
taskguard sync --github

# 2. Work on tasks, sync updates
taskguard update status backend-001 doing
taskguard sync --github

# 3. Complete tasks
taskguard update status backend-001 done
taskguard sync --github

# 4. Archive completed work (closes GitHub issues)
taskguard validate                    # Verify sync status
taskguard archive --dry-run          # Preview
taskguard archive                    # Archives + closes issues

# 5. If needed later, restore (reopens GitHub issues)
taskguard restore backend-001
```

### Bulk Import from Markdown
```bash
# Preview tasks before creating (recommended first step)
taskguard import-md ANALYSIS.md --area github --prefix gh --dry-run

# Create tasks from markdown sections (## Tasks, ## Action Items, etc.)
taskguard import-md ANALYSIS.md --area github --prefix gh

# After import, sync to GitHub
taskguard sync --github

# Supports: [HIGH]/[CRITICAL]/[MEDIUM]/[LOW] priority markers
# Extracts: Numbered lists, checklists, action items
```

For complex workflows, see detailed documentation:
- **Remote team collaboration**: `taskguard sync --remote`
- **Template customization**: `.taskguard/templates/`
- **Complex debugging**: Comprehensive error analysis
- **Batch operations**: Multi-task management strategies

---

**Remember**: TaskGuard is the manager - it tells you which tasks are ready, validates dependencies, and organizes work by priority. Your job: create well-structured tasks and let TaskGuard manage execution flow.