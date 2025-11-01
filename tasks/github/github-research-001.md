---
id: github-research-001
title: GitHub Projects v2 API Research and Priority Update
status: done
priority: critical
tags: [research, graphql, projects-v2, api-updates]
dependencies: []
assignee: developer
created: 2025-01-30T23:00:00Z
estimate: 1h
complexity: 3
area: github
---

# GitHub Projects v2 API Research and Priority Update

## Context
**CRITICAL PRIORITY SHIFT**: Community users requested **GitHub Dashboard (Projects v2 boards)** visibility, NOT just Issues. The existing task breakdown (github-infra-001 through github-sync-001) was focused on Issues-only. This research task documents 2025 GraphQL API updates and updates implementation priorities.

## Key Finding: Projects v2 Dashboard is Primary Goal

### What Users Want
```
TaskGuard Local Tasks → GitHub Projects Board → See on Dashboard
```

**NOT**:
```
TaskGuard Tasks → GitHub Issues (no board)
```

### Architecture
```
TaskGuard (Local)          GitHub GraphQL API       GitHub Projects Dashboard
─────────────────          ──────────────────       ─────────────────────────
tasks/backend-001.md  →    Issue #123         →     Project Board
  status: doing               state: OPEN             Column: "In Progress"
  priority: high              —                       Priority: High
```

## 2025 GraphQL API Updates

### ✅ New Features (Available Now)

1. **ISSUE_ADVANCED Search Type (March 2025)**
   - Complex issue queries with AND/OR
   - Becomes default Sept 4, 2025
   - **Use**: Better filtering when syncing issues to project

2. **ProjectV2StatusUpdate Object (June 2024)**
   - GraphQL object for project status updates
   - View, create, update, delete status updates
   - **Use**: MUST include in types.rs

3. **Enhanced project_v2_item Webhook**
   - Field changes include previous/current values
   - **Use**: Real-time sync (future enhancement)

4. **convertProjectV2DraftIssueItemToIssue Mutation**
   - Convert drafts to issues
   - **Use**: Optional feature

### ⚠️ Deprecations

1. **Projects Classic** - Removed April 1, 2025
   - **Impact**: MUST use Projects v2 (which we are)

## Updated Implementation Priority

### OLD Plan (Issues-Only)
```
infra-002 → infra-003 → infra-004 → infra-005 → sync-001
(Client)    (Mapper)    (Mutations)  (Queries)   (Command)
Focus: Create issues, sync status
```

### NEW Plan (Projects v2 Dashboard Priority)
```
infra-002 → infra-003 → infra-004 → infra-005 → sync-001
(Client)    (Mapper)    (Mutations)  (Queries)   (Command)
Focus: Create issues + add to project board + sync columns
```

### Key Changes Needed

#### github-infra-002 (Types & Client)
**ADD**:
- `ProjectItem` struct (from guide)
- `FieldValue` struct (from guide)
- `ProjectV2StatusUpdate` struct (NEW 2025 API)
- `project_number` in GitHubConfig

**PRIORITY**: Projects v2 types are PRIMARY, not optional

#### github-infra-003 (Mapper)
**ADD**:
- `project_item_id` to TaskMapping
- Status mapping for Projects columns
- Helper to get project field IDs

#### github-infra-004 (Mutations)
**ADD**:
- `add_issue_to_project()` mutation
- `update_project_item_status()` mutation
- `get_status_field_info()` query (field IDs)

**PRIORITY**: These are CORE features, not enhancements

#### github-infra-005 (Queries)
**ADD**:
- `get_project_id()` query
- `get_project_items()` query with pagination
- Parse project item field values

#### github-sync-001 (Command)
**CHANGE**:
- Sync to Projects board, not just issues
- Map TaskGuard status → Project columns
- Update project item status on sync

## Updated Causality Chain

```
✅ github-infra-001 (DONE) - Git Commit Tracking
    ↓
⭕ github-infra-002 (UPDATED) - GitHub API Module with Projects v2 Types
    ↓ MUST include: ProjectItem, FieldValue, ProjectV2StatusUpdate
🚫 github-infra-003 (UPDATED) - Task-Issue-Project Mapper
    ↓ MUST include: project_item_id mapping
🚫 github-infra-004 (UPDATED) - GitHub Mutations (Issues + Projects)
    ↓ MUST include: add_issue_to_project, update_project_item_status
🚫 github-infra-005 (UPDATED) - GitHub Queries (Issues + Projects)
    ↓ MUST include: get_project_id, get_project_items
🚫 github-sync-001 (UPDATED) - Sync to Projects Dashboard
    🎯 DELIVERS: Tasks visible on GitHub Projects board
```

## Implementation Checklist

### Phase 1: Foundation (github-infra-002)
- [ ] Add Projects v2 types to types.rs
  - [ ] ProjectItem
  - [ ] FieldValue
  - [ ] ProjectV2StatusUpdate (NEW)
- [ ] Update GitHubConfig to include project_number
- [ ] GraphQL client supports Projects v2 queries

### Phase 2: Mapper (github-infra-003)
- [ ] Add project_item_id to TaskMapping
- [ ] Map TaskGuard status → Project column names
- [ ] Helper to cache project field IDs

### Phase 3: Mutations (github-infra-004)
- [ ] Implement add_issue_to_project()
- [ ] Implement update_project_item_status()
- [ ] Implement get_status_field_info()

### Phase 4: Queries (github-infra-005)
- [ ] Implement get_project_id()
- [ ] Implement get_project_items() with pagination
- [ ] Parse project field values

### Phase 5: Command (github-sync-001)
- [ ] Push: Create issue → Add to project → Set status
- [ ] Pull: Read project items → Update local tasks
- [ ] Bidirectional: Detect conflicts on project board

## Success Criteria

After implementation, users should see:

```bash
$ taskguard sync --github

🌐 GITHUB PROJECTS SYNC
   Project: "My Project" (https://github.com/users/me/projects/1)

📤 PUSH: Local Tasks → GitHub Projects
   ➕ backend-001 - Implement JWT Auth
      ✅ Created issue #123
      ✅ Added to project board
      ✅ Set status: In Progress

📊 SUMMARY
   Created: 1 issue
   Added to project: 1 item
   Status updated: 1 item

🔗 View on GitHub: https://github.com/users/me/projects/1
```

**User visits dashboard and sees**:
- Project board with columns (Todo, In Progress, Done, etc.)
- TaskGuard tasks as cards in correct columns
- Status matches local TaskGuard status

## Next Steps

1. ✅ Update github-infra-002 task to include Projects v2 types
2. ✅ Update github-infra-003 through github-sync-001 tasks
3. Start implementation with github-infra-002 (updated scope)

## References

- [GitHub Changelog: API Updates March 2025](https://github.blog/changelog/2025-03-06-github-issues-projects-api-support-for-issues-advanced-search-and-more/)
- [GitHub Docs: Using API to Manage Projects](https://docs.github.com/en/issues/planning-and-tracking-with-projects/automating-your-project/using-the-api-to-manage-projects)
- [GITHUB_INTEGRATION_GUIDE.md](../../GITHUB_INTEGRATION_GUIDE.md) - Already includes Projects v2 implementation
- [GraphQL API Changelog](https://docs.github.com/en/graphql/overview/changelog)
