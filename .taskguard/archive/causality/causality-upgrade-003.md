---
id: causality-upgrade-003
title: Enhanced Validate Command - Detect Legacy Format Tasks and Causality Issues
status: done
priority: medium
tags:
- causality
- upgrade
- validation
dependencies:
- causality-upgrade-001
- causality-upgrade-002
assignee: developer
created: 2025-10-30T15:00:00Z
estimate: 3h
complexity: 6
area: causality
---

# Enhanced Validate Command - Detect Legacy Format and Causality Issues

> **⚠️ CRITICAL WORKFLOW NOTICE:**
>
> **This task MUST be completed in ONE dedicated session.**
>
> When this task is marked `done`, the AI agent completing it MUST:
> 1. Fill the "Session Handoff" section below with complete implementation details
> 2. Document what was changed, what runtime behavior to expect, and what dependencies were affected
> 3. Create a clear handoff for the developer/next AI agent working on `causality-upgrade-004`
>
> **The next task (`causality-upgrade-004`) will be handled in a NEW session** and depends on this handoff for context.

## Intent
Extend the validate command to detect legacy format tasks and report causality-related issues, helping users identify tasks that need upgrading and AI agents that need more context.

## Pre-Implementation Exploration
**Before coding, AI agent must explore:**
- [ ] **Horizontal scan**: Review existing validate.rs implementation
- [ ] **Vertical scan**: Trace validation flow (Load tasks → Check structure → Report issues)
- [ ] **Git context**: Check recent validate.rs changes (`git log --oneline -20 src/commands/validate.rs`)
- [ ] **Complexity check**: Will this fit in one session context?
  - ✅ YES - Extending existing command with new checks
  - Causality: Load tasks → Detect format → Check causality → Report → Suggest actions

## Implementation Context
**Files/functions this touches:**
- Primary: `src/commands/validate.rs` - Add legacy format detection
- Related: `src/task.rs` - May need helper functions
- Dependencies: Existing validation logic

**Expected changes:**
- Add legacy format detection
- Add causality chain completeness checks
- Enhance reporting with upgrade suggestions
- Show statistics on causality-aware vs legacy tasks

## Expected Causality Chain
**What should happen when this works:**

### Normal Validation with Legacy Tasks:
1. User runs `taskguard validate` → All tasks loaded
2. Legacy format detected → Count and identify
3. Validation checks run → Dependencies, conflicts, format
4. Report generated → Shows legacy vs causality-aware stats
5. Recommendations shown → Suggests upgrade command

### All Causality-Aware:
1. User runs `taskguard validate` → All tasks loaded
2. No legacy tasks found → Only causality checks
3. Causality completeness checked → Identify incomplete sections
4. Report generated → Shows causality quality metrics
5. Recommendations shown → Suggests improvements

**Failure modes:**
- If task parsing fails → Count as validation error
- If causality sections malformed → Warn but don't fail
- If too many warnings → Prioritize critical issues

## Current State
- [ ] Review existing validate command implementation
- [ ] Add legacy format detection function
- [ ] Add causality completeness checks
- [ ] Enhance reporting format
- [ ] Add upgrade suggestions
- [ ] Test with mixed format task set

## Enhanced Validation Output

### Example: Mixed Format Tasks
```bash
$ taskguard validate

🚦 TASK STATUS
   ✅ Available tasks (dependencies satisfied):
      ⭕ auth-001 - Implement JWT Auth (causality-aware)
      ⭕ setup-001 - Project Setup (causality-aware)

   🚫 Blocked tasks:
      ❌ api-001 - User API (waiting for: auth-001)

📋 FORMAT ANALYSIS
   ✅ Causality-aware tasks: 12
   ⚠️  Legacy format tasks: 13

   Legacy tasks (missing causality sections):
      📄 backend-001 - Database setup
      📄 backend-002 - API endpoints
      📄 frontend-001 - Login UI
      ... (10 more)

   💡 Run: taskguard upgrade --all

🔍 CAUSALITY QUALITY
   ⚠️  3 tasks have incomplete causality sections:
      backend-003 - Empty "Intent" section
      api-002 - No "Expected Causality Chain" documented
      testing-001 - Missing "Session Handoff" for completed task

✅ DEPENDENCY VALIDATION
   No issues found in 25 tasks

📊 SUMMARY
   Total tasks: 25
   Available: 2
   Blocked: 1
   Causality-aware: 12 (48%)
   Legacy format: 13 (52%)
   Causality issues: 3

💡 RECOMMENDATIONS
   - Upgrade legacy tasks: taskguard upgrade --all
   - 3 tasks need causality documentation improvement
   - System is 48% causality-aware
```

