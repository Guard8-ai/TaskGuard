---
id: tools-006
title: Document and finalize import-md command implementation
status: done
priority: critical
tags:
- tools
- import
- documentation
dependencies: [testing-002]
assignee: developer
created: 2025-10-30T13:40:32.559556679Z
estimate: 2h
complexity: 3
area: tools
---

# Document and finalize import-md command implementation

## Context
The `import-md` command has been successfully implemented to convert markdown analysis files (like PR2_GITHUB_COMPATIBILITY_ANALYSIS.md) into TaskGuard tasks. The implementation includes full functionality with 10/10 tests passing.

## Implementation Summary

### Command Syntax
```bash
taskguard import-md <file.md> --area <area> --prefix <prefix> [--dry-run]
```

### Parameters
- `<file.md>` - Path to markdown file to import
- `--area` - Task area (e.g., github, causality, tools)
- `--prefix` - Task ID prefix (e.g., gh, caus, tools)
- `--dry-run` - Preview import without creating files

### Supported Markdown Patterns

**1. Numbered Lists with Severity**
```markdown
1. **[HIGH]** Task title
   - Description or context
   - Additional details
```

**2. Checklist Items**
```markdown
- [ ] Task to be done
- [x] Already completed task
```

**3. Action Items**
```markdown
### Action Items
- **Action:** Do something important
  - Supporting details
```

**4. Nested Task Lists**
```markdown
1. Parent task
   - Child detail
   - Another detail
   a. Subtask A
   b. Subtask B
```

### Priority Mapping
- `[HIGH]`, `[CRITICAL]` → high
- `[MEDIUM]`, `[MED]` → medium
- `[LOW]` → low
- Default → medium

