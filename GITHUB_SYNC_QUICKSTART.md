# GitHub Projects Sync - Quick Start

## TL;DR

Add GitHub Projects bidirectional sync to TaskGuard in ~15 hours of development.

## What This Adds

```bash
# Setup integration
taskguard sync github --setup

# Push tasks to GitHub → Create issues & update project board
taskguard sync github --push

# Pull from GitHub → Update local tasks from issue changes
taskguard sync github --pull

# Two-way sync with conflict resolution
taskguard sync github --bidirectional
```

## Architecture

```
TaskGuard Task          GitHub Issue           GitHub Project Board
───────────────         ────────────           ────────────────────
backend-001.md    →     Issue #123       →     Status: In Progress
  status: doing           state: OPEN            Column: Doing
  title: "Auth"           title: "Auth"
```

## Quick Implementation Steps

### 1. Add Dependencies (2 min)

```toml
# Cargo.toml
[dependencies]
reqwest = { version = "0.11", features = ["json", "blocking"] }
serde_json = "1.0"
dotenv = "0.15"
```

### 2. Create Module Structure (5 min)

```bash
mkdir -p src/github
touch src/github/{mod.rs,client.rs,queries.rs,mutations.rs,types.rs,mapper.rs}
```

### 3. Copy Implementation (2-3 hours)

Follow detailed code in `GITHUB_INTEGRATION_GUIDE.md`

Key files:
- `src/github/types.rs` - Data structures
- `src/github/client.rs` - GraphQL client
- `src/github/queries.rs` - Read from GitHub
- `src/github/mutations.rs` - Write to GitHub
- `src/github/mapper.rs` - Task ↔ Issue conversion
- `src/commands/sync.rs` - CLI command extension

### 4. Test (1 hour)

```bash
# Set token
export GITHUB_TOKEN="ghp_your_token"

# Setup
taskguard sync github --setup

# Dry run
taskguard sync github --push --dry-run

# Real sync
taskguard sync github --push
```

## Status Mapping

| TaskGuard | GitHub Projects |
|-----------|----------------|
| todo      | Todo           |
| doing     | In Progress    |
| review    | In Review      |
| done      | Done           |
| blocked   | Blocked        |

## Configuration

```toml
# .taskguard/config.toml
[github]
enabled = true
token_env = "GITHUB_TOKEN"
repository = "owner/repo"
project_number = 1
sync_mode = "bidirectional"
```

## State Storage

```
.taskguard/
└── state/
    └── github_mapping.toml    # task_id → issue_number mapping
```

**Add to .gitignore:**
```
.taskguard/state/github_mapping.toml
.env
```

## GraphQL Queries You'll Need

### Get Project ID
```graphql
query($owner: String!, $repo: String!, $number: Int!) {
  repository(owner: $owner, name: $repo) {
    projectV2(number: $number) {
      id
    }
  }
}
```

### Create Issue
```graphql
mutation($repositoryId: ID!, $title: String!, $body: String!) {
  createIssue(input: {
    repositoryId: $repositoryId,
    title: $title,
    body: $body
  }) {
    issue {
      id
      number
    }
  }
}
```

### Add to Project
```graphql
mutation($projectId: ID!, $contentId: ID!) {
  addProjectV2ItemById(input: {
    projectId: $projectId,
    contentId: $contentId
  }) {
    item {
      id
    }
  }
}
```

## Development Workflow

1. **Start with types** - Define data structures
2. **Build client** - HTTP + GraphQL wrapper
3. **Add queries** - Read-only operations (test these first!)
4. **Add mutations** - Write operations (test carefully!)
5. **Build mapper** - Conversion logic
6. **Extend CLI** - User interface

## Testing Strategy

### Unit Tests
```rust
#[test]
fn test_status_conversion() {
    assert_eq!(
        mapper::taskguard_to_github(&TaskStatus::Doing),
        "In Progress"
    );
}
```

