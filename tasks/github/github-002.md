---
id: github-002
title: Improve GitHub issue descriptions to use Context section
status: done
priority: medium
tags:
- github
- sync
- enhancement
dependencies: []
assignee: developer
created: 2025-11-03T19:49:05.561355766Z
estimate: 1h
complexity: 3
area: github
---

# Improve GitHub issue descriptions to use Context section

## Context

Currently, GitHub issue descriptions are created from the first paragraph of markdown content (src/commands/sync.rs:526-539), which often captures template boilerplate or generic text instead of the meaningful Context section.

**Current behavior:**
```rust
// Lines 526-531
let description = task.content
    .lines()
    .skip_while(|line| line.starts_with('#') || line.trim().is_empty())
    .take_while(|line| !line.trim().is_empty())  // ← Takes first paragraph
    .collect::<Vec<_>>()
    .join("\n");
```

**Problem:**
Most TaskGuard tasks have this structure:
```markdown
# Task Title

> ⚠️ SESSION WORKFLOW NOTICE...  ← First paragraph is template boilerplate

## Context
Actual meaningful description here...  ← This is what we want!
```

The sync grabs the template notice instead of the Context section.

## Objectives

- Extract the `## Context` section content for GitHub issue descriptions
- Fall back to current behavior if no Context section exists
- Maintain 200-character limit for descriptions
- Preserve existing issue body format (TaskGuard ID + description)

## Tasks

- [ ] Update description extraction logic in `src/commands/sync.rs:526-539`
- [ ] Add function to extract Context section from markdown
- [ ] Add fallback to current behavior if no Context found
- [ ] Test with existing task files (especially those with template notices)
- [ ] Verify issue descriptions are more meaningful

## Implementation

**Location:** `src/commands/sync.rs` lines 524-545

**New approach:**
```rust
// Helper function to extract Context section
fn extract_context_section(content: &str) -> Option<String> {
    let lines: Vec<&str> = content.lines().collect();

    // Find "## Context" header
    let context_start = lines.iter()
        .position(|line| line.trim() == "## Context")?;

    // Collect lines until next ## header or end
    let context_content: Vec<&str> = lines[context_start + 1..]
        .iter()
        .take_while(|line| !line.starts_with("## "))
        .skip_while(|line| line.trim().is_empty())
        .copied()
        .collect();

    if context_content.is_empty() {
        None
    } else {
        Some(context_content.join("\n").trim().to_string())
    }
}

// In push_tasks_to_github():
let description = if let Some(context) = extract_context_section(&task.content) {
    // Use Context section
    if context.len() > 200 {
        &context[..200]
    } else {
        &context
    }
} else {
    // Fallback to current behavior
    let desc = task.content
        .lines()
        .skip_while(|line| line.starts_with('#') || line.trim().is_empty())
        .take_while(|line| !line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    if desc.is_empty() {
        "No description"
    } else if desc.len() > 200 {
        &desc[..200]
    } else {
        &desc
    }
};
```

## Acceptance Criteria

✅ **Context section extracted correctly:**
- Issues created from tasks with Context section use that content
- Template boilerplate is skipped

✅ **Fallback works:**
- Tasks without Context section fall back to current behavior
- No panics or errors on edge cases

✅ **Format preserved:**
- Issue body still has "TaskGuard ID: xxx" at top
- 200-character limit respected
- "Synced from TaskGuard" footer present

## Technical Notes

**Edge cases to handle:**
- Tasks without Context section (use fallback)
- Empty Context sections (use fallback)
- Very short Context (<50 chars) - still use it
- Multi-paragraph Context (join with newlines, then truncate)

**Testing tasks:**
- github-fix-2 (has Context section)
- tools-003 (has generic template)
- backend-016 (short task)

## Updates
- 2025-11-03: Task created
- Analysis shows sync.rs lines 526-539 need updating