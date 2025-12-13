---
id: github-infra-003
title: Implement Task-Issue Mapper
status: done
priority: high
tags:
- github
- infrastructure
dependencies: [github-infra-002]
assignee: developer
created: 2025-10-30T21:50:00Z
estimate: 3h
complexity: 6
area: github
---

# Implement Task-Issue-Project Mapper

## Context
**PROJECTS V2 FOCUS**: We need persistent mapping between TaskGuard task IDs, GitHub issues, AND **GitHub Projects v2 board items**. This enables:
1. Bidirectional sync with Projects dashboard
2. Prevents duplicate issue/project item creation
3. Tracks task position on project board
4. Maps TaskGuard status ↔ Project column status

## Dependencies
**Requires:** github-infra-002 (GitHub API client)
**Why:** Mapper needs client to query GitHub for issue information

## Objectives
1. Create mapping storage in `.taskguard/github-mapping.json`
2. Implement CRUD operations for task-issue-project mappings
3. Add lookup functions (task → issue, issue → task, **task → project_item**)
4. Implement status conversion: TaskGuard status ↔ GitHub Projects column
5. Cache project field IDs for efficient updates
6. Handle archived tasks in mapping

## Implementation Plan

### 1. Create Mapping Types (src/github/mapper.rs)

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use anyhow::{Context, Result};
use crate::config::find_taskguard_root;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueMapping {
    pub task_id: String,
    pub issue_number: i64,          // Changed from u32 to i64 for GraphQL
    pub issue_id: String,           // GraphQL node ID
    pub project_item_id: String,    // REQUIRED: Projects v2 item ID (not optional)
    pub synced_at: String,          // ISO 8601 timestamp
    pub is_archived: bool,          // Track if task is archived
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskIssueMapper {
    mappings: HashMap<String, IssueMapping>, // key: task_id
    #[serde(skip)]
    file_path: PathBuf,
}

impl TaskIssueMapper {
    /// Load existing mappings or create new
    pub fn load() -> Result<Self> {
        let root = find_taskguard_root()
            .context("Not in a TaskGuard project")?;
        let file_path = root.join(".taskguard/github-mapping.json");

        if file_path.exists() {
            let content = fs::read_to_string(&file_path)
                .context("Failed to read mapping file")?;
            let mut mapper: TaskIssueMapper = serde_json::from_str(&content)
                .context("Failed to parse mapping file")?;
            mapper.file_path = file_path;
            Ok(mapper)
        } else {
            Ok(TaskIssueMapper {
                mappings: HashMap::new(),
                file_path,
            })
        }
    }

    /// Save mappings to disk
    pub fn save(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self)
            .context("Failed to serialize mappings")?;
        fs::write(&self.file_path, json)
            .context("Failed to write mapping file")?;
        Ok(())
    }

    /// Add or update a mapping
    pub fn set_mapping(&mut self, mapping: IssueMapping) {
        self.mappings.insert(mapping.task_id.clone(), mapping);
    }

    /// Get mapping by task ID
    pub fn get_mapping(&self, task_id: &str) -> Option<&IssueMapping> {
        self.mappings.get(task_id)
    }

    /// Get task ID by issue number
    pub fn get_task_id_by_issue(&self, issue_number: u32) -> Option<String> {
        self.mappings
            .values()
            .find(|m| m.issue_number == issue_number)
            .map(|m| m.task_id.clone())
    }

    /// Mark task as archived in mapping
    pub fn mark_archived(&mut self, task_id: &str) -> Result<()> {
        if let Some(mapping) = self.mappings.get_mut(task_id) {
            mapping.is_archived = true;
            Ok(())
        } else {
            Err(anyhow::anyhow!("No mapping found for task: {}", task_id))
        }
    }

    /// Get all active (non-archived) mappings
    pub fn get_active_mappings(&self) -> Vec<&IssueMapping> {
        self.mappings
            .values()
            .filter(|m| !m.is_archived)
            .collect()
    }

    /// Get all archived mappings
    pub fn get_archived_mappings(&self) -> Vec<&IssueMapping> {
        self.mappings
            .values()
            .filter(|m| m.is_archived)
            .collect()
    }

    /// Remove a mapping
    pub fn remove_mapping(&mut self, task_id: &str) -> Option<IssueMapping> {
        self.mappings.remove(task_id)
    }

    /// Get project item ID by task ID
    pub fn get_project_item_id(&self, task_id: &str) -> Option<&str> {
        self.mappings
            .get(task_id)
            .map(|m| m.project_item_id.as_str())
    }
}

