---
id: github-fix-3
title: Clean Command GitHub Protection
status: todo
priority: medium
tags:
- github
- fix
dependencies: []
assignee: developer
created: 2025-10-30T14:22:23.822308036Z
estimate: ~
complexity: 6
area: github
---


**Location:** `src/commands/clean.rs`

```rust
pub fn run(dry_run: bool, _days: Option<u32>) -> Result<()> {
    // ... existing code ...

    // NEW: Check for GitHub integration BEFORE allowing clean
    let github_enabled = is_github_sync_enabled()?;

    if github_enabled {
        let mapper = load_github_mapper()?;
        let mut synced_tasks = Vec::new();

        for (path, task_id, title) in &files_to_delete {
            if mapper.get_mapping(task_id).is_some() {
                synced_tasks.push((task_id.clone(), title.clone()));
            }
        }

        if !synced_tasks.is_empty() {
            println!("⚠️  GITHUB SYNC PROTECTION");
            println!();
            println!("   The following tasks are synced with GitHub:");
            for (id, title) in &synced_tasks {
                println!("   🌐 {} - {}", id, title);
            }
            println!();
            println!("   ❌ BLOCKED: Cannot delete synced tasks with 'clean'");
            println!();
            println!("💡 OPTIONS:");
            println!("   1. Use 'taskguard archive' instead (preserves history + closes GitHub issues)");
            println!("   2. Run 'taskguard sync github --close-issues' then clean");
            println!("   3. Disable GitHub sync in .taskguard/config.toml");
            println!();

            // Remove synced tasks from deletion list
            files_to_delete.retain(|(_, id, _)| {
                !synced_tasks.iter().any(|(synced_id, _)| synced_id == id)
            });

            if files_to_delete.is_empty() {
                println!("   ℹ️  No non-synced tasks to clean");
                return Ok(());
            }

            println!("   ℹ️  Continuing with {} non-synced tasks", files_to_delete.len());
            println!();
        }
    }

    // ... rest of existing clean logic ...
}
```


## Technical Notes
Location: `src/commands/clean.rs`