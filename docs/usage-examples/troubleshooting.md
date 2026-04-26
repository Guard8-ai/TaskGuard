# Troubleshooting

Common issues and solutions.

---

## Installation

### `taskguard: command not found`

**Cause:** `~/.cargo/bin` is not on your `PATH`.

```bash
# Check
echo $PATH | tr ':' '\n' | grep cargo

# Fix (bash)
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc && source ~/.bashrc

# Fix (zsh)
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc && source ~/.zshrc
```

### Build fails on install

```bash
# Ensure Rust toolchain is up to date
rustup update stable

# Retry
cargo install taskguard --force
```

---

## Initialization

### `taskguard init` creates no `tasks/` directory

Ensure you are in the right project directory and have write permission.

```bash
ls -la   # Check you're in the right place
pwd
```

### `.taskguard/` not found when running commands

TaskGuard walks up the directory tree to find `.taskguard/`. Either you haven't run `taskguard init`, or you're outside the project tree.

```bash
# Check from project root
ls .taskguard/

# If missing, initialize
taskguard init
```

---

## Task Creation

### `CAUTION: Task has no dependencies`

Every task must have dependencies (v0.4.0+). This is intentional — it enforces causality chains.

```bash
# Add at least one dependency
taskguard create --title "My task" --area backend --dependencies "setup-001"

# For a genuine spike/research task with no logical parent
taskguard create --title "Research rate limiting options" --allow-orphan-task
```

### Task ID already exists

IDs are auto-generated and should never collide. If you see this error, an archived task may have been manually moved back to `tasks/`.

```bash
# Check archives for the conflicting ID
ls .taskguard/archive/

# Let TaskGuard generate the next available ID (don't specify one manually)
taskguard create --title "..." --area backend --dependencies "setup-001"
```

---

## YAML / Parse Errors

### `Failed to parse YAML front-matter`

Common causes:

1. **Missing `---` delimiters** — the file must start with `---` and have a second `---` closing the front-matter.
2. **Tab characters** — YAML requires spaces, not tabs.
3. **Unquoted special characters** — colons, brackets, and `#` in values must be quoted.
4. **Missing required field** — `id`, `title`, `status`, `area`, and `created` are required.

```bash
# Identify the bad file
taskguard validate

# Check YAML manually
head -20 tasks/backend/backend-001.md
```

**Correct front-matter:**

```yaml
---
id: backend-001
title: "Task with: special characters"
status: todo
priority: medium
area: backend
created: 2025-10-05T10:00:00Z
dependencies:
  - setup-001
---
```

---

## Dependency Issues

### `Circular dependency detected`

Two or more tasks depend on each other, creating a cycle.

```bash
# See which tasks are involved
taskguard validate

# Fix: remove one of the circular dependencies
taskguard update dependencies backend-002 "setup-001"
taskguard validate
```

### `Missing dependency: xyz-001`

A task references a dependency ID that doesn't exist in `tasks/` or `.taskguard/archive/`.

```bash
# Check what tasks exist
taskguard list

# Either create the missing task
taskguard create --title "..." --area xyz --dependencies "setup-001"

# Or update the dependency to a valid ID
taskguard update dependencies backend-003 "setup-001"
```

### Task is blocked but I've completed all dependencies

The dependency tasks must have `status: done`, not just `doing` or `review`.

```bash
# Mark the dependency as done
taskguard update status backend-001 done

# Re-check
taskguard validate
```

---

## Orphan Tasks

### `CAUTION: X orphan task(s) found`

Orphan tasks have no dependencies and nothing depends on them.

```bash
# See which tasks are orphans
taskguard validate --orphans

# Fix: add a dependency to connect the task to the graph
taskguard update dependencies backend-007 "setup-001"
```

---

## GitHub Sync

### `gh: command not found`

GitHub sync requires the `gh` CLI to be installed and authenticated.

```bash
# Install gh (macOS)
brew install gh

# Authenticate
gh auth login

# Verify
gh auth status
```

### `Could not find project number N`

The `project_number` in `.taskguard/github.toml` doesn't match any GitHub project.

```bash
# List your projects
gh api graphql -f query='{ viewer { projectsV2(first: 10) { nodes { number title } } } }'

# Update github.toml with the correct number
```

### Duplicate issues created after branch switch

Each branch has its own working tree, and sync detects duplicates by matching task ID + content hash in the issue body. If you see duplicates:

1. They likely have different content hashes (task was modified on one branch).
2. Close the stale duplicate manually on GitHub.
3. Remove the stale entry from `.taskguard/github-mapping.json`.

---

## Archive / Restore

### `Cannot archive: task is depended on by active tasks`

A non-done task lists this task as a dependency. The protection is intentional — archiving a dependency of an active task would break the graph.

```bash
# Find the dependent task
taskguard validate

# Option A: Complete the dependent task first
taskguard update status backend-002 done
taskguard archive

# Option B: Update the dependent's dependencies to remove this task
taskguard update dependencies backend-002 "setup-001"
taskguard archive
```

### Archived task not found by `taskguard restore`

The task file must be in `.taskguard/archive/{area}/`. Check manually:

```bash
find .taskguard/archive -name "*.md" | xargs grep -l "id: backend-003"
```

---

## Performance

### `taskguard list` is slow

This is unlikely unless you have thousands of task files. If it happens:

```bash
# Filter to reduce output
taskguard list --area backend
taskguard list --status doing

# Check for very large task files
taskguard stats
```