### Features Implemented
- ✅ Multiple section types (## Tasks, ## Action Items, ## Breaking Changes, etc.)
- ✅ Priority extraction from [HIGH], [CRITICAL], etc. markers
- ✅ Automatic task ID generation (area-001, area-002, etc.)
- ✅ Dry-run mode for preview
- ✅ Full YAML front-matter with metadata
- ✅ Preserves task descriptions and context
- ✅ Handles nested lists and sub-items
- ✅ Smart duplicate detection
- ✅ Comprehensive error handling

## Example Usage

### Basic Import
```bash
# Import GitHub analysis with dry-run preview
cargo run -- import-md PR2_GITHUB_COMPATIBILITY_ANALYSIS.md \
  --area github --prefix gh --dry-run

# Actually create the tasks
cargo run -- import-md PR2_GITHUB_COMPATIBILITY_ANALYSIS.md \
  --area github --prefix gh
```

### Import Results
```
🔍 DRY RUN MODE - No files will be created
📝 Processing: PR2_GITHUB_COMPATIBILITY_ANALYSIS.md

✅ Would create 8 tasks in area 'github':
   📄 gh-001: [HIGH] Implement GitHub Projects V2 OAuth scope
   📄 gh-002: [HIGH] Add GraphQL API client for Projects V2
   📄 gh-003: [MEDIUM] Create field mapping system
   ...

💡 To create these tasks, run without --dry-run
```

### Real-World Test Results
```bash
# Tested with PR2_GITHUB_COMPATIBILITY_ANALYSIS.md
✅ Created 8 tasks in tasks/github/

# Tested with CAUSALITY_CHAIN_INTEGRITY_ISSUES.md
✅ Created 6 tasks in tasks/causality/

# All 10 comprehensive tests passing
✅ test_extract_tasks_from_section
✅ test_numbered_list_extraction
✅ test_action_items_extraction
✅ test_priority_extraction
✅ test_multiple_sections
✅ test_nested_lists
✅ test_duplicate_task_detection
✅ test_generate_task_id
✅ test_extract_priority_from_title
✅ test_create_task_file_content
```

## Technical Implementation

### File Structure
- [src/commands/import_md.rs](src/commands/import_md.rs:1-509) - Main implementation (509 lines)
- [tests/import_md_tests.rs](tests/import_md_tests.rs:1-339) - Test suite (339 lines)

### Key Functions
- `execute()` - Main command handler with dry-run support
- `extract_tasks_from_section()` - Regex-based task extraction
- `extract_priority_from_title()` - Priority marker parsing
- `generate_task_id()` - Auto-incrementing ID generation
- `create_task_file_content()` - YAML + Markdown generation

### Regex Patterns Used
```rust
// Numbered lists: "1. **[HIGH]** Task title"
Regex::new(r"^\s*\d+\.\s+\*\*\[([^\]]+)\]\*\*\s+(.+)$")

// Priority markers: "[HIGH]", "[CRITICAL]"
Regex::new(r"\[(?i)(HIGH|CRITICAL|MEDIUM|MED|LOW)\]")

// Checklist: "- [ ] Task"
Regex::new(r"^\s*-\s+\[[ x]\]\s+(.+)$")

// Action items: "- **Action:** Description"
Regex::new(r"^\s*-\s+\*\*Action:\*\*\s+(.+)$")
```

## Tasks
- [x] Implement basic import-md command structure
- [x] Add markdown parsing for numbered lists
- [x] Add checklist item parsing
- [x] Add action item parsing
- [x] Implement priority extraction
- [x] Add dry-run mode
- [x] Create comprehensive test suite
- [x] Test with PR2_GITHUB_COMPATIBILITY_ANALYSIS.md
- [x] Test with CAUSALITY_CHAIN_INTEGRITY_ISSUES.md
- [x] Document command usage and features
- [ ] Consider adding "## Breaking Scenarios" section support
- [ ] Consider adding "## Requirements" section support
- [ ] Consider adding --start-number flag support
- [ ] Clean up test task files (tasks/github/, tasks/causality/)

## Acceptance Criteria
✅ **Functionality:**
- Command successfully imports markdown files
- Multiple section types supported
- Priority extraction working correctly
- Task IDs auto-generated sequentially
- Dry-run mode provides accurate preview

✅ **Testing:**
- 10/10 comprehensive tests passing
- Tested with real-world markdown files
- Edge cases handled (duplicates, nested lists, etc.)

✅ **Documentation:**
- Command usage documented in CLAUDE.md
- Examples provided in this task file
- Code well-commented and maintainable

## Technical Notes

### Design Decisions
1. **Regex-based parsing** - Simple, flexible, handles various markdown formats
2. **Section-based extraction** - Processes "## Tasks", "## Action Items", etc. sections
3. **Priority mapping** - Maps [HIGH], [CRITICAL], etc. to TaskGuard priorities
4. **Auto-ID generation** - Finds next available number (area-001, area-002...)
5. **Dry-run first** - Encourages preview before creating files

### Limitations & Future Enhancements
- Currently doesn't parse "## Breaking Scenarios" or "## Requirements" sections
- No --start-number flag (always starts from next available)
- No dependency auto-creation from task relationships
- No bi-directional linking (markdown → tasks only)

### Known Issues
- Pre-existing test failures in `ai_integration_tests` and `cli_integration_tests`
- These are NOT related to import-md implementation
- All 10 import-md specific tests passing

## Usage Examples

### Example 1: GitHub Analysis Import
```bash
# Preview import
taskguard import-md PR2_GITHUB_COMPATIBILITY_ANALYSIS.md \
  --area github --prefix gh --dry-run

# Create tasks
taskguard import-md PR2_GITHUB_COMPATIBILITY_ANALYSIS.md \
  --area github --prefix gh

# Result: tasks/github/gh-001.md through gh-008.md created
```

### Example 2: Causality Analysis Import
```bash
# Import causality chain issues
taskguard import-md CAUSALITY_CHAIN_INTEGRITY_ISSUES.md \
  --area causality --prefix caus

# Result: tasks/causality/caus-001.md through caus-006.md created
```

### Example 3: Dry-Run Output
```
🔍 DRY RUN MODE - No files will be created
📝 Processing: PR2_GITHUB_COMPATIBILITY_ANALYSIS.md

✅ Would create 8 tasks in area 'github':
   📄 gh-001: [HIGH] Implement GitHub Projects V2 OAuth scope
      Priority: high
      File: tasks/github/gh-001.md

   📄 gh-002: [HIGH] Add GraphQL API client for Projects V2
      Priority: high
      File: tasks/github/gh-002.md

💡 To create these tasks, run without --dry-run
```

## Testing

### Test Coverage
```bash
# Run import-md tests only
cargo test import_md_tests -- --nocapture

# Run with verbose output
cargo test import_md_tests -- --nocapture --test-threads=1
```

### Test Results
```
running 10 tests
test test_extract_tasks_from_section ... ok
test test_numbered_list_extraction ... ok
test test_action_items_extraction ... ok
test test_priority_extraction ... ok
test test_multiple_sections ... ok
test test_nested_lists ... ok
test test_duplicate_task_detection ... ok
test test_generate_task_id ... ok
test test_extract_priority_from_title ... ok
test test_create_task_file_content ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Version Control
- [x] Committed import_md.rs implementation
- [x] Committed import_md_tests.rs test suite
- [x] Updated CLAUDE.md with command documentation
- [x] Updated this task file with comprehensive documentation

## Updates
- 2025-10-30: Task created
- 2025-10-30: Implementation completed and tested (10/10 tests passing)
- 2025-10-30: Documentation completed in this task file
- 2025-10-30: Task marked as done - feature fully functional
