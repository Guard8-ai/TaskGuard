# Efficiency Commands

## Test Commands Reference

### 1. Stats Command - Storage Analysis
```bash
taskguard stats
```
**What it analyzes:**
- Scans: `tasks/` directory recursively
- Shows: Total storage, file counts, average sizes
- Breakdown by: area (backend, frontend, etc.)
- Breakdown by: status (todo, done, doing, etc.)
- Top 10 largest task files

**Expected output:**
- Total storage in KB/MB
- Each area's file count and size
- Status distribution
- List of largest tasks

---

### 2. Clean Command - Delete Completed Tasks
```bash
# DRY RUN first (safe, shows what would be deleted)
taskguard clean --dry-run

# Actually delete (removes files)
taskguard clean
```
**What it does:**
- Finds: ALL tasks with `status: done` (no age filtering)
- Deletes: Those task files from `tasks/` directory
- Shows: Files deleted, space saved

**Expected output:**
- If no completed tasks: "No cleanup needed"
- If completed tasks found: List of ALL files to delete + size savings

---

### 3. Archive Command - Preserve Completed Tasks
```bash
# DRY RUN first
taskguard archive --dry-run

# Actually archive
taskguard archive
```
**What it does:**
- Finds: ALL tasks with `status: done` (no age filtering)
- Archives: Those tasks to preserve history without cluttering active tasks

**Where files are saved:**
- **From:** `tasks/backend/backend-001.md`
- **To:** `.taskguard/archive/backend/backend-001.md`
- Files moved (not copied), preserves structure by area

**Expected output:**
- List of ALL completed tasks to archive
- Archive location: `.taskguard/archive/`
- Files moved and freed space shown

---

### 4. Compact Command - Reduce File Sizes
```bash
# DRY RUN first
taskguard compact --dry-run

# Actually compact
taskguard compact
```
**What it does:**
- Removes: Excessive empty lines (max 1 consecutive)
- Removes: Trailing whitespace
- Keeps: YAML structure intact
- Typical: 15-30% size reduction

**Expected output:**
- Per-file: before/after sizes and % reduction
- Total: files compacted, space saved

---

## Storage Efficiency Workflow

### Initial Analysis
```bash
# 1. Check current storage usage
taskguard stats

# 2. See potential file size savings
taskguard compact --dry-run
```

### Cleanup Strategy

**Option A: Preserve History (Archive)**
```bash
# Archive ALL completed tasks to preserve history
taskguard archive --dry-run
taskguard archive
```

**Option B: Delete Completed Tasks (Clean)**
```bash
# Permanently delete ALL completed tasks
taskguard clean --dry-run
taskguard clean
```

**Option C: Both**
```bash
# 1. Compact files first (reduce size)
taskguard compact

# 2. Then archive or clean completed tasks
taskguard archive  # OR taskguard clean
```

### Binary Size Optimization
- **Before optimization:** 5.8M
- **After optimization:** 2.8M (51% reduction)
- **Savings:** 3MB per installation

## Safety Notes
- Always use `--dry-run` first to preview changes
- `clean` permanently deletes files (use `archive` to preserve)
- `archive` moves files to `.taskguard/archive/` (can be restored)
- `compact` modifies files in-place (preserves structure)
- All commands show detailed summaries before executing
