---
id: github-003
title: Add task file links to GitHub issue descriptions
status: todo
priority: medium
tags:
- github
- sync
- enhancement
dependencies: [github-002]
assignee: developer
created: 2025-11-03T19:49:11.283052119Z
estimate: 30m
complexity: 2
area: github
---

# Add task file links to GitHub issue descriptions

## Context

GitHub issue descriptions currently don't include a link to the actual TaskGuard task file in the repository. This makes it hard to navigate from GitHub dashboard back to the source task for full context.

**Current issue body:**
```
**TaskGuard ID:** github-fix-2

## Description

Context section content here...

---
*Synced from TaskGuard*
```

**Desired issue body:**
```
**TaskGuard ID:** github-fix-2
**Task File:** [tasks/github/github-fix-2.md](https://github.com/Guard8-ai/TaskGuard/blob/master/tasks/github/github-fix-2.md)

## Description

Context section content here...

---
*Synced from TaskGuard*
```

## Dependencies

**Requires:** github-002 (Context section extraction)
**Why:** Should be done after improving descriptions to avoid double-sync updates

## Objectives

- Add clickable task file link to GitHub issue descriptions
- Use repository base URL from config
- Construct correct GitHub blob URL with branch
- Apply to both new issues and existing issues (via update sync)

## Tasks

- [ ] Get repository URL from GitHub config
- [ ] Construct file path from task.area and task.id
- [ ] Build GitHub blob URL (e.g., `/blob/master/tasks/{area}/{id}.md`)
- [ ] Update issue body format in `src/commands/sync.rs`
- [ ] Test link navigation from GitHub to task file

## Implementation

**Location:** `src/commands/sync.rs` lines 541-545

**Updated body format:**
```rust
// Get repo URL from config
let repo_url = format!("https://github.com/{}/{}", config.owner, config.repo);
let file_path = format!("tasks/{}/{}.md", task.area, task.id);
let file_url = format!("{}/blob/master/{}", repo_url, file_path);

let body = format!(
    "**TaskGuard ID:** {}  \n**Task File:** [{}]({})\n\n## Description\n\n{}\n\n---\n*Synced from TaskGuard*",
    task.id,
    file_path,
    file_url,
    description
);
```

**Notes:**
- Uses `master` branch by default (could be made configurable)
- Double space + `\n` creates proper line break in markdown
- File path constructed from `task.area` and `task.id`

## Acceptance Criteria

✅ **Links work correctly:**
- Click task file link in GitHub issue → opens correct file on GitHub
- Link format: `https://github.com/Guard8-ai/TaskGuard/blob/master/tasks/{area}/{id}.md`

✅ **Applied to all syncs:**
- New issues created with links
- Existing issues updated with links on next sync
- Links work for all task areas (backend, github, tools, etc.)

✅ **Handles edge cases:**
- Tasks in subdirectories work correctly
- Special characters in task IDs handled
- Works with dry-run mode

## Technical Notes

**Branch detection (future enhancement):**
Could detect current branch instead of hardcoding `master`:
```rust
let repo = Repository::open(".")?;
let branch = repo.head()?.shorthand().unwrap_or("master");
```

**Config location:**
GitHub config is in `.taskguard/github.toml`:
```toml
owner = "Guard8-ai"
repo = "TaskGuard"
project_number = 1
```

## Updates
- 2025-11-03: Task created
- Depends on github-002 for description improvements
