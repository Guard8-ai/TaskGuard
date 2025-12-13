# State Management

Learn how TaskGuard stores and manages task state locally and in Git.

---

## Local-First Architecture

TaskGuard follows a **local-first** philosophy:

- ✅ All data stays on your machine
- ✅ No cloud sync required
- ✅ No network dependencies
- ✅ Complete data ownership

---

## File System Structure

TaskGuard uses your project's file system for all state:

```
my-project/
├── .taskguard/                # TaskGuard configuration
│   ├── config.toml           # Project settings
│   ├── templates/            # Task templates
│   └── state/                # Local state (gitignored)
│
├── tasks/                    # Task files (version controlled)
│   ├── setup/
│   │   ├── setup-001.md
│   │   └── setup-002.md
│   ├── backend/
│   │   ├── backend-001.md
│   │   └── backend-002.md
│   └── [other areas]/
│
└── .git/                     # Git repository
```

---

## Configuration State

### `.taskguard/config.toml`

Project-level configuration (committed to Git):

```toml
[project]
name = "My Project"
version = "0.2.2"
areas = ["setup", "backend", "frontend", "api", "auth", "testing"]

[settings]
statuses = ["todo", "doing", "review", "done", "blocked"]
priorities = ["low", "medium", "high", "critical"]
complexity_scale = "1-10"
default_estimate_unit = "hours"

[git]
auto_add_tasks = true
auto_commit_on_status_change = false

[ai]
enabled = true
claude_code_integration = true
```

**Why it's versioned:**
- Team members share same configuration
- Areas and statuses consistent across team
- Settings travel with the project

---

## Task State

### Task Files (`tasks/`)

Each task is a **Markdown file with YAML metadata**:

**Location:** `tasks/{area}/{id}.md`

**State stored:**
- Task metadata (YAML front-matter)
- Task content (Markdown body)
- Checklist item status
- Dependency relationships

**Example:**
```yaml
---
id: backend-001
title: "Implement auth"
status: doing          # ← Current state
priority: high
dependencies: [setup-001]  # ← Relationship state
---

# Markdown content with current notes
```

**Why files:**
- ✅ Human-readable
- ✅ Git-trackable
- ✅ Editable with any text editor
- ✅ Diffable for collaboration
- ✅ No database required

---

## Local State (Not Versioned)

### `.taskguard/state/`

**Gitignored** - local-only state:

```
.taskguard/state/
├── cache/                  # Performance caches
├── last_sync.json         # Last git sync timestamp
└── user_prefs.json        # User-specific settings
```

**Why gitignored:**
- User-specific preferences
- Machine-specific caches
- Not shared across team

---

## Git Integration

TaskGuard is **Git-native** - it uses Git as the persistence and collaboration layer.

### What's in Git

**Versioned:**
```
.taskguard/config.toml     ✅ Shared configuration
.taskguard/templates/      ✅ Shared templates
tasks/**/*.md              ✅ All task files
AGENTIC_AI_TASKGUARD_GUIDE.md  ✅ AI guides
```

**Gitignored:**
```
.taskguard/state/          ❌ Local state
```

### Benefits of Git Storage

1. **Version History**
   ```bash
   git log tasks/backend/backend-001.md
   # See complete task history
   ```

2. **Collaboration**
   ```bash
   git push  # Share tasks with team
   git pull  # Get team's tasks
   ```

3. **Branching**
   ```bash
   git checkout -b feature/new-tasks
   # Work on tasks in isolation
   ```

4. **Conflict Resolution**
   ```bash
   # Git handles conflicts in task files
   # Resolve manually when needed
   ```

---

## State Persistence

### How State is Saved

**Task Creation:**
```bash
taskguard create --title "Task" --area backend
```
1. Generate unique ID
2. Create YAML + Markdown file
3. Write to `tasks/backend/backend-NNN.md`
4. File is now the **source of truth**

**Task Update:**
```bash
taskguard update status backend-001 doing
```
1. Read `tasks/backend/backend-001.md`
2. Parse YAML front-matter
3. Update `status` field
4. Write back to file

**No Database:**
- State lives in text files
- Changes are atomic file writes
- No separate database to corrupt or sync

---

## State Synchronization

### Team Collaboration

**Workflow:**

1. **Alice creates tasks:**
   ```bash
   taskguard create --title "API endpoint" --area backend
   git add tasks/
   git commit -m "Add backend tasks"
   git push
   ```

2. **Bob pulls tasks:**
   ```bash
   git pull
   taskguard list  # Sees Alice's tasks
   ```

3. **Bob updates task:**
   ```bash
   taskguard update status backend-001 doing
   git add tasks/backend/backend-001.md
   git commit -m "Start working on backend-001"
   git push
   ```

4. **Alice syncs:**
   ```bash
   git pull
   taskguard list --status doing  # Sees Bob's update
   ```

### Conflict Resolution

**Scenario:** Alice and Bob both edit `backend-001.md`

