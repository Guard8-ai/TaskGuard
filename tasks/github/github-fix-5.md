---
id: github-fix-5
title: Add GitHub-Aware Validation
status: done
priority: medium
tags:
- github
- fix
dependencies: []
assignee: developer
created: 2025-10-30T14:22:23.822472944Z
estimate: ~
complexity: 4
area: github
---

**Location:** `src/commands/validate.rs`

```rust
pub fn run() -> Result<()> {
    // ... existing validation ...

    // NEW: GitHub sync validation
    if is_github_sync_enabled()? {
        println!();
        println!("🌐 GITHUB SYNC VALIDATION");

        let mapper = load_github_mapper()?;
        let all_tasks = load_all_tasks_including_archive()?;
        let task_ids: HashSet<_> = all_tasks.iter().map(|t| t.id.clone()).collect();

        let mut orphaned_mappings = Vec::new();
        let mut archived_synced_tasks = Vec::new();

        for (task_id, mapping) in mapper.all_mappings() {
            if !task_ids.contains(task_id) {
                orphaned_mappings.push((task_id.clone(), mapping.issue_number));
            } else if let Some(task) = all_tasks.iter().find(|t| t.id == *task_id) {
                if task.is_archived {
                    archived_synced_tasks.push((task_id.clone(), mapping.issue_number));
                }
            }
        }

        if !orphaned_mappings.is_empty() {
            println!("   ⚠️  ORPHANED MAPPINGS (task deleted but mapping remains):");
            for (task_id, issue_num) in orphaned_mappings {
                println!("      {} → Issue #{} (task not found)", task_id, issue_num);
            }
        }

        if !archived_synced_tasks.is_empty() {
            println!("   📦 ARCHIVED SYNCED TASKS:");
            for (task_id, issue_num) in archived_synced_tasks {
                println!("      {} → Issue #{} (task archived)", task_id, issue_num);
            }
        }
    }

    Ok(())
}
```


## Technical Notes
Location: `src/commands/validate.rs`