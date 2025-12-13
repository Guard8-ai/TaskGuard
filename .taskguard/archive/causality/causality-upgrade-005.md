---
id: causality-upgrade-005
title: Integration Testing - Verify Complete Causality-Aware System Workflow
status: done
priority: high
tags:
- causality
- upgrade
- testing
dependencies:
- causality-upgrade-001
- causality-upgrade-002
- causality-upgrade-003
- causality-upgrade-004
assignee: developer
created: 2025-10-30T15:00:00Z
estimate: 4h
complexity: 7
area: causality
---

# Integration Testing - Verify Complete Causality-Aware System Workflow

> **⚠️ CRITICAL WORKFLOW NOTICE:**
>
> **This task MUST be completed in ONE dedicated session.**
>
> When this task is marked `done`, the AI agent completing it MUST:
> 1. Fill the "Session Handoff" section below with complete implementation details
> 2. Document what was changed, what runtime behavior to expect, and what dependencies were affected
> 3. Create a clear handoff for the developer explaining the complete causality-aware system
>
> **This is the FINAL task in the causality-upgrade series.** The session handoff should summarize the entire causality system implementation.

## Intent
Create comprehensive integration tests that verify the entire causality-aware system works end-to-end, from task creation through upgrade to validation. This ensures the system achieves its goal of preventing temporal drift in AI agent workflows.

## Pre-Implementation Exploration
**Before coding, AI agent must explore:**
- [ ] **Horizontal scan**: Review existing test infrastructure in tests/ directory
- [ ] **Vertical scan**: Trace test execution flow (Setup → Actions → Assertions)
- [ ] **Git context**: Check testing patterns in recent commits
- [ ] **Complexity check**: Will this fit in one session context?
  - ⚠️ MODERATE - Multiple test scenarios, end-to-end flows
  - Causality: Test setup → Execute commands → Verify state → Validate causality preservation

## Implementation Context
**Files/functions this touches:**
- NEW: `tests/causality_integration_tests.rs` - Integration test suite
- Related: `tests/` directory - Existing test utilities
- Dependencies: All causality-aware commands (create, upgrade, validate)

**Expected changes:**
- Create comprehensive integration test suite
- Test full workflow: create → upgrade → validate
- Verify causality preservation
- Test edge cases and error conditions

## Expected Causality Chain
**What should happen when this works:**

### Test Execution Flow:
1. Test suite runs → Temporary test environment created
2. Each test scenario executes → Commands run → State verified
3. Assertions check causality → Format verified → Content validated
4. Cleanup runs → Test environment removed
5. Results reported → Failures clearly shown

### What Gets Tested:
1. **New task creation** → Template applied → Causality sections present
2. **Legacy task upgrade** → Sections added → Content preserved
3. **Validation detection** → Legacy found → Causality quality assessed
4. **End-to-end workflow** → Create → Work → Upgrade → Validate → All works together

**Failure modes:**
- If test environment corrupt → Cleanup and retry
- If command fails → Capture output, show error
- If assertion fails → Show expected vs actual

## Current State
- [ ] Review existing test infrastructure
- [ ] Create test utilities for causality verification
- [ ] Write test suite for new task creation
- [ ] Write test suite for upgrade command
- [ ] Write test suite for validate enhancements
- [ ] Write end-to-end workflow tests
- [ ] Test edge cases and error conditions
- [ ] Document test coverage

## Test Suite Structure

### Test Categories

1. **Template Tests** (causality-upgrade-001)
   - New task has all causality sections
   - YAML frontmatter valid
   - Markdown structure correct
   - Sections have proper placeholders

2. **Upgrade Tests** (causality-upgrade-002)
   - Single task upgrade works
   - Bulk upgrade works
   - Dry-run mode works
   - Content preservation verified
   - Git commits created

3. **Validation Tests** (causality-upgrade-003)
   - Legacy format detected
   - Causality quality assessed
   - Statistics accurate
   - Recommendations shown

4. **End-to-End Workflow Tests**
   - Complete workflow from creation to completion
   - Mixed format handling
   - Causality chain tracking across commands

## Test Implementation

### Test Utilities

```rust
// Helper functions for testing causality features
mod causality_test_utils {
    pub fn create_test_task(title: &str, area: &str) -> Result<Task> {
        // Create task using create command
    }

    pub fn create_legacy_task(id: &str, content: &str) -> Result<()> {
        // Create task in legacy format for upgrade testing
    }

    pub fn has_causality_section(content: &str, section: &str) -> bool {
        // Check if content has specific causality section
    }

    pub fn verify_content_preserved(original: &str, upgraded: &str) -> bool {
        // Ensure original content exists in upgraded version
    }

    pub fn run_upgrade(args: &[&str]) -> Result<String> {
        // Execute upgrade command and capture output
    }

    pub fn run_validate() -> Result<String> {
        // Execute validate command and capture output
    }
}
```