// Status Conversion for Projects v2 (Enhanced for flexible mapping)
impl TaskIssueMapper {
    /// Convert TaskGuard status to best-match GitHub Projects column name
    /// Returns a priority list of possible column names to try
    pub fn taskguard_status_to_project_options(status: &TaskStatus) -> Vec<&'static str> {
        match status {
            TaskStatus::Todo => vec!["Backlog", "Todo", "To Do", "Ready"],
            TaskStatus::Doing => vec!["In progress", "In Progress", "Doing", "Working"],
            TaskStatus::Review => vec!["In review", "Review", "Reviewing"],
            TaskStatus::Done => vec!["Done", "Completed", "Complete"],
            TaskStatus::Blocked => vec!["Blocked", "Backlog"], // Fallback to Backlog if no Blocked column
        }
    }

    /// Find best matching project option ID for a TaskGuard status
    /// Takes available project options and returns the best match
    pub fn find_best_status_option(
        status: &TaskStatus,
        available_options: &[(String, String)], // (option_id, option_name)
    ) -> Option<String> {
        let preferences = Self::taskguard_status_to_project_options(status);

        // Try each preference in order
        for preferred_name in preferences {
            if let Some((id, _)) = available_options
                .iter()
                .find(|(_, name)| name.eq_ignore_ascii_case(preferred_name))
            {
                return Some(id.clone());
            }
        }

        // No match found
        None
    }

    /// Convert GitHub Projects column to TaskGuard status
    /// Enhanced with more patterns from real-world project boards
    pub fn project_column_to_taskguard_status(column: &str) -> TaskStatus {
        let normalized = column.to_lowercase();

        // Todo/Backlog patterns
        if normalized.contains("todo")
            || normalized.contains("backlog")
            || normalized.contains("ready")
            || normalized == "to do" {
            return TaskStatus::Todo;
        }

        // In Progress patterns
        if normalized.contains("in progress")
            || normalized.contains("in-progress")
            || normalized.contains("doing")
            || normalized.contains("working") {
            return TaskStatus::Doing;
        }

        // Review patterns
        if normalized.contains("review") {
            return TaskStatus::Review;
        }

        // Done patterns
        if normalized.contains("done")
            || normalized.contains("complete")
            || normalized.contains("closed")
            || normalized.contains("finished") {
            return TaskStatus::Done;
        }

        // Blocked pattern
        if normalized.contains("blocked") {
            return TaskStatus::Blocked;
        }

        // Default fallback
        TaskStatus::Todo
    }
}
```

### 2. Add to Module Exports (src/github/mod.rs)

```rust
pub mod client;
pub mod types;
pub mod config;
pub mod mapper;

pub use client::GitHubClient;
pub use types::*;
pub use config::*;
pub use mapper::*;
```

## Acceptance Criteria

✅ **Persistence:**
- Mappings stored in `.taskguard/github-mapping.json`
- JSON format is human-readable
- File survives process restarts

✅ **CRUD Operations:**
- Can add new mappings (with project_item_id)
- Can retrieve mappings by task ID
- Can find task ID by issue number
- Can get project_item_id by task ID (NEW)
- Can mark tasks as archived
- Can remove mappings

✅ **Status Conversion (Projects v2 - ENHANCED):**
- **Flexible TaskGuard → Projects mapping**: Returns priority list of possible column names
- **Smart matching**: Finds best available option from user's project configuration
- **Bidirectional**: GitHub Projects column → TaskGuard status with pattern matching
- **Real-world patterns**: Supports "Backlog", "Ready", "In progress", "In review", etc.
- **Dashboard compatibility**: Matches columns seen in user screenshots
- **Graceful fallback**: Default to Todo for unknown columns

✅ **Filtering:**
- Can get only active mappings
- Can get only archived mappings
- Archived flag properly maintained

✅ **Error Handling:**
- Graceful handling of missing file
- Proper error messages for invalid JSON
- Safe concurrent access (file-based locking future)

## Testing

```bash
# Build project
cargo build

