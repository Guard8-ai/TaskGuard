---
id: causality-005
title: "Update import-md with orphan detection and CAUTION output"
status: todo
priority: high
tags: [causality, v0.4.0, import-md, soft-enforcement]
dependencies: [causality-004]
assignee: developer
created: 2025-12-21T12:00:00Z
estimate: 2h
complexity: 5
area: causality
---

# Update import-md with orphan detection and CAUTION output

> **AI Agent Instructions:**
> 1. Read the existing import_md.rs implementation
> 2. Add orphan detection AFTER successful import
> 3. Output CAUTION message but DO NOT FAIL
> 4. AI agents will see CAUTION and fix orphans manually

## Context

The `import-md` command bulk imports tasks from markdown files. If imported tasks don't specify dependencies, they become orphans. Unlike `create`, bulk import should **succeed with CAUTION** rather than fail - this allows AI agents to fix orphans after import.

## Key Difference from `create`

| Command | Orphan Behavior | Exit Code |
|---------|-----------------|-----------|
| `create` | CAUTION + **FAIL** | 1 |
| `import-md` | CAUTION + **SUCCEED** | 0 |

Rationale: Bulk imports are valuable even with some orphans. AI agents will fix them.

## Requirements

### 1. Detect Orphans After Import

After all tasks are imported, check which ones are orphans:

```rust
fn detect_orphan_imports(imported_tasks: &[Task], existing_tasks: &[Task]) -> Vec<&Task> {
    // Build reverse dependency map from ALL tasks (existing + imported)
    let all_tasks: Vec<&Task> = existing_tasks.iter()
        .chain(imported_tasks.iter())
        .collect();

    let mut has_dependents: HashSet<String> = HashSet::new();
    for task in &all_tasks {
        for dep in &task.dependencies {
            has_dependents.insert(dep.clone());
        }
    }

    // Filter imported tasks that are orphans
    imported_tasks.iter()
        .filter(|t| {
            t.dependencies.is_empty()
            && !has_dependents.contains(&t.id)
            && t.id != "setup-001"
        })
        .collect()
}
```

### 2. Output Format

Successful import with dependencies:
```
✅ Imported 5 tasks:
   ✅ backend-002 (depends on: backend-001)
   ✅ api-001 (depends on: backend-002)
   ✅ api-002 (depends on: api-001)
   ✅ docs-001 (depends on: api-001)
   ✅ testing-001 (depends on: setup-001)

📊 SUMMARY
   Imported: 5
   With dependencies: 5
   Orphans: 0
```

Import with orphans:
```
✅ Imported 5 tasks:
   ✅ backend-002 (depends on: backend-001)
   ✅ api-001 (depends on: backend-002)
   ✅ api-002 (depends on: api-001)
   ⚠️  docs-001 (no dependencies)
   ⚠️  testing-001 (no dependencies)

⚠️  CAUTION: 2 orphan tasks created (no dependencies, nothing depends on them):
   - docs-001: API Documentation
   - testing-001: Unit Test Setup

   Orphan tasks break causality tracking. Add dependencies with:
     taskguard update dependencies docs-001 "api-001"
     taskguard update dependencies testing-001 "setup-001"

📊 SUMMARY
   Imported: 5
   With dependencies: 3
   Orphans: 2
```

### 3. Handle Declared Dependencies

If the markdown file declares dependencies (using existing pattern):
```markdown
**Dependencies:** [setup-001, backend-001]
```

The import should parse and use them (this already exists in import_md.rs).

### 4. Exit Code

Always return 0 on successful import, even with orphans:
- Exit 0: Import succeeded (with or without orphans)
- Exit 1: Import failed (parse errors, file not found, etc.)

## Test Cases

### Test 1: Import with dependencies
```bash
cat > /tmp/test.md << 'EOF'
# Task List

## Setup Database
**Dependencies:** [setup-001]
Configure PostgreSQL connection.

## Create API Endpoints
**Dependencies:** [backend-001]
Implement REST endpoints.
EOF

taskguard import-md /tmp/test.md --area backend
# Expected: Success, no CAUTION
```

### Test 2: Import without dependencies
```bash
cat > /tmp/test.md << 'EOF'
# Task List

## Write Documentation
Document the API.

## Create Tests
Unit tests for API.
EOF

taskguard import-md /tmp/test.md --area docs
# Expected: Success + CAUTION about 2 orphans
```

### Test 3: Mixed import
```bash
cat > /tmp/test.md << 'EOF'
# Task List

## Backend Work
**Dependencies:** [setup-001]
Backend implementation.

## Random Task
Some orphan work.
EOF

taskguard import-md /tmp/test.md --area backend
# Expected: Success + CAUTION about 1 orphan
```

## Files to Modify

- [ ] `src/commands/import_md.rs` - Add orphan detection and CAUTION output
- [ ] `tests/import_md_tests.rs` - Add orphan detection tests

## Acceptance Criteria

- [ ] Orphan detection runs after successful import
- [ ] CAUTION message shows exact orphan task IDs
- [ ] CAUTION includes actionable fix commands
- [ ] Exit code is 0 even with orphans
- [ ] Tasks with declared dependencies are NOT flagged as orphans
- [ ] setup-001 is never flagged as orphan
- [ ] All existing import-md tests pass
