---
id: tools-001
title: Design md-to-tasks command specification
status: done
priority: high
tags:
- tools
- design
- automation
dependencies: []
assignee: developer
created: 2025-10-30T13:39:44.464322655Z
estimate: 4h
complexity: 6
area: tools
---

# Design md-to-tasks command specification

## Context

TaskGuard analysis documents (like PR2_GITHUB_COMPATIBILITY_ANALYSIS.md and CAUSALITY_CHAIN_INTEGRITY_ISSUES.md) contain structured sections with fixes, issues, and implementation steps that should be converted to actionable tasks.

Current state: 900+ line markdown files with structured content but no automated way to convert them to TaskGuard tasks.

Goal: Design a `taskguard import-md` command that can parse structured markdown and create task files automatically.

## Objectives

- Design CLI interface for `taskguard import-md` command
- Define markdown structure patterns to recognize (headings, lists, code blocks)
- Specify section-to-task mapping rules
- Define dependency extraction patterns
- Design task metadata inference (priority, complexity, area)
- Specify error handling for malformed markdown

## Tasks

- [ ] Define command syntax and options
- [ ] Design markdown parsing strategy (sections → tasks)
- [ ] Specify section type detection rules
- [ ] Design dependency extraction patterns
- [ ] Define metadata inference rules
- [ ] Document expected markdown input format
- [ ] Create example transformations

## Acceptance Criteria

✅ **Clear CLI Interface:**
- Command syntax defined: `taskguard import-md <file> [options]`
- Options specified (--area, --dry-run, --prefix, etc.)
- Help text documented

✅ **Parsing Strategy:**
- Heading levels mapped to task hierarchy
- List items recognized as subtasks or dependencies
- Code blocks preserved in task content
- Tables handled appropriately

✅ **Dependency Extraction:**
- "Fix #N" patterns recognized
- "Depends on" patterns detected
- Task ID references extracted

✅ **Metadata Inference:**
- Priority inferred from keywords (CRITICAL, HIGH, etc.)
- Complexity estimated from content length
- Area determined from file structure or tags
- Status set to "todo" by default

## Technical Notes

### Markdown Structure Patterns to Recognize

```markdown
## Issue #N: Title          → Create task: issue-N
**Priority:** HIGH          → priority: high
**Effort:** 4 hours        → estimate: 4h
Dependencies: [fix-1]      → dependencies: [fix-1]

### Fix #N: Title          → Create task: fix-N
**Location:** src/file.rs  → Add to technical notes

#### Breaking Scenario      → Include in task content
```rust
// code example              → Preserve in content
```
```

### Section Type Detection

1. **Issues** (## Issue #N, ### ❌ Issue #N)
   - Extract: ID, title, priority, dependencies
   - Map to: task with status=todo

2. **Fixes** (## Fix #N, ### Fix #N)
   - Extract: ID, title, effort estimate, location
   - Map to: task with code implementation details

3. **Requirements** (## Required Fixes, ### Testing Requirements)
   - Extract: List items as individual tasks
   - Group under common area

4. **Breaking Scenarios** (## Breaking Scenarios)
   - Extract: Each scenario as separate task or subtask
   - Include examples in task content

### Dependency Patterns

```
"Depends on Fix #1" → dependencies: [fix-1]
"Requires Issue #2" → dependencies: [issue-2]
"Blocked by" → dependencies: [...]
"Prerequisites:" → dependencies: [...]
```

### CLI Options

```bash
taskguard import-md <file> [OPTIONS]

Options:
  --area <AREA>          Default area for imported tasks
  --prefix <PREFIX>      Task ID prefix (default: inferred from file)
  --dry-run             Show what would be created without creating
  --start-number <N>    Starting task number (default: auto-detect)
  --tags <TAGS>         Additional tags (comma-separated)
  --priority <PRIORITY>  Override inferred priority
```

### Example Transformation

**Input (markdown):**
```markdown
## Fix #3: Update Archive to Close GitHub Issues

**Priority:** HIGH
**Effort:** 4 hours
**Location:** src/commands/archive.rs

Archive command should close GitHub issues when archiving completed tasks.

### Implementation
- Check for GitHub integration
- Load mapping file
- Close issues via GraphQL API
```

**Output (task file):**
```yaml
---
id: github-fix-3
title: Update Archive to Close GitHub Issues
status: todo
priority: high
estimate: 4h
area: github
dependencies: []
---

Archive command should close GitHub issues when archiving completed tasks.

## Implementation
- [ ] Check for GitHub integration
- [ ] Load mapping file
- [ ] Close issues via GraphQL API

## Technical Notes
Location: src/commands/archive.rs
```

## Testing

- [ ] Test with PR2_GITHUB_COMPATIBILITY_ANALYSIS.md
- [ ] Test with CAUSALITY_CHAIN_INTEGRITY_ISSUES.md
- [ ] Test with malformed markdown
- [ ] Test dependency extraction
- [ ] Test metadata inference
- [ ] Test dry-run mode

## Version Control

- [ ] Create design document in docs/
- [ ] Review design with stakeholders
- [ ] Document examples and edge cases
- [ ] Create specification for implementation team

## Updates

- 2025-10-30: Task created with full specification