### Test 1: New Task Has Causality Sections

```rust
#[test]
fn test_new_task_includes_causality_sections() {
    // Setup test environment
    let temp_dir = create_temp_taskguard_project();

    // Create new task
    let result = run_create_command(&temp_dir, "Test Task", "backend");
    assert!(result.is_ok());

    // Load created task
    let task_file = find_task_file(&temp_dir, "backend-001");
    let content = fs::read_to_string(task_file)?;

    // Verify all causality sections present
    assert!(has_causality_section(&content, "## Intent"));
    assert!(has_causality_section(&content, "## Pre-Implementation Exploration"));
    assert!(has_causality_section(&content, "## Implementation Context"));
    assert!(has_causality_section(&content, "## Expected Causality Chain"));
    assert!(has_causality_section(&content, "## Session Handoff"));

    // Verify YAML frontmatter valid
    let task = Task::from_file(task_file)?;
    assert_eq!(task.id, "backend-001");
    assert_eq!(task.title, "Test Task");
}
```

### Test 2: Upgrade Preserves Content

```rust
#[test]
fn test_upgrade_preserves_original_content() {
    let temp_dir = create_temp_taskguard_project();

    // Create legacy task with specific content
    let legacy_content = r#"---
id: backend-001
title: "Legacy Task"
area: backend
---

# Legacy Task

This is important original content.
It must not be lost during upgrade.

## Custom Section
With custom data.
"#;

    create_legacy_task(&temp_dir, "backend-001", legacy_content)?;

    // Run upgrade
    run_upgrade(&temp_dir, &["backend-001"])?;

    // Load upgraded task
    let task_file = find_task_file(&temp_dir, "backend-001");
    let upgraded = fs::read_to_string(task_file)?;

    // Verify causality sections added
    assert!(has_causality_section(&upgraded, "## Intent"));
    assert!(has_causality_section(&upgraded, "## Expected Causality Chain"));

    // Verify original content preserved
    assert!(upgraded.contains("This is important original content"));
    assert!(upgraded.contains("It must not be lost during upgrade"));
    assert!(upgraded.contains("## Custom Section"));
    assert!(upgraded.contains("With custom data"));
}
```

### Test 3: Validate Detects Legacy Format

```rust
#[test]
fn test_validate_detects_legacy_format() {
    let temp_dir = create_temp_taskguard_project();

    // Create mix of legacy and causality-aware tasks
    create_causality_aware_task(&temp_dir, "backend-001")?;
    create_causality_aware_task(&temp_dir, "backend-002")?;
    create_legacy_task(&temp_dir, "backend-003", LEGACY_CONTENT)?;
    create_legacy_task(&temp_dir, "api-001", LEGACY_CONTENT)?;

    // Run validate
    let output = run_validate(&temp_dir)?;

    // Verify detection
    assert!(output.contains("Causality-aware tasks: 2"));
    assert!(output.contains("Legacy format tasks: 2"));
    assert!(output.contains("backend-003"));
    assert!(output.contains("api-001"));
    assert!(output.contains("taskguard upgrade --all"));
}
```

### Test 4: Bulk Upgrade Works

```rust
#[test]
fn test_bulk_upgrade_all_legacy_tasks() {
    let temp_dir = create_temp_taskguard_project();

    // Create multiple legacy tasks
    for i in 1..=5 {
        create_legacy_task(&temp_dir, &format!("backend-{:03}", i), LEGACY_CONTENT)?;
    }

    // Run bulk upgrade
    let output = run_upgrade(&temp_dir, &["--all"])?;

    // Verify all upgraded
    assert!(output.contains("Upgraded 5 tasks"));

    // Verify each task now causality-aware
    for i in 1..=5 {
        let task_file = find_task_file(&temp_dir, &format!("backend-{:03}", i));
        let content = fs::read_to_string(task_file)?;
        assert!(has_causality_section(&content, "## Intent"));
        assert!(has_causality_section(&content, "## Expected Causality Chain"));
    }
}
```

### Test 5: Dry-Run Doesn't Modify Files

```rust
#[test]
fn test_upgrade_dry_run_preserves_files() {
    let temp_dir = create_temp_taskguard_project();
    create_legacy_task(&temp_dir, "backend-001", LEGACY_CONTENT)?;

    // Get original content
    let task_file = find_task_file(&temp_dir, "backend-001");
    let original = fs::read_to_string(&task_file)?;

    // Run dry-run upgrade
    let output = run_upgrade(&temp_dir, &["backend-001", "--dry-run"])?;

    // Verify preview shown
    assert!(output.contains("DRY RUN"));
    assert!(output.contains("No files modified"));

    // Verify file unchanged
    let after = fs::read_to_string(&task_file)?;
    assert_eq!(original, after);
}
```

