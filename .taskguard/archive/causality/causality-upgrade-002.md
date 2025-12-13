---
id: causality-upgrade-002
title: Implement taskguard upgrade Command - Migrate Legacy Tasks to Causality-Aware Format
status: done
priority: high
tags:
- causality
- upgrade
- command
dependencies:
- causality-upgrade-001
assignee: developer
created: 2025-10-30T15:00:00Z
estimate: 6h
complexity: 8
area: causality
---

# Implement taskguard upgrade Command

> **⚠️ CRITICAL WORKFLOW NOTICE:**
>
> **This task MUST be completed in ONE dedicated session.**
>
> When this task is marked `done`, the AI agent completing it MUST:
> 1. Fill the "Session Handoff" section below with complete implementation details
> 2. Document what was changed, what runtime behavior to expect, and what dependencies were affected
> 3. Create a clear handoff for the developer/next AI agent working on `causality-upgrade-003`
>
> **The next task (`causality-upgrade-003`) will be handled in a NEW session** and depends on this handoff for context.

## Intent
Provide a command to migrate existing legacy tasks to the new causality-aware format, enabling users to upgrade their task database without manual editing. This preserves historical work while adding causality preservation capabilities.

## Pre-Implementation Exploration
**Before coding, AI agent must explore:**
- [ ] **Horizontal scan**: Review similar migration patterns in codebase
- [ ] **Vertical scan**: Trace command flow (CLI → Parser → File Read → Transform → File Write)
- [ ] **Git context**: Check how other commands handle file modifications
- [ ] **Complexity check**: Will this fit in one session context?
  - ⚠️ MODERATE - Multiple operations: read, transform, validate, write, git commit
  - Causality: User command → Load task → Detect legacy format → Add sections → Preserve content → Write → Commit

## Implementation Context
**Files/functions this touches:**
- NEW: `src/commands/upgrade.rs` - Main upgrade command implementation
- Modify: `src/commands/mod.rs` - Add upgrade export
- Modify: `src/main.rs` - Add upgrade CLI command
- Related: `src/task.rs` - Task parsing and serialization
- Git integration: May need git commit functionality

**Expected changes:**
- Create new upgrade command module
- Implement legacy format detection
- Add section insertion logic
- Preserve existing YAML and content
- Add dry-run support
- Git commit after upgrade

## Expected Causality Chain
**What should happen when this works:**

### Single Task Upgrade:
1. User runs `taskguard upgrade task-001` → Command parses task file
2. Legacy format detected → Missing sections identified
3. New sections inserted → Existing content preserved
4. File validated → Parsing succeeds
5. Git commit created → Changes tracked

### Bulk Upgrade:
1. User runs `taskguard upgrade --all` → All tasks scanned
2. Legacy tasks identified → Listed for user
3. Each task upgraded → Progress shown
4. All validated → Report generated
5. Git commit created → Batch changes tracked

### Dry-Run Mode:
1. User runs `taskguard upgrade --dry-run` → Tasks analyzed
2. Changes previewed → No files modified
3. Report shown → User can review impact

**Failure modes:**
- If YAML corrupted → Skip task, show error, continue
- If file write fails → Rollback, show error, abort
- If git commit fails → Files upgraded but not committed (recoverable)
- If validation fails after upgrade → Don't write, show error

## Current State
- [ ] Create upgrade.rs file with command structure
- [ ] Implement legacy format detection
- [ ] Add section insertion logic that preserves content
- [ ] Add dry-run mode
- [ ] Add git commit functionality
- [ ] Implement --all flag for bulk upgrade
- [ ] Add progress reporting
- [ ] Write upgrade tests

## Command Specification

### Command Syntax
```bash
taskguard upgrade [task-id] [OPTIONS]
```

### Options
- `[task-id]` - Upgrade specific task (optional if using --all)
- `--all` - Upgrade all legacy format tasks
- `--dry-run` - Preview changes without modifying files

### Examples
```bash
# Upgrade single task
$ taskguard upgrade backend-001

# Preview changes
$ taskguard upgrade backend-001 --dry-run

# Upgrade all legacy tasks
$ taskguard upgrade --all

# Preview all upgrades
$ taskguard upgrade --all --dry-run
```

## Legacy Format Detection

A task is "legacy format" if it's missing ANY of these sections:
- `## Intent`
- `## Pre-Implementation Exploration`
- `## Implementation Context`
- `## Expected Causality Chain`
- `## Session Handoff`

**Detection logic:**
```rust
fn is_legacy_format(content: &str) -> bool {
    let required_sections = [
        "## Intent",
        "## Pre-Implementation Exploration",
        "## Implementation Context",
        "## Expected Causality Chain",
        "## Session Handoff",
    ];

    required_sections.iter().any(|section| !content.contains(section))
}
```

## Section Insertion Strategy

**Preserve everything, add what's missing:**

