---
id: github-001
title: Auto-add synced issues to GitHub Projects v2 board
status: done
priority: critical
tags:
- github
dependencies: []
assignee: developer
created: 2025-11-01T20:00:41.260090453Z
estimate: ~
complexity: 3
area: github
---

# Auto-add synced issues to GitHub Projects v2 board

> **⚠️ SESSION WORKFLOW NOTICE (for AI Agents):**
>
> **This task should be completed in ONE dedicated session.**
>
> When you mark this task as `done`, you MUST:
> 1. Fill the "Session Handoff" section at the bottom with complete implementation details
> 2. Document what was changed, what runtime behavior to expect, and what dependencies were affected
> 3. Create a clear handoff for the developer/next AI agent working on dependent tasks
>
> **If this task has dependents,** the next task will be handled in a NEW session and depends on your handoff for context.

## Context
Currently `taskguard sync --github` creates GitHub Issues but doesn't add them to the Projects v2 board. Users want to see tasks in their GitHub Projects kanban view (like the screenshot shows) to track progress visually.

**Reference Screenshot:**
![GitHub Projects v2 Board Example](file:///data/eliran/Downloads/WhatsApp%20Image%202025-10-22%20at%2012.20.34.jpeg)

*This is how the Projects v2 board should look after implementation - with tasks organized in columns by status.*

**Current State:**
- ✅ Issues created in repository
- ❌ Issues NOT added to Projects v2 board
- ❌ Status columns not synced

**Desired State:**
- ✅ Issues automatically added to project board
- ✅ Status set to match TaskGuard status (todo → Backlog, doing → In Progress, etc.)
- ✅ Users can see all tasks in Projects v2 kanban view

## Objectives
1. **Auto-add issues to project** - When sync creates/updates an issue, automatically add it to the configured Projects v2 board
2. **Sync status columns** - Map TaskGuard status (todo/doing/review/done/blocked) to GitHub Projects v2 status field
3. **Update project_item_id** - Save the project item ID in the mapping file for future updates
4. **Handle status changes** - When task status changes locally, update the column in Projects v2

## Tasks
- [ ] Add project integration after issue creation in `push_tasks_to_github()`
- [ ] Call `GitHubMutations::add_issue_to_project()` for new issues
- [ ] Get project status field info using `GitHubMutations::get_status_field_info()`
- [ ] Map TaskGuard status to Projects v2 column using `TaskIssueMapper::find_best_status_option()`
- [ ] Call `GitHubMutations::update_project_item_status()` to set column
- [ ] Update mapping with `project_item_id`
- [ ] Handle status updates for existing issues
- [ ] Test with real GitHub Projects v2 board

## Acceptance Criteria
✅ **Issue Creation:**
- When `taskguard sync --github` creates a new issue, it's automatically added to the Projects v2 board
- Issue appears in the correct status column based on task status

✅ **Status Mapping:**
- TaskStatus::Todo → "Backlog" or "To Do" column
- TaskStatus::Doing → "In Progress" column
- TaskStatus::Review → "In Review" column
- TaskStatus::Done → "Done" column
- TaskStatus::Blocked → "Blocked" column (or Backlog if no blocked column)

✅ **Status Updates:**
- Changing task status locally (e.g., todo → doing) and re-syncing updates the Projects v2 column
- Updates are reflected in GitHub Projects kanban view

✅ **Error Handling:**
- Graceful handling if project doesn't exist
- Clear error if project_number in config is invalid
- Handles missing status columns (uses best fallback)

## Technical Notes

**Available Infrastructure (Already Implemented):**
```rust
// Add issue to project (returns project_item_id)
GitHubMutations::add_issue_to_project(client, project_id, issue_id) -> Result<String>

// Get status field info (returns field_id and available options)
GitHubMutations::get_status_field_info(client, project_id) -> Result<(String, Vec<(String, String)>)>

// Find best matching column
TaskIssueMapper::find_best_status_option(status, available_options) -> Option<String>

// Update item status
GitHubMutations::update_project_item_status(client, project_id, item_id, field_id, option_id) -> Result<()>

// Get project ID from number
GitHubQueries::get_project_id(client, owner, number) -> Result<String>
```

**Implementation Location:**
- File: `src/commands/sync.rs`
- Function: `push_tasks_to_github()`
- After creating issue (around line 502), add project integration

**Mapping Storage:**
- Update `IssueMapping.project_item_id` field (currently empty string)
- Save updated mapping after project operations

## Testing

**Manual Testing:**
```bash
# 1. Create a test task
taskguard create --title "Test Projects Integration" --area testing

# 2. Sync to GitHub (should add to project board)
taskguard sync --github

# 3. Check GitHub Projects board - task should appear in Backlog

# 4. Change task status to 'doing'
taskguard update status testing-001 doing

# 5. Re-sync
taskguard sync --github

# 6. Check GitHub Projects - task should move to "In Progress" column
```

**Expected Output:**
```
🌐 GITHUB SYNC MODE

📤 PUSH: Local Tasks → GitHub Issues
   ➕ testing-001 - Test Projects Integration (creating issue)
      ✅ Created issue #44
      📋 Adding to project...
      ✅ Added to project (item ID: PVTI_xxx)
      🎯 Setting status to 'Backlog'
      ✅ Status set successfully

📊 PUSH SUMMARY
   Created: 1 (+ added to project)
   Updated: 0
   Skipped: 38
```

**Edge Cases:**
- [ ] Test with task status that has no matching column (should use fallback)
- [ ] Test when project doesn't exist (should show clear error)
- [ ] Test when project has custom column names
- [ ] Test status update for already-synced task

## Version Control
- [ ] Commit changes incrementally with clear messages
- [ ] Use descriptive commit messages that explain the "why"
- [ ] Consider creating a feature branch for complex changes
- [ ] Review changes before committing

## Updates
- 2025-11-01: Task created
- 2025-11-01: Implementation completed and tested successfully

## Session Handoff (AI: Complete this when marking task done)
**For the next session/agent working on dependent tasks:**

### What Changed
**Modified Files:**
- `src/commands/sync.rs` - Added Projects v2 board integration to `push_tasks_to_github()` function
- `src/github/setup.rs` - Made `get_project_id()` function public (was private)

**New Runtime Behavior:**
1. **Issue Creation Flow (lines 515-568):**
   - When creating a GitHub issue, immediately adds it to Projects v2 board
   - Retrieves project ID from config using `GitHubProjectSetup::get_project_id()`
   - Adds issue to project via `GitHubMutations::add_issue_to_project()`
   - Gets status field configuration from the project
   - Maps TaskGuard status to best matching column using `TaskIssueMapper::find_best_status_option()`
   - Sets the column status via `GitHubMutations::update_project_item_status()`
   - Saves `project_item_id` in mapping for future updates

2. **Status Update Flow (lines 473-498):**
   - When updating existing issues with status changes, now also updates Projects v2 column
   - Only updates if `project_item_id` is not empty (issue is on board)
   - Uses same status mapping logic as creation flow

**Console Output Changes:**
- New messages during sync: "📋 Adding to project...", "✅ Added to project", "🎯 Status set successfully"
- Status updates show: "🎯 Updating project status...", "✅ Updated project column"

### Causality Impact
**Causal Chain Created:**
```
taskguard sync --github
  └─> push_tasks_to_github()
      ├─> Create GitHub Issue
      └─> Add to Projects v2 Board (NEW!)
          ├─> Get project ID
          ├─> Add issue to project
          ├─> Get status field info
          ├─> Map TaskGuard status → Project column
          ├─> Set column status
          └─> Save project_item_id to mapping
```

**Event Triggers:**
- **Issue Creation** → Automatic project board addition
- **Status Change** (local task) → Updates both issue state AND project column on next sync
- **Sync Operation** → Validates project exists, handles missing columns gracefully

**No Async Flows:** All operations are synchronous GraphQL mutations

### Dependencies & Integration
**No New Dependencies Added** - Uses existing infrastructure:
- `GitHubProjectSetup::get_project_id()` - Now public instead of private
- `GitHubMutations::add_issue_to_project()`
- `GitHubMutations::get_status_field_info()`
- `GitHubMutations::update_project_item_status()`
- `TaskIssueMapper::find_best_status_option()`

**Integration Points:**
- Integrates seamlessly with existing `push_tasks_to_github()` workflow
- Uses `.taskguard/github.toml` config for `project_number`
- Saves `project_item_id` to `.taskguard/github-mapping.json`
- Error handling with `.context()` for clear error messages

**Affected Areas:**
- All tasks synced with `taskguard sync --github` now appear on Projects v2 board
- Status changes propagate to both issue state (open/closed) and project column

### Verification & Testing
**Verified Working:**
```bash
./target/release/taskguard sync --github
```
- ✅ Created issue #44 for github-001
- ✅ Added to project (item ID: PVTI_lADODcinKs4BHAJUzggm9Pg)
- ✅ Status set to "Todo" column
- ✅ Mapping file updated with project_item_id

**How to Test:**
1. Create a new task: `taskguard create --title "Test" --area testing`
2. Run sync: `taskguard sync --github`
3. Check GitHub Projects board - issue should appear in appropriate column
4. Change task status locally, re-sync
5. Verify column updates on GitHub Projects board

**Known Edge Cases:**
- If `project_number` in config is invalid → Clear error message
- If status has no matching column → Uses best fallback from `find_best_status_option()`
- If project doesn't exist → GraphQL error with context
- Old issues (created before this implementation) have empty `project_item_id` → Won't update columns until manually added to board

### Context for Next Task
**Important Decisions:**
1. **Made `get_project_id()` public** in `GitHubProjectSetup` - This allows other modules to retrieve project IDs without duplicating GraphQL queries
2. **Integrated into existing sync flow** - No new commands, seamlessly extends `taskguard sync --github`
3. **Graceful fallback** - Uses `find_best_status_option()` for fuzzy column matching (handles custom board layouts)
4. **Only updates columns if on board** - Checks `!mapping.project_item_id.is_empty()` before attempting status updates

**Gotchas:**
- **Old issues need manual migration** - Issues created before this implementation (project_item_id is empty) won't get status updates. To fix: manually add them to board or re-create issues
- **Project must exist** - Config must have valid `project_number` before sync works
- **Status mapping is fuzzy** - "todo" → "Backlog"/"To Do"/"Todo" (whichever exists)

**Next Steps for Developer:**
- Consider adding bulk migration command to add old issues to board
- Consider adding `--skip-project` flag for users who don't want board integration
- Test with custom project board layouts (non-standard column names)
