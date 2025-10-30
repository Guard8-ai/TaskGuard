---
id: testing-004
title: Fix CLI integration test failures
status: todo
priority: high
tags:
- testing
- cli
- integration
dependencies: []
assignee: developer
created: 2025-10-30T14:43:36.397006446Z
estimate: 6h
complexity: 6
area: testing
---

# Fix CLI integration test failures

## Context
Currently 31 out of 32 CLI integration tests are failing in `tests/cli_integration_tests.rs`. All failures show the same error: "No such file or directory (os error 2)", suggesting that the tests cannot find the taskguard binary or are looking in the wrong location.

**Test File:** [tests/cli_integration_tests.rs](tests/cli_integration_tests.rs)

## Current Status

### Passing Tests (1/32) ✅
- test_create_with_all_options (only one passing)

### Failing Tests (31/32) ❌
All failures show: `Error: No such file or directory (os error 2)`

**Basic Commands:**
1. test_help_command
2. test_version_command
3. test_invalid_command
4. test_missing_required_args

**Init Command:**
5. test_init_command
6. test_init_already_initialized
7. test_create_without_init

**Create Command:**
8. test_create_basic_task
9. test_create_minimum_required_args

**List Command:**
10. test_list_empty_project
11. test_list_with_tasks
12. test_list_filter_by_area
13. test_list_filter_by_status
14. test_invalid_area_filter

**Validate Command:**
15. test_validate_empty_project
16. test_validate_with_dependencies
17. test_validate_circular_dependencies

**Lint Command:**
18. test_lint_empty_project
19. test_lint_with_tasks
20. test_lint_area_filter
21. test_lint_verbose_mode

**Sync Command:**
22. test_sync_without_git
23. test_sync_with_git_no_commits
24. test_sync_with_task_commits
25. test_sync_verbose_mode

**AI Command:**
26. test_ai_basic_interaction
27. test_ai_empty_input
28. test_ai_status_inquiry
29. test_ai_task_creation_guidance

**Performance:**
30. test_large_project_performance
31. test_command_timeout_handling

## Root Cause Analysis

The consistent "No such file or directory" error suggests:

1. **Binary Path Issue:** Tests may be looking for `taskguard` binary in wrong location
2. **Build Issue:** Binary may not be built before running tests
3. **Test Setup Issue:** Test framework may not be setting up environment correctly
4. **Path Resolution:** Tests may use hardcoded paths instead of dynamic binary location

## Objectives
- Fix all 31 failing CLI integration tests
- Ensure tests can find and execute taskguard binary
- Maintain test isolation and reliability
- Add proper error handling and debugging output

## Tasks

### Phase 1: Investigation
- [ ] Review cli_integration_tests.rs to understand test setup
- [ ] Check how tests locate the taskguard binary
- [ ] Verify cargo test builds binary before running tests
- [ ] Compare passing test (test_create_with_all_options) with failing tests

### Phase 2: Binary Path Resolution
- [ ] Implement proper binary path resolution using env!("CARGO_BIN_EXE_taskguard")
- [ ] Add fallback to compiled binary location
- [ ] Ensure binary is built before tests run
- [ ] Add debug output for binary path

### Phase 3: Test Infrastructure
- [ ] Fix test helper functions if needed
- [ ] Ensure proper test isolation (temp directories)
- [ ] Add better error messages for debugging
- [ ] Implement retry logic for flaky tests if needed

### Phase 4: Fix Test Groups
- [ ] Fix basic command tests (help, version, invalid)
- [ ] Fix init command tests
- [ ] Fix create command tests
- [ ] Fix list command tests
- [ ] Fix validate command tests
- [ ] Fix lint command tests
- [ ] Fix sync command tests
- [ ] Fix AI command tests
- [ ] Fix performance tests

### Phase 5: Validation
- [ ] Run all 32 CLI integration tests
- [ ] Ensure 32/32 tests pass
- [ ] Run full test suite to verify no regressions
- [ ] Add CI/CD test pipeline if not present

## Acceptance Criteria

✅ **All Tests Passing:**
- 32/32 CLI integration tests pass
- No "file not found" errors
- Tests run reliably in CI/CD environment

✅ **Binary Resolution:**
- Tests correctly locate taskguard binary
- Works in both cargo test and IDE test runner
- Proper error messages when binary not found

✅ **Test Quality:**
- Tests are isolated and don't interfere with each other
- Clear error messages for debugging
- Fast execution (under 10 seconds for full suite)

## Technical Notes

### Likely Fix
The tests probably need to use:
```rust
use assert_cmd::Command;

let mut cmd = Command::cargo_bin("taskguard")?;
cmd.arg("help");
cmd.assert().success();
```

Or use the `env!("CARGO_BIN_EXE_taskguard")` macro to get binary path.

### Key Files to Check
- [tests/cli_integration_tests.rs](tests/cli_integration_tests.rs) - Main test file
- Cargo.toml - Ensure [[bin]] section is correct
- Test helper functions - Check how binary is invoked

### Common Test Patterns
```bash
# Run CLI tests only
cargo test --test cli_integration_tests -- --nocapture

# Run specific test
cargo test --test cli_integration_tests test_help_command -- --nocapture

# Build binary first then test
cargo build && cargo test --test cli_integration_tests
```

### Expected Behavior After Fix
- Tests should locate binary at: `target/debug/taskguard` or via cargo_bin
- All commands should execute successfully
- Tests should be isolated with temp directories
- Clear output showing what went wrong if tests fail

## Testing Checklist

**Basic Commands:**
- [ ] test_help_command
- [ ] test_version_command
- [ ] test_invalid_command
- [ ] test_missing_required_args

**Init & Create:**
- [ ] test_init_command
- [ ] test_init_already_initialized
- [ ] test_create_without_init
- [ ] test_create_basic_task
- [ ] test_create_minimum_required_args
- [ ] test_create_with_all_options (already passing)

**List:**
- [ ] test_list_empty_project
- [ ] test_list_with_tasks
- [ ] test_list_filter_by_area
- [ ] test_list_filter_by_status
- [ ] test_invalid_area_filter

**Validate:**
- [ ] test_validate_empty_project
- [ ] test_validate_with_dependencies
- [ ] test_validate_circular_dependencies

**Lint:**
- [ ] test_lint_empty_project
- [ ] test_lint_with_tasks
- [ ] test_lint_area_filter
- [ ] test_lint_verbose_mode

**Sync:**
- [ ] test_sync_without_git
- [ ] test_sync_with_git_no_commits
- [ ] test_sync_with_task_commits
- [ ] test_sync_verbose_mode

**AI:**
- [ ] test_ai_basic_interaction
- [ ] test_ai_empty_input
- [ ] test_ai_status_inquiry
- [ ] test_ai_task_creation_guidance

**Performance:**
- [ ] test_large_project_performance
- [ ] test_command_timeout_handling

## Version Control
- [ ] Create feature branch: `fix/cli-integration-tests`
- [ ] Commit fixes for binary resolution
- [ ] Commit fixes for each test group
- [ ] Merge to master after all tests pass

## Updates
- 2025-10-30: Task created - 31/32 CLI tests failing with "No such file or directory"
