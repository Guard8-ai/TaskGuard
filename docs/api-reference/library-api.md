# Library API

TaskGuard is distributed as a standalone CLI binary. It does not publish a Rust library crate for external consumption.

---

## Using TaskGuard Programmatically

The recommended integration approach is to invoke `taskguard` as a subprocess from your tool or script.

### Shell / Bash

```bash
# Create a task and capture the output
taskguard create \
  --title "Add rate limiting" \
  --area backend \
  --priority high \
  --dependencies "setup-001" \
  --estimate "3h"

# Check exit code: 0 = success, non-zero = error
if ! taskguard validate; then
  echo "Validation failed"
  exit 1
fi
```

### Python

```python
import subprocess
import json

result = subprocess.run(
    ["taskguard", "list", "--area", "backend"],
    capture_output=True, text=True, check=True
)
print(result.stdout)
```

### Node.js / TypeScript

```typescript
import { execSync } from "child_process";

const output = execSync(
  'taskguard list --area api',
  { encoding: "utf8" }
);
console.log(output);
```

---

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | General error (invalid arguments, missing file, etc.) |
| `2` | Validation failure (circular dependency, orphan, etc.) |

---

## Task File Format

Since tasks are plain Markdown files with YAML front-matter, you can also read and write them directly. See [Task File Format](task-file-format.md) for the complete schema.

---

## AI Agent Integration

For AI agents (Claude Code, etc.), the primary integration surface is the `AGENTIC_AI_TASKGUARD_GUIDE.md` file copied to your project root on `taskguard init`. See [AI Integration](../features/ai-integration.md) for the full guide.
