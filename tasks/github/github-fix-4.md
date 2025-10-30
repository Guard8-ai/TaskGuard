---
id: github-fix-4
title: Update GitHub Sync to Handle Archive
status: todo
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


## Technical Notes
Location: `src/commands/sync.rs` - Update GitHub sync functions