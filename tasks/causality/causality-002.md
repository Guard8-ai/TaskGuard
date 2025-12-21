---
id: causality-002
title: "Add --allow-orphan-task flag and CAUTION to create command"
status: todo
priority: high
tags: [causality, v0.4.0, create, cli]
dependencies: [causality-001]
assignee: developer
created: 2025-12-21T12:00:00Z
estimate: 2h
complexity: 5
area: causality
---

# Add --allow-orphan-task flag and CAUTION to create command

> **AI Agent Instructions:**
> 1. Read the existing create.rs implementation first
> 2. Add the new CLI flag to main.rs
> 3. Implement orphan detection logic in create.rs
> 4. Add CAUTION message output
> 5. Write tests for both success and failure cases

## Context

The `taskguard create` command currently allows creating tasks without dependencies. This breaks causality tracking and allows AI agents to drift into disconnected work.

## Requirements

### 1. Add CLI Flag

In `src/main.rs`, add to the `Create` command:

```rust
/// Allow creating task without dependencies (not recommended)
#[arg(long)]
allow_orphan_task: bool,
```

### 2. Implement CAUTION Logic

In `src/commands/create.rs`, after parsing dependencies:

```rust
// Check if orphan (no dependencies provided)
if dependency_list.is_empty() && !allow_orphan_task {
    eprintln!("⚠️  CAUTION: Task has no dependencies.");
    eprintln!("   Orphan tasks break causality tracking and reduce AI agent effectiveness.");
    eprintln!();
    eprintln!("   Options:");
    eprintln!("     --dependencies <ids>    Link to originating task(s)");
    eprintln!("     --allow-orphan-task     Create anyway (not recommended)");
    eprintln!();
    eprintln!("   Example:");
    eprintln!("     taskguard create --title \"{}\" --dependencies \"setup-001\"", title);
    return Err(anyhow::anyhow!("Task creation blocked: no dependencies specified"));
}
```

### 3. Success Message for --allow-orphan-task

When `--allow-orphan-task` is used:

```rust
if allow_orphan_task && dependency_list.is_empty() {
    println!("⚠️  Note: Created orphan task (no dependencies).");
    println!("   Consider adding dependencies later with:");
    println!("   taskguard update dependencies {} \"<task-id>\"", task.id);
}
```

## Exit Codes

| Scenario | Exit Code |
|----------|-----------|
| Created with dependencies | 0 |
| Created with --allow-orphan-task | 0 |
| No dependencies, no flag | 1 |

## Test Cases

### Test 1: Create without dependencies fails
```bash
taskguard create --title "Test" --area testing
# Expected: CAUTION message, exit 1
```

### Test 2: Create with dependencies succeeds
```bash
taskguard create --title "Test" --area testing --dependencies "setup-001"
# Expected: Success, exit 0
```

### Test 3: Create with --allow-orphan-task succeeds
```bash
taskguard create --title "Test" --area testing --allow-orphan-task
# Expected: Success with note, exit 0
```

## Files to Modify

- [ ] `src/main.rs` - Add `allow_orphan_task` flag to Create command
- [ ] `src/commands/create.rs` - Add orphan detection and CAUTION logic
- [ ] `tests/cli_integration_tests.rs` - Add tests for new behavior

## Acceptance Criteria

- [ ] `--allow-orphan-task` flag recognized by CLI
- [ ] CAUTION message uses exact wording (AI agents parse it)
- [ ] Exit code 1 when blocked, 0 when allowed
- [ ] Existing functionality unchanged for tasks with dependencies
- [ ] Tests pass
