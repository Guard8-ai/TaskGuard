# Advanced Configuration

Customize TaskGuard behavior via `.taskguard/config.toml`.

---

## Configuration File

```toml
[project]
name = "My Project"
version = "0.3.0"
areas = ["setup", "backend", "frontend", "api", "auth", "testing", "deployment"]

[settings]
statuses = ["todo", "doing", "review", "done", "blocked"]
priorities = ["low", "medium", "high", "critical"]
complexity_scale = "1-10"
default_estimate_unit = "hours"

[git]
auto_add_tasks = true
auto_commit_on_status_change = false

[ai]
enabled = true
claude_code_integration = true
```

---

## Custom Areas

Add project-specific areas:

```toml
[project]
areas = ["planning", "design", "backend", "frontend", "mobile", "testing", "docs"]
```

New areas are auto-added when you create tasks:

```bash
taskguard create --title "Task" --area mobile  # Adds 'mobile' to config
```

---

## Custom Templates

Override default templates per area:

```
.taskguard/
├── templates/
│   ├── backend.md     # Custom backend template
│   ├── frontend.md    # Custom frontend template
│   └── _default.md    # Fallback for all areas
```

Template priority:
1. `.taskguard/templates/{area}.md`
2. `.taskguard/templates/_default.md`
3. Built-in domain-specific template

---

## GitHub Configuration

`.taskguard/github.toml`:

```toml
owner = "your-username"
repo = "your-repo"
project_number = 1
```

---

## State Files

Local state (gitignored):

```
.taskguard/
├── state/
│   └── github-mapping.json  # Task-to-issue mapping
├── archive/                  # Archived tasks
```

---

## Environment Variables

```bash
TASKGUARD_ROOT=/path/to/project  # Override project root
RUST_LOG=debug                   # Enable debug logging
```
