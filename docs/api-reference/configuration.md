# Configuration Reference

TaskGuard uses two configuration files, both located in `.taskguard/`.

---

## `.taskguard/config.toml`

Project-level configuration. Created by `taskguard init`. **Committed to Git.**

### `[project]` section

```toml
[project]
name = "My Project"
version = "0.2.2"
areas = ["setup", "backend", "frontend", "api", "auth", "testing", "deployment"]
```

| Key | Type | Description |
|-----|------|-------------|
| `name` | String | Project name (informational) |
| `version` | String | TaskGuard config schema version |
| `areas` | List\<String\> | Active task areas. New areas are auto-added when you create a task in an area not yet listed. |

### `[settings]` section

```toml
[settings]
statuses = ["todo", "doing", "review", "done", "blocked"]
priorities = ["low", "medium", "high", "critical"]
complexity_scale = "1-10"
default_estimate_unit = "hours"
```

| Key | Type | Description |
|-----|------|-------------|
| `statuses` | List\<String\> | Valid status values |
| `priorities` | List\<String\> | Valid priority values |
| `complexity_scale` | String | Complexity score range (informational) |
| `default_estimate_unit` | String | Default unit for time estimates |

### `[git]` section

```toml
[git]
auto_add_tasks = true
auto_commit_on_status_change = false
commit_message_template = "Task {{id}}: {{action}} - {{title}}"
```

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `auto_add_tasks` | Bool | `true` | Whether `taskguard sync` suggests adding task IDs found in commits |
| `auto_commit_on_status_change` | Bool | `false` | Auto-commit when status is updated (not yet implemented) |
| `commit_message_template` | String | — | Template for generated commit messages (`{{id}}`, `{{action}}`, `{{title}}` placeholders) |

### `[ai]` section

```toml
[ai]
enabled = true
claude_code_integration = true
auto_suggestions = true
complexity_analysis = true
```

| Key | Type | Description |
|-----|------|-------------|
| `enabled` | Bool | Enable AI integration features |
| `claude_code_integration` | Bool | Enable Claude Code specific features (guide auto-copy on init) |
| `auto_suggestions` | Bool | Enable auto-suggestions in `taskguard ai` output |
| `complexity_analysis` | Bool | Enable complexity scoring in `taskguard lint` |

---

## `.taskguard/github.toml`

GitHub integration configuration. **Optional.** Created manually. **Committed to Git.**

When this file is present, `taskguard sync --github`, `taskguard archive`, and `taskguard restore` will interact with GitHub Issues and Projects v2.

```toml
owner = "your-org-or-username"
repo = "your-repository"
project_number = 1
```

| Key | Type | Description |
|-----|------|-------------|
| `owner` | String | GitHub organization or user owning the repository |
| `repo` | String | Repository name |
| `project_number` | Integer | GitHub Projects v2 project number (visible in the project URL) |

**Authentication:** TaskGuard uses `gh auth token` at runtime — the GitHub CLI (`gh`) must be authenticated. No token is stored in `github.toml`.

**Finding your project number:** Navigate to your GitHub project board. The URL is `https://github.com/orgs/{org}/projects/{number}` or `https://github.com/users/{user}/projects/{number}`. The number at the end is `project_number`.

---

## Custom Templates

Area-specific task templates live in `.taskguard/templates/`. Create a file named `{area}.md` to override the built-in template for that area.

```
.taskguard/
└── templates/
    ├── backend.md       # Overrides built-in backend template
    ├── api.md           # Overrides built-in api template
    └── _default.md      # Fallback for any area without a specific template
```

**Template lookup order:**
1. `.taskguard/templates/{area}.md`
2. `.taskguard/templates/_default.md`
3. Built-in template for the area
4. Generic built-in template

**Template variables:**

| Variable | Replaced with |
|----------|--------------|
| `{{title}}` | Task title passed to `taskguard create` |
| `{{date}}` | Current date in ISO 8601 format |

---

## Root Detection

TaskGuard walks up the directory tree from the current working directory looking for a `.taskguard/` folder (the same pattern git uses to find `.git/`). All commands work from any subdirectory of the project.

---

## Local State (Gitignored)

`.taskguard/state/` is automatically added to `.gitignore` on `init`. It holds ephemeral local state (caches, session info) that should not be committed.

`.taskguard/archive/` and `.taskguard/github-mapping.json` **are** committed to Git — they form part of the project's permanent task history.
