# GitHub Integration

Bidirectional sync between TaskGuard and GitHub Issues/Projects v2.

---

## Setup

Create `.taskguard/github.toml`:

```toml
owner = "your-username"
repo = "your-repo"
project_number = 1  # GitHub Projects v2 number
```

Ensure GitHub CLI is authenticated:
```bash
gh auth status
```

---

## Sync Commands

### `taskguard sync --github`

Sync local tasks to GitHub Issues.

```bash
taskguard sync --github              # Sync all tasks
taskguard sync --github --dry-run    # Preview changes
taskguard sync --github --backfill-project  # Add existing issues to Projects v2
```

**What happens:**

- New tasks create GitHub Issues
- Task updates sync to existing issues
- Status changes update Projects v2 board columns
- Context section used for issue description

### `taskguard archive`

Archive completed tasks and close GitHub issues.

```bash
taskguard archive --dry-run  # Preview
taskguard archive            # Archive + close issues
```

### `taskguard restore`

Restore archived tasks and reopen GitHub issues.

```bash
taskguard restore backend-001
```

---

## Status Mapping

| TaskGuard | GitHub Projects v2 |
|-----------|-------------------|
| `todo` | Backlog |
| `doing` | In Progress |
| `review` | In Review |
| `done` | Done |
| `blocked` | Blocked |

---

## Cross-Branch Sync

TaskGuard v0.3.0 detects duplicate tasks across branches:

- Adds branch name and content hash to issue body
- Warns when same task ID exists on different branches
- Prevents duplicate GitHub issues

---

## Validation

`taskguard validate` shows GitHub sync status:

```
Available tasks:
   setup-001 - Setup Development (synced: #42)

Blocked tasks:
   backend-001 - API Implementation (synced: #43)

Archived tasks:
   auth-001 - Authentication (synced: #41, archived)
```

---

## Workflow Example

```bash
# 1. Create and sync
taskguard create --title "Feature X" --area backend
taskguard sync --github

# 2. Work and update
taskguard update status backend-001 doing
taskguard sync --github

# 3. Complete and archive
taskguard update status backend-001 done
taskguard sync --github
taskguard archive
```