### Example: All Causality-Aware
```bash
$ taskguard validate

🚦 TASK STATUS
   ✅ Available tasks: 15
   🚫 Blocked tasks: 10

📋 FORMAT ANALYSIS
   ✅ All 25 tasks are causality-aware
   🎉 System is 100% causality-aware

🔍 CAUSALITY QUALITY
   ✅ All tasks have complete causality sections
   ✅ All completed tasks have session handoffs
   ✅ Causality chains properly documented

✅ VALIDATION PASSED
   No issues found in 25 tasks

📊 SUMMARY
   Total tasks: 25
   Available: 15
   Blocked: 10
   Causality-aware: 25 (100%)
   System health: EXCELLENT
```

## Detection Logic

### Legacy Format Detection
```rust
fn is_legacy_format(content: &str) -> bool {
    let required_sections = [
        "## Intent",
        "## Pre-Implementation Exploration",
        "## Implementation Context",
        "## Expected Causality Chain",
        "## Session Handoff",
    ];

    // Task is legacy if missing ANY required section
    required_sections.iter().any(|section| !content.contains(section))
}
```

### Causality Completeness Check
```rust
fn check_causality_completeness(task: &Task, content: &str) -> Vec<String> {
    let mut issues = Vec::new();

    // Check if sections are present but empty
    if has_empty_section(content, "## Intent") {
        issues.push("Empty 'Intent' section".to_string());
    }

    if has_empty_section(content, "## Expected Causality Chain") {
        issues.push("No causality chain documented".to_string());
    }

    // Completed tasks should have session handoff
    if task.status == "done" && has_empty_section(content, "## Session Handoff") {
        issues.push("Completed task missing session handoff".to_string());
    }

    issues
}

fn has_empty_section(content: &str, section_header: &str) -> bool {
    if let Some(pos) = content.find(section_header) {
        // Extract section content until next ## header or end
        let after_header = &content[pos + section_header.len()..];
        let section_content = after_header
            .split("\n##")
            .next()
            .unwrap_or("")
            .trim();

        // Check if only contains placeholder text
        section_content.is_empty()
            || section_content.starts_with("[AI:")
            || section_content.starts_with("To be filled")
    } else {
        true // Section doesn't exist = empty
    }
}
```

## Enhanced Reporting

### New Report Sections

1. **FORMAT ANALYSIS**
   - Count causality-aware vs legacy tasks
   - List legacy tasks (with limit, show "X more")
   - Calculate percentage
   - Suggest upgrade command

2. **CAUSALITY QUALITY**
   - Check completeness of causality sections
   - Identify tasks with empty/placeholder sections
   - Check completed tasks for session handoffs
   - Suggest improvements

3. **RECOMMENDATIONS**
   - Prioritize actions based on impact
   - Show specific commands to run
   - Calculate system health percentage

## Implementation Steps

1. **Add format detection:**
   ```rust
   pub fn run() -> Result<()> {
       let tasks = load_all_tasks()?;

       // Categorize by format
       let mut causality_aware = Vec::new();
       let mut legacy = Vec::new();

       for task in &tasks {
           let content = fs::read_to_string(&task.file_path)?;
           if is_legacy_format(&content) {
               legacy.push(task);
           } else {
               causality_aware.push(task);
           }
       }

       // ... existing validation logic ...
   }
   ```

2. **Add quality checks:**
   ```rust
   // For causality-aware tasks, check completeness
   let mut incomplete = Vec::new();
   for task in &causality_aware {
       let content = fs::read_to_string(&task.file_path)?;
       let issues = check_causality_completeness(task, &content);
       if !issues.is_empty() {
           incomplete.push((task, issues));
       }
   }
   ```

3. **Enhance reporting:**
   - Add FORMAT ANALYSIS section
   - Add CAUSALITY QUALITY section
   - Add RECOMMENDATIONS section
   - Show percentage metrics

4. **Add color coding:**
   - Green: Causality-aware and complete
   - Yellow: Causality-aware but incomplete
   - Gray: Legacy format

5. **Test with various task sets:**
   - All legacy
   - All causality-aware
   - Mixed formats
   - Incomplete causality sections

## Statistics to Track

- Total tasks
- Causality-aware count & percentage
- Legacy count & percentage
- Incomplete causality sections count
- Completed tasks with handoffs count
- System health score (0-100%)

**System Health Calculation:**
```rust
let health_score = (
    (causality_aware_count * 100) +
    (complete_causality_count * 50) +
    (handoff_count * 25)
) / (total_tasks * 175) * 100;
```

## Task Dependencies
- **Blocks**: None (validation is final step)
- **Blocked by**:
  - causality-upgrade-001 (needs format definition)
  - causality-upgrade-002 (needs upgrade command to suggest)
- **Related**: All causality-aware upgrade tasks

## Complexity Assessment
- **Estimate**: 3 hours
- **Complexity**: 6/10 - Extending existing command with new checks
- **Risk factors**:
  - Format detection false positives
  - Performance with many tasks
  - Report readability
  - Need clear, actionable recommendations

## Session Notes
- **Created**: 2025-10-30
- **Next session should**: Review validate.rs structure, understand existing checks

## Session Handoff
To be filled when task is complete.