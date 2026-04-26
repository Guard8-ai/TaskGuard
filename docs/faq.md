# Frequently Asked Questions

---

## General

### What is TaskGuard?

TaskGuard is a local-first, Git-native task management CLI built in Rust, designed specifically for AI agent workflows. Tasks are plain Markdown files with YAML front-matter, stored in your project's `tasks/` directory and committed to Git like source code.

### Why not just use GitHub Issues, Jira, or Linear?

Those tools are cloud-based and optimized for human UIs. TaskGuard is different in three ways:

1. **Local-first** — no cloud, no account, works offline, zero latency.
2. **Git-native** — tasks evolve with the code in the same repository. `git blame` tells you exactly when and why a task was created.
3. **AI-agent-first** — mandatory dependency chains, causality tracking, and session handoff sections are designed for AI agents, not just humans.

### What makes it "AI-agent-first"?

- Every task must declare what it depends on, forming a DAG that agents can traverse to understand execution order.
- The `CAUTION` keyword on validation errors is chosen deliberately — LLMs respond more strongly to CAUTION than to generic warnings.
- Each task template includes a **Session Handoff** section so a stateless AI agent can pick up completed work in a new session.
- `taskguard init` copies `AGENTIC_AI_TASKGUARD_GUIDE.md` to the project root, which AI agents auto-discover.

### Is TaskGuard free?

TaskGuard is open source under the MIT License with Commons Clause. You can use, modify, and distribute it freely. The Commons Clause prevents selling TaskGuard itself as a hosted service.

---

## Setup

### What platforms does TaskGuard support?

Linux, macOS, Windows, and WSL2. Pre-built binaries for all three are available on the [releases page](https://github.com/Guard8-ai/TaskGuard/releases).

### Do I need Rust installed to use TaskGuard?

No — download a pre-built binary from the releases page. Rust is only required if you want to build from source (`cargo install taskguard`).

### Can I use TaskGuard without a GitHub account?

Yes. GitHub integration is entirely optional. The core workflow (create, list, update, validate, sync with git history) works with no network access whatsoever.

---

## Tasks and Data

### Where are tasks stored?

In `tasks/{area}/{area}-{NNN}.md` files inside your project directory. They are committed to Git like any other file.

### Can I edit task files directly in my text editor?

Yes. Task files are plain Markdown with YAML front-matter. Any editor works. Just be careful with the YAML front-matter syntax (no tabs, required fields present).

### How do I back up my tasks?

Tasks are in Git — every commit is a backup. Push to a remote (`git push`) and your tasks are backed up. Use `git tag` to mark important milestones.

### What happens to archived tasks?

Archived tasks are moved to `.taskguard/archive/{area}/` — still in Git, still queryable with `taskguard list --include-archive`. They are just separated from the active task list.

### Can task IDs be reused after archiving?

No. When generating a new ID, TaskGuard scans both active and archived tasks to find the current maximum for that area, then increments. `backend-003` will never be reused if it was once archived.

---

## Dependencies and Causality

### Why are dependencies mandatory?

Mandatory dependencies enforce **causality tracking** — every task must trace back to `setup-001`. This creates a directed acyclic graph (DAG) where:

- Agents know which tasks are available (all dependencies done) vs blocked.
- The task tree documents *why* each task exists, not just *what* it does.
- Orphan tasks (disconnected work with no clear origin) are detected and flagged.

### When should I use `--allow-orphan-task`?

For genuine research spikes, exploratory tasks, or other work that has no logical predecessor. Use sparingly. A `CAUTION` is printed if you create orphans via `import-md`.

### Can a task depend on an archived task?

Yes. Archived dependencies are shown with a `📦` suffix in `taskguard validate` output. A done + archived task is still a valid dependency.

### What is a circular dependency and how do I fix it?

A circular dependency is when task A depends on B, and B depends on A (directly or transitively). Use `taskguard validate` to identify the cycle, then remove one of the conflicting dependency edges.

---

## AI Integration

### How do I set up Claude Code to use TaskGuard?

Add this to `~/.claude/CLAUDE.md` (your global Claude Code memory):

```markdown
## TaskGuard
If `.taskguard/` exists in the project root: read AGENTIC_AI_TASKGUARD_GUIDE.md
before starting any work. Follow the dependency workflow.
```

Claude Code will then auto-discover and use TaskGuard in every project that has been initialized.

### What is the Session Handoff section for?

AI agents are stateless — each session starts fresh. The Session Handoff section at the bottom of each task file records:
- Which files/functions were changed
- How the causality chain connects (what calls what)
- How to verify the work
- Context for whoever works on dependent tasks next

This allows the next AI session to pick up exactly where the previous one left off.

### Can TaskGuard call Claude or another LLM directly?

No. The `taskguard ai` command is fully in-process pattern matching — no external API calls, no API keys, no network. This is intentional: it keeps TaskGuard free, fast, and offline.

---

## GitHub Integration

### Do I need a GitHub project board set up before syncing?

TaskGuard will create a Projects v2 board automatically if one doesn't exist (with `--github` sync). You only need to set `project_number` if you have an existing board you want to use.

### What happens to GitHub issues when I archive a task?

The corresponding issue is closed on GitHub, and `is_archived: true` is set in `.taskguard/github-mapping.json`. `taskguard restore` will reopen it.

### Will syncing create duplicate issues if I run it multiple times?

No. TaskGuard detects duplicates by embedding the task ID and a content hash in the issue body. Re-syncing on the same branch with the same task content is idempotent.

---

## Customization

### Can I add custom task areas?

Yes. Either create a `tasks/{area}/` directory manually and run `taskguard validate --sync-areas`, or simply create a task with `--area my-new-area` and the area is auto-added to `config.toml`.

### Can I customize the task template?

Yes. Create `.taskguard/templates/{area}.md` with your custom template. Use `{{title}}` and `{{date}}` as placeholders. See [Configuration](api-reference/configuration.md#custom-templates) for details.

### Can I add custom statuses or priorities?

The status and priority enums are hardcoded in the binary for now. The `config.toml` `[settings]` section is informational but does not extend the valid values.
