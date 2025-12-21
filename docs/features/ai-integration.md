# AI Integration

Zero-setup integration with Claude Code and other AI agents.

---

## Quick Start

TaskGuard automatically provides AI guidance:

```bash
taskguard init  # Creates AGENTIC_AI_TASKGUARD_GUIDE.md
```

AI agents can use TaskGuard immediately via CLI commands.

---

## Core Commands for AI

```bash
# Create tasks (dependencies required in v0.4.0+)
taskguard create --title "Task" --area backend --priority high --dependencies "setup-001"

# View available work and orphans
taskguard validate
taskguard validate --orphans

# Update status
taskguard update status <id> doing

# Mark complete
taskguard update status <id> done
```

!!! warning "CAUTION: Causality Tracking"
    AI agents must use `--dependencies` when creating tasks.
    `setup-001` is auto-created by `taskguard init` as the root.

---

## Domain-Specific Templates

TaskGuard v0.3.0 provides tailored templates for each area with causation chain prompts:

| Area | Focus |
|------|-------|
| `api` | Request lifecycle, middleware, routes |
| `auth` | Authentication flow, tokens, sessions |
| `backend` | Service orchestration, DI, errors |
| `data` | Schema, queries, transactions |
| `frontend` | Component lifecycle, state, effects |
| `testing` | Fixtures, assertions, isolation |

Each template includes:

- **Causation Chain Prompt**: Trace execution flow
- **Pre-flight Checks**: Verification commands
- **Session Handoff**: Context for next session

---

## Custom Templates

Override templates in `.taskguard/templates/{area}.md`:

```bash
mkdir -p .taskguard/templates
# Create custom template
vim .taskguard/templates/backend.md
```

Template variables:
- `{{title}}` - Task title
- `{{date}}` - Creation date

---

## AI Guide Reference

See `AGENTIC_AI_TASKGUARD_GUIDE.md` for complete AI workflow:

- Command reference
- Status flow
- Dependency patterns
- Common mistakes
- GitHub sync

---

## Natural Language (Future)

```bash
taskguard ai "what should I work on next?"
```