### Integration Tests
```bash
# Create test repo/project on GitHub
export GITHUB_TOKEN="test_token"
export TEST_REPO="your-test/repo"

# Run tests
cargo test github_integration
```

### Manual Testing
1. Create 2-3 test tasks locally
2. Push to GitHub with dry-run
3. Verify in browser
4. Push for real
5. Make changes on GitHub
6. Pull back
7. Verify local changes

## Common Issues & Solutions

### "Token not found"
```bash
export GITHUB_TOKEN="ghp_xxxxx"
```

### "Project not found"
Get project number from URL:
```
https://github.com/users/YOU/projects/3
                                      ↑ This is the number
```

### "GraphQL errors"
Test query in GitHub's GraphQL Explorer:
https://docs.github.com/en/graphql/overview/explorer

### "Rate limiting"
GitHub API has rate limits:
- 5000 requests/hour with token
- Use pagination for large repos
- Cache project metadata

## Security Checklist

- [ ] Token stored in environment variable (not code)
- [ ] .gitignore includes `.taskguard/state/`
- [ ] Token has minimal scopes (`repo`, `project`)
- [ ] No token in logs/output
- [ ] Mapping file not committed

## Performance Tips

1. **Batch operations** - Update multiple items per request when possible
2. **Cache metadata** - Store project ID, field IDs locally
3. **Lazy loading** - Only fetch when needed
4. **Rate limit handling** - Respect GitHub limits
5. **Pagination** - Use cursors for large datasets

## Future Enhancements

After basic sync works:

1. **Webhooks** - Real-time sync on GitHub events
2. **Labels** - Sync TaskGuard tags ↔ GitHub labels
3. **Assignees** - Map assignee field
4. **Dependencies** - Create linked issues
5. **Comments** - Sync task updates ↔ issue comments
6. **Milestones** - Group tasks by project milestones
7. **Auto-sync** - Trigger on `taskguard list` etc.

## Debugging

Enable verbose logging:
```bash
RUST_LOG=debug taskguard sync github --push 2> debug.log
```

Print GraphQL responses:
```rust
dbg!(&result);  // In client.rs execute_query()
```

Test individual queries:
```bash
# Use curl or GitHub CLI
gh api graphql -f query='...'
```

## Estimated Timeline

| Task | Time |
|------|------|
| Module structure | 30 min |
| Types & client | 2 hours |
| Queries | 2 hours |
| Mutations | 2 hours |
| Mapper | 2 hours |
| Sync logic | 3 hours |
| CLI integration | 1 hour |
| Testing | 3 hours |
| **Total** | **~15 hours** |

## Resources

- [GitHub GraphQL API Docs](https://docs.github.com/en/graphql)
- [GitHub Projects V2 API](https://docs.github.com/en/issues/planning-and-tracking-with-projects/automating-your-project/using-the-api-to-manage-projects)
- [GraphQL Explorer](https://docs.github.com/en/graphql/overview/explorer)
- [TaskGuard Git Sync Code](src/commands/sync.rs) - Similar patterns

## Getting Help

1. Check `GITHUB_INTEGRATION_GUIDE.md` for detailed implementation
2. Test queries in GraphQL Explorer first
3. Use `--dry-run` flag extensively
4. Review existing `sync.rs` for patterns
5. GitHub API has great error messages - read them!

## Success Criteria

✅ Can push new tasks → creates issues + adds to project
✅ Can pull changes → updates local task status
✅ Detects conflicts → prompts user for resolution
✅ Maintains task-issue mapping
✅ Handles errors gracefully
✅ Respects rate limits
✅ Secure (no token leaks)

## Next Steps

1. Read `GITHUB_INTEGRATION_GUIDE.md` for full implementation
2. Set up test GitHub repository and project
3. Get GitHub personal access token
4. Start with types.rs and client.rs
5. Test queries before mutations
6. Build incrementally with tests

Good luck! 🚀