**Git merge conflict:**
```yaml
<<<<<<< HEAD
status: review
=======
status: doing
>>>>>>> alice/update
```

**Resolution:**
```bash
# Manual resolution
vim tasks/backend/backend-001.md
# Choose correct status

git add tasks/backend/backend-001.md
git commit -m "Resolve status conflict"
```

**TaskGuard philosophy:** Surfaces conflicts, doesn't auto-resolve. Developer decides.

---

## State Consistency

### Validation

TaskGuard validates state on every operation:

```bash
taskguard validate
```

**Checks:**
1. All task files have valid YAML
2. Dependencies reference existing tasks
3. No circular dependencies
4. IDs match filenames

**Repairs:**
- Parse errors → Skip file, warn user
- Missing dependencies → Flag as error
- Circular deps → Flag as error

**No auto-fix:** TaskGuard reports issues, you fix them.

---

## State Queries

### Reading State

**All tasks:**
```bash
taskguard list
```
- Walks `tasks/` directory
- Parses all `.md` files
- Builds in-memory task graph

**Filtered:**
```bash
taskguard list --area backend --status todo
```
- Same process, apply filters

**Validation:**
```bash
taskguard validate
```
- Load all tasks
- Build dependency graph
- Analyze for issues

**Performance:** O(n) for n tasks. Fast even for 1000+ tasks.

---

## State Modifications

### Task Updates

**Status change:**
```bash
taskguard update status backend-001 done
```

**Process:**
1. Load `tasks/backend/backend-001.md`
2. Parse YAML
3. Update `status: todo` → `status: done`
4. Serialize YAML
5. Write file

**Atomic:** File write is atomic. No partial updates.

### Bulk Operations

**Not supported:** TaskGuard doesn't support bulk updates.

**Why:** Prevents accidental data loss. You update one task at a time.

**Workaround for bulk:**
```bash
# Use shell loops
for id in backend-001 backend-002 backend-003; do
  taskguard update status $id done
done
```

---

## State Backup & Recovery

### Backup Strategy

**Git is your backup:**
```bash
# Every commit is a backup
git log --oneline tasks/

# Restore to any point
git checkout <commit> tasks/
```

**Manual backup:**
```bash
# Copy tasks directory
cp -r tasks/ tasks-backup-$(date +%Y%m%d)

# Or create git tag
git tag -a v1.0-tasks -m "Backup before refactor"
```

### Recovery

**Undo last change:**
```bash
git checkout HEAD tasks/backend/backend-001.md
```

**Restore deleted task:**
```bash
git log --all --full-history -- tasks/backend/backend-001.md
git checkout <commit> -- tasks/backend/backend-001.md
```

**Restore entire state:**
```bash
git checkout <commit> tasks/
```

---

## State Migration

### Upgrading TaskGuard

**Minor versions (0.2.x → 0.2.y):**
- State format unchanged
- No migration needed

**Major versions (0.2.x → 0.3.0):**
- May include migration script
- Run: `taskguard migrate`
- Always backup first: `git tag pre-migration`

---

## Performance Considerations

### Scalability

**Number of tasks:**
- ✅ 1-100 tasks: Instant
- ✅ 100-1000 tasks: Fast (<100ms)
- ⚠️ 1000+ tasks: Slower (still <1s)

**Optimization:**
- TaskGuard caches file reads
- Parallel I/O where possible
- Lazy loading for large repos

### File Size

**Recommended task file size:** <50KB

**Large tasks:**
- Split into multiple tasks
- Move detailed docs to separate files
- Link from task file

---

## State Debugging

### Inspect State

**Check task file:**
```bash
cat tasks/backend/backend-001.md
```

**Validate YAML:**
```bash
taskguard validate
```

**Git history:**
```bash
git log -p tasks/backend/backend-001.md
```

### Common Issues

**Parse error:**
```bash
taskguard list
# Warning: Skipping tasks/backend/backend-001.md: Invalid YAML
```

**Fix:** Check YAML syntax, ensure `---` delimiters

**Missing dependency:**
```bash
taskguard validate
# Error: backend-001 depends on missing task 'setup-099'
```

**Fix:** Update `dependencies` or create missing task

---

## Best Practices

### 1. Commit Tasks Frequently
```bash
git add tasks/
git commit -m "Add authentication tasks"
```

### 2. Use Branches for Experiments
```bash
git checkout -b task-reorganization
# Modify tasks
# Merge or discard
```

### 3. Tag Milestones
```bash
git tag -a v1.0-complete -m "All v1.0 tasks done"
```

### 4. Regular Validation
```bash
taskguard validate
# Run before commits
```

### 5. Small Task Files
- Keep under 50KB
- Split large tasks
- Link to external docs

---

## Next Steps

- [Task Structure](task-structure.md) - Understand task file format
- [Dependencies](../features/dependencies.md) - Master dependency management
- [Git Sync](../features/git-sync.md) - Automate with Git integration