### Test 6: End-to-End Workflow

```rust
#[test]
fn test_complete_causality_workflow() {
    let temp_dir = create_temp_taskguard_project();

    // 1. Create new task (causality-aware by default)
    run_create_command(&temp_dir, "New Feature", "backend")?;

    // 2. Create legacy task to upgrade
    create_legacy_task(&temp_dir, "api-001", LEGACY_CONTENT)?;

    // 3. Validate should show mixed state
    let output = run_validate(&temp_dir)?;
    assert!(output.contains("Causality-aware tasks: 1"));
    assert!(output.contains("Legacy format tasks: 1"));

    // 4. Upgrade legacy task
    run_upgrade(&temp_dir, &["api-001"])?;

    // 5. Validate should now show all causality-aware
    let output = run_validate(&temp_dir)?;
    assert!(output.contains("Causality-aware tasks: 2"));
    assert!(output.contains("Legacy format tasks: 0"));
    assert!(output.contains("100% causality-aware"));
}
```

### Test 7: Causality Quality Detection

```rust
#[test]
fn test_validate_detects_incomplete_causality() {
    let temp_dir = create_temp_taskguard_project();

    // Create task with empty causality sections
    let incomplete = r#"---
id: backend-001
title: "Incomplete Task"
status: done
area: backend
---

# Incomplete Task

## Intent
[AI: Document intent]

## Expected Causality Chain
[AI: Document causality]

## Session Handoff
[AI: Complete this]
"#;

    create_task_file(&temp_dir, "backend-001", incomplete)?;

    // Run validate
    let output = run_validate(&temp_dir)?;

    // Should detect incomplete sections
    assert!(output.contains("CAUSALITY QUALITY"));
    assert!(output.contains("incomplete causality sections"));
    assert!(output.contains("backend-001"));
    assert!(output.contains("Empty 'Intent' section"));
}
```

## Test Coverage Goals

- ✅ **Template Application**: New tasks have all sections
- ✅ **Content Preservation**: Upgrade doesn't lose data
- ✅ **Format Detection**: Validate identifies legacy tasks
- ✅ **Bulk Operations**: Upgrade --all works correctly
- ✅ **Dry-Run Mode**: Preview without modification
- ✅ **End-to-End Flow**: Complete workflow tested
- ✅ **Quality Detection**: Incomplete sections identified
- ✅ **Error Handling**: Commands fail gracefully
- ✅ **Git Integration**: Commits created correctly

## Edge Cases to Test

1. **Empty task files**
2. **Malformed YAML frontmatter**
3. **Missing required fields**
4. **Very large task files** (>1MB)
5. **Special characters in content**
6. **Concurrent upgrades** (race conditions)
7. **Partially upgraded tasks** (interrupted upgrade)
8. **Git conflicts during commit**

## Implementation Steps

1. **Setup test infrastructure:**
   - Create causality_integration_tests.rs
   - Add test utilities module
   - Setup temp directory helpers

2. **Write core tests:**
   - Template tests
   - Upgrade tests
   - Validation tests

3. **Write workflow tests:**
   - End-to-end scenarios
   - Mixed format handling

4. **Write edge case tests:**
   - Error conditions
   - Boundary cases

5. **Document test coverage:**
   - What's tested
   - What's not tested
   - Coverage percentage

6. **Run and verify:**
   ```bash
   cargo test causality_integration_tests
   ```

## Success Criteria

- ✅ All tests pass
- ✅ Coverage >90% for causality code
- ✅ Edge cases handled gracefully
- ✅ Clear error messages
- ✅ Fast test execution (<30 seconds)
- ✅ No flaky tests

## Task Dependencies
- **Blocks**: None (testing is final validation)
- **Blocked by**:
  - causality-upgrade-001 (template implementation)
  - causality-upgrade-002 (upgrade command)
  - causality-upgrade-003 (validate enhancements)
  - causality-upgrade-004 (documentation as reference)
- **Related**: All causality-aware upgrade tasks

## Complexity Assessment
- **Estimate**: 4 hours
- **Complexity**: 7/10 - Comprehensive test suite, multiple scenarios
- **Risk factors**:
  - Test flakiness (temp dirs, file I/O)
  - Coverage gaps
  - Edge cases missed
  - Performance issues with large test data

## Session Notes
- **Created**: 2025-10-30
- **Next session should**: Review existing test patterns, setup test infrastructure

## Session Handoff
To be filled when task is complete.