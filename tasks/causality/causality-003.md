---
id: causality-003
title: "Add --orphans flag to validate command"
status: todo
priority: high
tags: [causality, v0.4.0, validate, cli]
dependencies: [causality-002]
assignee: developer
created: 2025-12-21T12:00:00Z
estimate: 2h
complexity: 4
area: causality
---

# Add --orphans flag to validate command

> **AI Agent Instructions:**
> 1. Read the existing validate.rs implementation first
> 2. Add the new CLI flag to main.rs
> 3. Implement orphan detection logic (forward + backward check)
> 4. setup-001 is EXEMPT from orphan status (it's the root)
> 5. Write tests for orphan detection

## Context

The `taskguard validate` command checks dependencies and task health. Extend it to detect orphan tasks - tasks with no dependencies AND nothing depends on them.

## Requirements

### 1. Add CLI Flag

In `src/main.rs`, add to the `Validate` command:

```rust
/// Show orphan tasks (no dependencies and nothing depends on them)
#[arg(long)]
orphans: bool,
```

### 2. Implement Orphan Detection

In `src/commands/validate.rs`:

```rust
fn find_orphan_tasks(tasks: &[Task]) -> Vec<&Task> {
    // Build reverse dependency map (who depends on whom)
    let mut has_dependents: HashSet<String> = HashSet::new();
    for task in tasks {
        for dep in &task.dependencies {
            has_dependents.insert(dep.clone());
        }
    }

    // Orphan = no dependencies AND no dependents AND not setup-001
    tasks.iter()
        .filter(|t| {
            t.dependencies.is_empty()
            && !has_dependents.contains(&t.id)
            && t.id != "setup-001"  // Root is exempt
        })
        .collect()
}
```

### 3. Output Format

When `--orphans` is specified:

```
🔍 ORPHAN TASKS
   Tasks with no dependencies and nothing depends on them:

   ⚠️  docs-001 - API Documentation
   ⚠️  testing-001 - Unit Test Setup

   To fix, add dependencies:
     taskguard update dependencies docs-001 "api-001"
     taskguard update dependencies testing-001 "setup-001"

📊 SUMMARY
   Total orphans: 2
   (setup-001 is exempt as root task)
```

If no orphans:
```
🔍 ORPHAN TASKS
   ✅ No orphan tasks found. All tasks are connected to the dependency graph.
```

### 4. Include in Regular Validation

Even without `--orphans` flag, include a summary line:

```
📊 SUMMARY
   Total tasks: 25
   Available: 8
   Blocked: 12
   Orphans: 2 (use --orphans to see details)
```

## Orphan Definition

```
ORPHAN =
  task.dependencies.is_empty()      // No forward links
  AND !has_dependents(task.id)      // No backward links
  AND task.id != "setup-001"        // Root is always valid
```

## Test Cases

### Test 1: Detect orphan task
```bash
# Create orphan
taskguard create --title "Orphan" --area testing --allow-orphan-task

# Should show orphan
taskguard validate --orphans
# Expected: Lists testing-001 as orphan
```

### Test 2: Task with dependencies is not orphan
```bash
taskguard create --title "Linked" --area testing --dependencies "setup-001"
taskguard validate --orphans
# Expected: testing-002 NOT listed
```

### Test 3: Task that is depended upon is not orphan
```bash
# Create A with no deps (orphan)
taskguard create --title "A" --area testing --allow-orphan-task
# Create B depending on A
taskguard create --title "B" --area testing --dependencies "testing-001"
taskguard validate --orphans
# Expected: testing-001 NOT listed (B depends on it)
# Expected: testing-002 NOT listed (has dependencies)
```

### Test 4: setup-001 is never orphan
```bash
taskguard validate --orphans
# Expected: setup-001 NOT listed even if nothing depends on it
```

## Files to Modify

- [ ] `src/main.rs` - Add `orphans` flag to Validate command
- [ ] `src/commands/validate.rs` - Add orphan detection and output
- [ ] `tests/cli_integration_tests.rs` - Add tests for orphan detection

## Acceptance Criteria

- [ ] `--orphans` flag recognized by CLI
- [ ] Orphan detection considers both forward and backward dependencies
- [ ] `setup-001` is always exempt from orphan status
- [ ] Summary shows orphan count even without `--orphans` flag
- [ ] Output format is clear and actionable
- [ ] Tests pass
