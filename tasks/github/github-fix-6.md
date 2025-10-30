---
id: github-fix-6
title: Add Restore Command
status: todo
priority: medium
tags:
- github
- fix
dependencies: []
assignee: developer
created: 2025-10-30T14:22:23.822696839Z
estimate: ~
complexity: 8
area: github
---


**Location:** `src/commands/restore.rs` (NEW FILE)

```rust
use anyhow::{Context, Result};
use std::fs;
use crate::config::{get_tasks_dir, find_taskguard_root};
use crate::task::Task;

/// Restore archived task back to active tasks
pub fn run(task_id: &str, dry_run: bool) -> Result<()> {
    let root = find_taskguard_root()
        .context("Not in a TaskGuard project")?;
    let archive_dir = root.join(".taskguard").join("archive");
    let tasks_dir = get_tasks_dir()?;

    // Find archived task
    let archived_task = find_archived_task(&archive_dir, task_id)?;

    println!("📦 RESTORE ARCHIVED TASK");
    println!("   Task: {} - {}", archived_task.id, archived_task.title);
    println!("   From: {}", /* archive path */);
    println!("   To: {}", /* tasks path */);

    if dry_run {
        println!("🔍 DRY RUN - No files moved");
        return Ok(());
    }

    // Move back to tasks/
    // Update GitHub sync if needed

    println!("✅ Task restored successfully");
    Ok(())
}
```

---

## 6. Recommended Workflow for Using Both Features

### Safe Workflow

```bash
# 1. Check GitHub sync status BEFORE archiving
$ taskguard validate

# 2. Use archive (NOT clean) for completed tasks
$ taskguard archive --dry-run  # Preview
$ taskguard archive            # Archives + closes GitHub issues

# 3. GitHub sync continues to work with archived tasks
$ taskguard sync github --pull  # Still finds archived tasks

# 4. If GitHub status changes, restore task
$ taskguard restore backend-001  # Moves back to tasks/
$ taskguard sync github --pull   # Now can update
```

### Unsafe Workflow (AVOID)

```bash
# ❌ DON'T: Archive without GitHub sync awareness
$ taskguard archive  # Orphans GitHub issues

# ❌ DON'T: Clean synced tasks
$ taskguard clean  # Deletes files but leaves GitHub issues open

# ❌ DON'T: Sync after archiving without Fix #1
$ taskguard sync github --pull  # Fails to find archived tasks
```

---

## 7. Breaking Scenarios Summary

| Scenario | Current Behavior | Impact | Required Fix |
|----------|-----------------|--------|--------------|
| **Archive synced task** | Moves file, leaves GitHub issue open, mapping intact | GitHub issue orphaned, sync breaks | Fix #2: Close GitHub issue when archiving |
| **Clean synced task** | Deletes file, leaves GitHub issue open, mapping intact | Permanent data loss, orphaned GitHub issue | Fix #3: Block deletion of synced tasks |
| **Pull updates for archived task** | Sync can't find task (not in tasks/) | Status divergence, sync fails | Fix #1: Include archive in task loading |
| **Archive + pull bidirectional** | Sync doesn't detect conflicts | Silent sync failure, state divergence | Fix #4: Handle archived tasks in sync |
| **Clean + mapping file stale** | Mapping points to deleted file | Sync errors, orphaned mappings | Fix #5: Validate GitHub mappings |
| **GitHub status change after archive** | No way to update archived task | Permanent state divergence | Fix #6: Add restore command |

---

## 8. Implementation Priority

### Phase 1: Critical Blocking Issues (MUST FIX BEFORE GITHUB SYNC)

1. **Fix #1** - Update `load_all_tasks()` to include archive directory
   - Priority: CRITICAL
   - Effort: 2 hours
   - Blocks: All GitHub sync operations

2. **Fix #3** - Add GitHub sync protection to clean command
   - Priority: CRITICAL
   - Effort: 3 hours
   - Prevents: Permanent data loss

### Phase 2: GitHub Integration (REQUIRED FOR SYNC)

3. **Fix #2** - Archive command closes GitHub issues
   - Priority: HIGH
   - Effort: 4 hours
   - Enables: Safe archiving with sync

4. **Fix #4** - GitHub sync handles archived tasks
   - Priority: HIGH
   - Effort: 4 hours
   - Enables: Pull updates for archived tasks

### Phase 3: Quality of Life (RECOMMENDED)

5. **Fix #5** - GitHub-aware validation
   - Priority: MEDIUM
   - Effort: 2 hours
   - Provides: Early warning of sync issues

6. **Fix #6** - Restore command
   - Priority: MEDIUM
   - Effort: 3 hours
   - Enables: Unarchiving when needed

**Total Estimated Effort:** 18 hours

---

## 9. Testing Requirements

### Test Suite for GitHub + Archive Compatibility

```rust
#[cfg(test)]
mod github_archive_tests {
    #[test]
    fn test_load_includes_archived_tasks() {
        // Create task, archive it, verify load_all_tasks() finds it
    }

    #[test]
    fn test_archive_closes_github_issue() {
        // Archive synced task, verify GitHub issue closed
    }

    #[test]
    fn test_clean_blocks_synced_tasks() {
        // Try to clean synced task, verify blocked
    }

    #[test]
    fn test_pull_updates_archived_task() {
        // Change GitHub issue status, pull, verify archived task updated
    }

    #[test]
    fn test_restore_enables_sync() {
        // Archive task, restore it, verify sync works
    }

    #[test]
    fn test_validate_detects_orphaned_mappings() {
        // Delete task, verify validate warns about orphaned mapping
    }
}
```

---

## 10. Decision: Pull PR #2 or Wait?

### Option A: Pull Now + Hotfix (RECOMMENDED)

**Pros:**
- Get Termux support and stats/compact commands
- Archive concept is valuable, just needs GitHub awareness
- Can fix issues incrementally

**Cons:**
- Must immediately disable archive/clean or add warnings
- Requires 18 hours of additional work

**Action Plan:**
```bash
# 1. Pull PR #2
git pull origin master

# 2. Add warning to archive/clean commands
# Modify src/commands/archive.rs and clean.rs to warn users

# 3. Implement fixes incrementally
# Start with Fix #1 (critical for GitHub sync)

# 4. Test thoroughly before enabling
```

### Option B: Wait for Fixes (CONSERVATIVE)

**Pros:**
- No broken commands in codebase
- Clean implementation timeline

**Cons:**
- Delays getting valuable efficiency commands
- No immediate pressure to fix issues

---

## 11. Conclusion

**Status:** 🚨 **BLOCKING** - GitHub Projects integration cannot proceed without addressing these issues

**Severity Assessment:**
- Archive command: **HIGH** severity - Breaks sync, orphans issues
- Clean command: **CRITICAL** severity - Permanent data loss + orphaned issues
- Overall impact: **BLOCKING** for GitHub integration

**Recommendation:** Implement **Fix #1** and **Fix #3** BEFORE starting GitHub Projects integration. These are critical to prevent data corruption and sync failures.

**Timeline Impact:**
- Original GitHub sync estimate: 15 hours
[ ] Additional fixes required: 18 hours
- **Total effort: 33 hours** (more than doubled)

**Next Steps:**
1. Review and approve this analysis
2. Decide: Pull PR #2 now or wait?
3. If pulling: Implement critical fixes first (Fix #1, #3)
4. Then proceed with GitHub integration
5. Add comprehensive test suite

## Technical Notes
Location: `src/commands/restore.rs` (NEW FILE)