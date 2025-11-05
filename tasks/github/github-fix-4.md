---
id: github-fix-4
title: Update GitHub Sync to Handle Archive
status: done
priority: medium
tags:
- github
- fix
dependencies: []
assignee: developer
created: 2025-10-30T14:22:23.822394165Z
estimate: ~
complexity: 4
area: github
---


**Location:** `src/commands/sync.rs` - Update GitHub sync functions

```rust
fn pull_from_github(
    client: &GitHubClient,
    tasks: &[TaskWithLocation],  // ← Now includes archived tasks
    mapper: &mut TaskIssueMapper,
    dry_run: bool,
) -> Result<()> {
    println!("📥 Pulling updates from GitHub...\n");

    let issues = GitHubQueries::get_repository_issues(client)?;

    for issue in issues {
        if let Some(task_id) = mapper.get_task_id_by_issue(issue.number) {
            println!("📝 Checking issue #{} → {}", issue.number, task_id);

            // Find the task (including archived)
            if let Some(task_with_loc) = tasks.iter().find(|t| t.task.id == task_id) {
                if task_with_loc.is_archived {
                    println!("   📦 Task is archived");

                    // Option 1: Update archived task file
                    // Option 2: Warn but don't update
                    // Option 3: Unarchive if GitHub status changed

                    if task_with_loc.task.status != map_github_status(&issue.state) {
                        println!("   ⚠️  GitHub status changed but task is archived");
                        println!("      Local: {:?} (archived)", task_with_loc.task.status);
                        println!("      GitHub: {}", issue.state);
                        println!("      Run 'taskguard restore {}' to sync", task_id);
                    }
                } else {
                    // Normal sync for active tasks
                    // ... existing update logic ...
                }
            } else {
                println!("   ⚠️  Task not found: {}", task_id);
            }
        }
    }

    Ok(())
}
```


## Bug Found During Analysis (2025-11-03)

**Current Behavior:**
Sync completely skips archived tasks at two locations:
- Line 459-462: `push_tasks_to_github()` skips archived tasks
- Line 729-733: `backfill_project_board()` skips archived tasks

**Problems:**
1. Tasks archived before first sync never get GitHub issues created
2. Can't use backfill to retroactively sync old completed work
3. 26 archived tasks have no GitHub representation

**Example Scenario:**
```
1. Task marked done
2. Task archived (moved to .taskguard/archive/)
3. Run sync --github → SKIPPED (no issue created)
4. Run sync --github --backfill-project → ALSO SKIPPED
```

**Fix Required:**
- Remove archive skip in normal sync (or add flag to include archived)
- Add `--include-archived` flag for backfill
- Create closed issues for archived tasks
- Keep them on Projects board in "Done" column

## Technical Notes
Location: `src/commands/sync.rs` - Update GitHub sync functions