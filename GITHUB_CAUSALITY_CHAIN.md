# GitHub Integration Causality Chain - Complete End-to-End Flow

## 🎯 End Goal: See TaskGuard Tasks in GitHub Dashboard

Users should be able to run `taskguard sync --github` and see all their tasks appear as issues in their GitHub repository dashboard.

## 📊 Complete Causality Chain

```
✅ github-infra-001 (DONE) - Git Commit Tracking
│   What: Archive creates commits with task IDs
│   Why: Foundation for tracking task lifecycle
│   
└──> ⭕ github-infra-002 (READY) - GitHub API Module Foundation
    │   What: GraphQL client, types, configuration
    │   Why: Need client before we can make API calls
    │   Estimate: 4h
    │
    └──> ⭕ github-infra-003 (BLOCKED) - Task-Issue Mapper
        │   What: Persistent mapping between tasks and issues
        │   Why: Prevent duplicate issue creation
        │   Estimate: 3h
        │
        └──> ⭕ github-infra-004 (BLOCKED) - GitHub Mutations
            │   What: create_issue, update_issue_state, close_issue
            │   Why: Write operations to GitHub
            │   Estimate: 4h
            │
            └──> ⭕ github-infra-005 (BLOCKED) - GitHub Queries
                │   What: get_repository_issues, get_issue_by_number
                │   Why: Read operations from GitHub
                │   Estimate: 3h
                │
                └──> ⭕ github-sync-001 (BLOCKED) - Add --github Flag to Sync Command
                    │   🎯 THIS DELIVERS THE END GOAL
                    │   What: taskguard sync --github (bidirectional sync)
                    │   Why: Push local tasks → GitHub Issues
                    │   Estimate: 4h
                    │
                    └──> OPTIONAL ENHANCEMENTS (BLOCKED)
                        │
                        ├──> github-fix-2: Archive Integration
                        │    Close GitHub issues when archiving tasks
                        │
                        ├──> github-fix-3: Clean Protection
                        │    Prevent deletion of tasks with active GitHub issues
                        │
                        ├──> github-fix-4: Sync Archive Support
                        │    Sync archived tasks with GitHub
                        │
                        ├──> github-fix-5: GitHub Validation
                        │    Validate task-issue consistency
                        │
                        └──> github-fix-6: Restore Command
                             Restore archived tasks, reopen GitHub issues
```

## 🚀 Critical Path to End Goal

**PHASE 1: Foundation** (✅ 2h - DONE)
- github-infra-001: Git commit tracking

**PHASE 2: API Infrastructure** (⏱️ 14h - TODO)
- github-infra-002: GitHub API client (4h)
- github-infra-003: Task-Issue mapper (3h)
- github-infra-004: GitHub mutations (4h)
- github-infra-005: GitHub queries (3h)

**PHASE 3: User-Facing Feature** (⏱️ 4h - TODO)
- github-sync-001: `taskguard sync --github` command (4h)

**Total Critical Path: 20 hours**

After Phase 3 completes, users can:
```bash
taskguard sync --github
```

And see their tasks appear in GitHub Issues! 🎉

## 📝 What Happens When You Run `taskguard sync --github`

### First Run (No Issues Exist)
```bash
$ taskguard sync --github

🌐 GITHUB SYNC MODE
   Syncing local tasks with GitHub Issues...

📤 PUSH: Local Tasks → GitHub Issues
   ➕ backend-001 - Implement JWT Auth (creating issue)
      ✅ Created issue #123
   ➕ frontend-001 - Login Page (creating issue)
      ✅ Created issue #124
   ➕ testing-001 - Auth Tests (creating issue)
      ✅ Created issue #125

📊 PUSH SUMMARY
   Created: 3
   Updated: 0
   Skipped: 0 (already in sync)

📥 PULL: GitHub Issues → Local Tasks
   ✅ All tasks in sync with GitHub

✅ Sync mapping saved
```

### Subsequent Runs (Issues Exist)
```bash
$ taskguard sync --github

🌐 GITHUB SYNC MODE
   Syncing local tasks with GitHub Issues...

📤 PUSH: Local Tasks → GitHub Issues
   🔄 backend-001 - Implement JWT Auth (status mismatch)
      Local: Done, GitHub: OPEN
      ✅ Updated GitHub issue to CLOSED

📊 PUSH SUMMARY
   Created: 0
   Updated: 1
   Skipped: 2 (already in sync)

📥 PULL: GitHub Issues → Local Tasks
   ✅ All tasks in sync with GitHub

✅ Sync mapping saved
```

## 🗂️ Task Status Overview

| Phase | Task | Status | Complexity | Estimate | Blocks |
|-------|------|--------|------------|----------|--------|
| 1 | github-infra-001 | ✅ DONE | 4 | 2h | infra-002 |
| 2 | github-infra-002 | ⭕ TODO | 7 | 4h | infra-003 |
| 2 | github-infra-003 | ⭕ TODO | 6 | 3h | infra-004 |
| 2 | github-infra-004 | ⭕ TODO | 7 | 4h | infra-005 |
| 2 | github-infra-005 | ⭕ TODO | 6 | 3h | sync-001 |
| 3 | github-sync-001 | ⭕ TODO | 8 | 4h | 🎯 **GOAL** |

**Critical Path Total:** 20 hours (2h done + 18h remaining)

## 🎯 Next Session Goal

**Implement github-infra-002: GitHub API Module Foundation**

This is the next blocking task. Once complete, it unblocks the entire GitHub infrastructure chain.

**Files to create:**
- `src/github/mod.rs`
- `src/github/client.rs`
- `src/github/types.rs`
- `src/github/config.rs`

**Dependencies to add:**
```toml
reqwest = { version = "0.11", features = ["json", "blocking"] }
serde_json = "1.0"
```

**Verification:**
```rust
// Should compile and run
let client = GitHubClient::new("token".to_string())?;
let response = client.query(query, variables)?;
```

## 💡 Why This Order Matters

1. **infra-001 (Git tracking)** - Provides value immediately, works without GitHub
2. **infra-002 (Client)** - Can't call GitHub API without a client
3. **infra-003 (Mapper)** - Can't prevent duplicate issues without mapping
4. **infra-004 (Mutations)** - Can't create/update issues without write operations
5. **infra-005 (Queries)** - Can't sync without reading existing issues
6. **sync-001 (Command)** - Orchestrates all the pieces into user-facing feature

Each task builds on the previous, ensuring no "missing dependency" scenarios.

## 🎉 Success Criteria

After completing the critical path, users should be able to:

1. **Configure GitHub integration:**
   ```bash
   cat > .taskguard/github.toml <<EOF
   token = "github_pat_..."
   owner = "username"
   repo = "repo-name"
   EOF
   ```

2. **Sync tasks to GitHub:**
   ```bash
   taskguard sync --github
   ```

3. **See tasks in GitHub dashboard:**
   - Visit `https://github.com/username/repo-name/issues`
   - See all TaskGuard tasks as GitHub Issues
   - Issue titles match task titles
   - Issue states match task statuses (done=closed, other=open)

4. **Bidirectional sync works:**
   - Close issue on GitHub → shows warning in next sync
   - Update task status locally → updates GitHub issue
   - Mapping preserved in `.taskguard/github-mapping.json`

That's the complete end-to-end flow! 🚀
