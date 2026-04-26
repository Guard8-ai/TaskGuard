# Integration Patterns

Real-world patterns for integrating TaskGuard with other tools and workflows.

---

## Git Workflow Integration

Reference task IDs in commit messages and use `taskguard sync` to get status suggestions based on your git history.

```bash
# Include task ID in your commit message
git commit -m "backend-001: Implement JWT auth middleware"
git commit -m "feat(api): rate limiting — closes api-003"

# Analyze recent commits and get status suggestions
taskguard sync
taskguard sync --limit 100 --verbose

# Check remote branch too
taskguard sync --remote
```

TaskGuard detects task IDs matching `{area}-{NNN}` in commit messages and suggests status transitions (e.g. `doing → done` when it sees "fix", "closes", "implement"). Suggestions are printed — never applied automatically.

**Tip:** Make it a habit to include the task ID in every commit. This creates an auditable trail linking code changes to task decisions.

---

## Claude Code Integration

TaskGuard is designed for AI-agent-first workflows. After `taskguard init`:

1. `AGENTIC_AI_TASKGUARD_GUIDE.md` is copied to your project root.
2. Add auto-detection to `~/.claude/CLAUDE.md`:

```markdown
## TaskGuard
If `.taskguard/` exists in the project: read AGENTIC_AI_TASKGUARD_GUIDE.md
before starting any work.
```

Claude Code will then automatically discover TaskGuard in any project and follow the dependency/causality workflow.

**Recommended AI workflow:**

```bash
# AI: "what should I work on next?"
taskguard ai "what should I work on next?"

# AI announces completion
taskguard ai "I just finished backend-002"

# Human reviews and updates
taskguard update status backend-002 done
taskguard validate
```

---

## Bulk Import with AI-Generated Plans

Use the `import-md` command to convert AI-generated analysis documents into TaskGuard tasks.

**Step 1 — Ask AI to write a plan:**

```
Write a markdown analysis of what needs to be done to add OAuth2 support.
Use this format for each item:
### Task #N: Title
**Priority:** high/medium/low
**Effort:** Xh
**Area:** backend/api/auth
**Dependencies:** setup-001
Description of the work.
```

**Step 2 — Preview and import:**

```bash
# Preview without creating files
taskguard import-md oauth-plan.md --area auth --dry-run

# Import for real
taskguard import-md oauth-plan.md --area auth

# Validate the resulting task graph
taskguard validate --orphans
```

**Step 3 — Fix any orphans the AI missed:**

```bash
taskguard update dependencies auth-002 "auth-001"
taskguard validate
```

---

## GitHub Issues / Projects v2

Bidirectional sync between TaskGuard and GitHub.

**Setup:**

```bash
# Create .taskguard/github.toml
cat > .taskguard/github.toml << 'EOF'
owner = "my-org"
repo = "my-repo"
project_number = 3
EOF

# Authenticate (uses gh CLI token)
gh auth status
```

**Daily workflow:**

```bash
# Create tasks locally first, then push to GitHub
taskguard create --title "Add caching layer" --area backend --dependencies "setup-001"
taskguard sync --github

# Preview what would happen
taskguard sync --github --dry-run

# Add existing issues to the Projects board
taskguard sync --github --backfill-project
```

**Archive and restore flow:**

```bash
# Archive done tasks → closes GitHub issues
taskguard archive

# Reopen a task (and its GitHub issue)
taskguard restore backend-003
```

---

## CI/CD: Validate Task Graph on Every Push

Catch circular dependencies and orphan tasks in CI before they reach humans.

```yaml
# .github/workflows/taskguard.yml
name: Validate Tasks

on:
  push:
    paths:
      - 'tasks/**'
      - '.taskguard/config.toml'
  pull_request:
    paths:
      - 'tasks/**'

jobs:
  validate:
    name: Task Graph Validation
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          FORCE_JAVASCRIPT_ACTIONS_TO_NODE24: true

      - name: Install TaskGuard
        run: cargo install taskguard

      - name: Validate task graph
        run: taskguard validate

      - name: Check for orphan tasks
        run: taskguard validate --orphans
```

---

## Pre-commit Hook

Block commits that would introduce orphan tasks or circular dependencies.

```bash
# .git/hooks/pre-commit
#!/bin/bash
if command -v taskguard &> /dev/null; then
  echo "Validating TaskGuard task graph..."
  if ! taskguard validate; then
    echo "❌ Task validation failed. Fix issues before committing."
    exit 1
  fi
fi
```

```bash
chmod +x .git/hooks/pre-commit
```

---

## Shell Aliases

Common shortcuts for daily use:

```bash
# ~/.bashrc or ~/.zshrc
alias tg='taskguard'
alias tgl='taskguard list'
alias tgv='taskguard validate'
alias tgd='taskguard update status'         # tgd backend-001 done
alias tgs='taskguard sync'
alias tgai='taskguard ai'
```

---

## Scripting with Task Output

Since task files are plain Markdown with YAML front-matter, they're easy to process with standard tools.

```bash
# Count tasks by status
grep -r "^status:" tasks/ | awk -F': ' '{print $2}' | sort | uniq -c

# Find all high-priority doing tasks
grep -rl "priority: high" tasks/ | xargs grep -l "status: doing"

# List all task IDs
grep -r "^id:" tasks/ | sed 's/.*id: //'
```