# Test will be added in integration test suite
# Manual test:
# 1. Create mapper and add mapping
# 2. Save to file
# 3. Load from file
# 4. Verify mapping preserved
```

## Technical Notes

### Persistence
- Use JSON for human readability (could switch to bincode for performance)
- Store in `.taskguard/` directory (gitignored by default)
- Keep mappings for archived tasks (needed for sync)
- Consider adding timestamps for staleness detection
- Future: Add file locking for concurrent access

### Status Mapping Strategy (Projects v2)
- **Priority-based matching**: Try multiple column name variations in order of preference
- **Real-world examples** (from user dashboard):
  - User has: "Backlog", "Ready", "In progress", "In review", "Done"
  - TaskGuard `todo` tries: Backlog → Todo → To Do → Ready
  - Finds "Backlog" first, uses that option ID
- **Bidirectional conversion**:
  - Forward: TaskGuard status → List of possible column names → Find best match
  - Reverse: GitHub column name → Pattern matching → TaskGuard status
- **Case-insensitive matching**: Handles "In Progress", "in progress", "IN PROGRESS"
- **Partial matching**: "in review" matches "In review", "Review in progress", etc.
- **Graceful degradation**: Falls back to Todo if no good match found

## Session Handoff Template

### What Changed
- [Created src/github/mapper.rs with TaskIssueMapper]
- [Added IssueMapping type with project_item_id (REQUIRED) and archived tracking]
- [Implemented persistence with JSON serialization]
- **[NEW]** [Enhanced status conversion with priority-based matching]
- **[NEW]** [Added find_best_status_option() for flexible column mapping]
- **[NEW]** [Pattern-matching for bidirectional status conversion]

### Causality Impact

#### Persistence Chain
- **Load → Memory**: Mappings loaded from disk into HashMap
- **Modify → Save**: Changes written back to JSON file
- **Archive → Mark**: Archive operations update is_archived flag
- **Sync → Query**: Sync operations query mappings for correlation

#### Status Mapping Chain (NEW - CRITICAL for Dashboard)
- **TaskGuard Status → Options List**: Get priority-ordered column names to try
- **Query Project → Available Options**: Get actual column names from user's project
- **Find Best Match → Option ID**: Smart matching returns correct option ID
- **Update Status → Dashboard**: User sees task in correct column
- **Read Column → TaskGuard Status**: Bidirectional sync from GitHub back to TaskGuard

### Dependencies Unblocked
- **github-infra-004**: Mutations (needs mapper to prevent duplicate issues AND find option IDs)
- **github-infra-005**: Queries (needs mapper to correlate data and map statuses)
- **github-sync-001**: Full sync workflow with flexible status mapping

### Next Task Context

#### For github-infra-004 (Mutations)
The mutations module will use the mapper to:
1. Check if task already has a GitHub issue before creating
2. Update the mapping with project_item_id after adding to project
3. Use find_best_status_option() to get correct option ID for status updates
4. Mark issues as archived when tasks are archived

#### Example Usage Flow
```rust
// 1. Get available options from project
let (field_id, options) = GitHubMutations::get_status_field_info(&client, &project_id)?;

// 2. Find best match for TaskGuard status
let task_status = TaskStatus::Doing;
let option_id = TaskIssueMapper::find_best_status_option(&task_status, &options)
    .ok_or("No matching status column found")?;

// 3. Update project item with matched option
GitHubMutations::update_project_item_status(
    &client, &project_id, &item_id, &field_id, &option_id
)?;

// Result: User sees task in "In progress" column (their project's name for Doing)
```
