---
id: backend-030
title: Fix archived task file path in GitHub issue body
status: todo
priority: high
tags:
- backend
- bug
- sync
- github
dependencies:
- setup-001
assignee: developer
created: 2025-12-21T11:41:26.000695523Z
estimate: ~
complexity: 3
area: backend
---

# Fix archived task file path in GitHub issue body

## Context

When `taskguard sync --github` creates an issue for an archived task, the issue body contains the wrong file path:
- **Current:** `tasks/{area}/{id}.md`
- **Expected:** `.taskguard/archive/{area}/{id}.md`

## Bug Location

`src/commands/sync.rs` line 914:
```rust
let file_path = format!("tasks/{}/{}.md", task.area, task.id);
```

Should check `is_archived` flag and use correct path.

## Tasks
- [ ] Fix file path in `create_or_update_missing_issues()` (line ~914)
- [ ] Also fix in `add_new_tasks_to_project()` if applicable
- [ ] Add test case for archived task sync
- [ ] Build + test + verify

## Acceptance Criteria
- [ ] Archived task issues show `.taskguard/archive/{area}/{id}.md` path
- [ ] Active task issues show `tasks/{area}/{id}.md` path
- [ ] All tests pass

---
**Session Handoff** (fill when done):
- Changed: [files/functions modified]
- Causality: [what triggers what]
- Verify: [how to test this works]
- Next: [context for dependent tasks]
