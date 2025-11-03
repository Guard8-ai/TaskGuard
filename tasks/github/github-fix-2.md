---
id: github-fix-2
title: Archive Command GitHub Issues Integration
status: todo
priority: medium
tags:
- github
- fix
dependencies: [github-infra-005]
assignee: developer
created: 2025-10-30T14:22:23.822200055Z
estimate: 2h
complexity: 5
area: github
---

# Archive Command GitHub Issues Integration

## Context
Now that we have Git commit tracking (github-infra-001) and full GitHub API infrastructure (github-infra-002 through 005), we can integrate GitHub Issues with the archive command.

When archiving done tasks that have associated GitHub issues, we should close those issues automatically.

## Dependencies
**Requires:** github-infra-005 (Complete GitHub API: client, mapper, mutations, queries)
**Why:** Need mapper to find issues, mutations to close them

## Implementation

**Location:** `src/commands/archive.rs`

```rust
pub fn run(dry_run: bool, _days: Option<u32>) -> Result<()> {
    // ... existing code to find tasks ...

    // NEW: Check for GitHub integration
    let github_enabled = is_github_sync_enabled()?;
    let mut mapper = if github_enabled {
        Some(load_github_mapper()?)
    } else {
        None
    };

    let mut files_to_archive = Vec::new();
    let mut github_issues_to_close = Vec::new();

    for entry in WalkDir::new(&tasks_dir) {
        // ... existing task loading ...

        if task.status == TaskStatus::Done {
            files_to_archive.push((path, task.clone()));

            // NEW: Check if task has GitHub issue
            if let Some(ref mapper) = mapper {
                if let Some(mapping) = mapper.get_mapping(&task.id) {
                    github_issues_to_close.push((
                        task.id.clone(),
                        mapping.issue_number,
                        mapping.project_item_id.clone(),
                    ));
                }
            }
        }
    }

    // Display archive plan
    println!("📋 ARCHIVE SUMMARY");
    println!("   Tasks to archive: {}", files_to_archive.len());

    if github_enabled && !github_issues_to_close.is_empty() {
        println!();
        println!("🌐 GITHUB INTEGRATION");
        println!("   The following GitHub issues will be closed:");
        for (task_id, issue_num, _) in &github_issues_to_close {
            println!("   📌 {} → Issue #{} (will close)", task_id, issue_num);
        }
        println!();
        println!("   ⚠️  Archived tasks will remain synced via .taskguard/archive/");
    }

    if dry_run {
        println!("🔍 DRY RUN MODE - No files moved, no issues closed");
        return Ok(());
    }

    // Archive files (existing logic)
    for (path, task) in files_to_archive {
        // ... existing archive logic ...
    }

    // NEW: Close GitHub issues
    if github_enabled {
        let client = create_github_client()?;

        for (task_id, issue_num, _project_item_id) in github_issues_to_close {
            println!("   🌐 Closing GitHub issue #{} for {}", issue_num, task_id);

            // Get issue ID from issue number
            if let Ok(issue_id) = get_issue_id(&client, issue_num) {
                match GitHubMutations::update_issue_state(&client, &issue_id, "closed") {
                    Ok(_) => {
                        println!("      ✅ Closed issue #{}", issue_num);

                        // Update mapping to reflect archived status
                        if let Some(ref mut mapper) = mapper {
                            // Keep mapping but mark as archived
                            // (Don't delete - need to track synced tasks)
                        }
                    }
                    Err(e) => {
                        println!("      ⚠️  Failed to close issue #{}: {}", issue_num, e);
                    }
                }
            }
        }

        // Save updated mapping
        if let Some(mapper) = mapper {
            mapper.save()?;
        }
    }

    Ok(())
}
```


## Bug Found During Analysis (2025-11-03)

**Issue:** Currently archived tasks are completely skipped during sync (src/commands/sync.rs:459-462)

**Impact:**
1. If a task is marked `done` and archived BEFORE first sync, no GitHub issue is created
2. The 26 existing archived tasks (created Sept-Oct 2025) have NO GitHub issues
3. `--backfill-project` flag also skips archived tasks (line 729-733)

**Root Cause:**
```rust
// Line 459-462 in sync.rs
if task.file_path.to_string_lossy().contains("archive") {
    continue;  // Skips archived tasks entirely
}
```

**Related Issue:** This blocks the ability to retroactively create GitHub issues for completed work

## Technical Notes
Location: `src/commands/archive.rs`