1. **Parse existing content:**
   - Extract YAML frontmatter (keep intact)
   - Extract title line (keep intact)
   - Extract existing markdown body (keep intact)

2. **Identify missing sections:**
   - Check each required section
   - Build list of sections to insert

3. **Insert sections intelligently:**
   - Add after title, before existing content
   - Use placeholder text for AI to fill
   - Preserve all existing text

4. **Validate result:**
   - Ensure YAML still parses
   - Ensure task structure valid
   - Run through validate command

## Example Upgrade Transformation

**Before (Legacy):**
```markdown
---
id: backend-001
title: "Implement JWT Auth"
status: todo
priority: high
area: backend
---

# Implement JWT Auth

Need to add JWT authentication to the API.

## Tasks
- [ ] Install JWT library
- [ ] Create middleware
- [ ] Add tests
```

**After (Causality-Aware):**
```markdown
---
id: backend-001
title: "Implement JWT Auth"
status: todo
priority: high
area: backend
---

# Implement JWT Auth

## Intent
[AI: Document what architectural/feature goal this serves and why it matters]

## Pre-Implementation Exploration
**Before coding, AI agent must explore:**
- [ ] **Horizontal scan**: Review similar patterns, related features, parallel implementations
- [ ] **Vertical scan**: Trace dependency chain (UI → API → Service → DB)
- [ ] **Git context**: Check recent commits affecting related code
- [ ] **Complexity check**: Will this fit in one session context?

## Implementation Context
**Files/functions this touches:**
[AI: Document files and entry points]

## Expected Causality Chain
**What should happen when this works:**
[AI: Document the expected event sequence and failure modes]

---

**ORIGINAL CONTENT BELOW - PRESERVED FROM LEGACY TASK:**

Need to add JWT authentication to the API.

## Tasks
- [ ] Install JWT library
- [ ] Create middleware
- [ ] Add tests

---

## Session Handoff
[AI: Complete this when marking task done]
```

## AI Agent Collaboration

**The upgrade command ENCOURAGES AI agents to fill in details:**
- Placeholder text uses `[AI: ...]` prompts
- AI agents may add their own insights during upgrade
- This is ENCOURAGED and valuable
- Original content always preserved in "ORIGINAL CONTENT" section

## Progress Reporting

### Single Task Upgrade:
```bash
$ taskguard upgrade backend-001

📦 UPGRADING TASK TO CAUSALITY-AWARE FORMAT
   Task: backend-001 - Implement JWT Auth

   Missing sections:
   ✅ Adding Intent section
   ✅ Adding Pre-Implementation Exploration
   ✅ Adding Implementation Context
   ✅ Adding Expected Causality Chain
   ✅ Adding Session Handoff section

   ✅ Preserved original content
   ✅ Validated task structure
   ✅ Git commit created

✅ Task upgraded successfully

💡 AI agents can now fill in the causality sections during work
```

### Bulk Upgrade:
```bash
$ taskguard upgrade --all

📦 BULK UPGRADE TO CAUSALITY-AWARE FORMAT
   Found 13 legacy format tasks

Upgrading tasks:
   ✅ backend-001 - Implement JWT Auth
   ✅ frontend-001 - Login UI
   ✅ api-001 - User endpoints
   ... (10 more)

✅ Upgraded 13 tasks successfully
   Git commit: "chore: Upgrade 13 tasks to causality-aware format"

💡 Review upgraded tasks and fill in causality details
```

## Implementation Steps

1. **Create upgrade.rs:**
   - Command structure with clap
   - Legacy format detection
   - Section insertion logic

2. **Add content preservation:**
   - Parse YAML frontmatter
   - Extract title and body
   - Insert new sections
   - Append original content

3. **Add validation:**
   - Ensure YAML parses
   - Ensure task loads correctly
   - Run validate command on result

4. **Add git integration:**
   - Auto-commit after upgrade
   - Descriptive commit messages
   - Handle git errors gracefully

5. **Add CLI integration:**
   - Export from mod.rs
   - Add to main.rs commands
   - Wire up arguments

6. **Test thoroughly:**
   - Test single task upgrade
   - Test bulk upgrade
   - Test dry-run mode
   - Test with various task formats

## Task Dependencies
- **Blocks**: causality-upgrade-003 (enhanced validate needs upgrade capability)
- **Blocked by**: causality-upgrade-001 (needs new template format defined)
- **Related**: All causality-aware upgrade tasks

## Complexity Assessment
- **Estimate**: 6 hours
- **Complexity**: 8/10 - File manipulation, content preservation, git integration
- **Risk factors**:
  - Content corruption if insertion logic wrong
  - YAML frontmatter parsing issues
  - Git commit failures
  - Need comprehensive testing

## Session Notes
- **Created**: 2025-10-30
- **Next session should**: Review existing command structure, understand task parsing

## Session Handoff
To be filled when task is complete.