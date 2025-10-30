---
id: github-infra-003
title: Implement Task-Issue Mapper
status: todo
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

# Implement Task-Issue Mapper

## Context
We need a persistent mapping between TaskGuard task IDs and GitHub issue numbers/IDs. This enables bidirectional sync and prevents duplicate issue creation.

## Dependencies
**Requires:** github-infra-002 (GitHub API client)
**Why:** Mapper needs client to query GitHub for issue information

## Objectives
1. Create mapping storage in `.taskguard/github-mapping.json`
2. Implement CRUD operations for task-issue mappings
3. Add lookup functions (task → issue, issue → task)
4. Handle archived tasks in mapping

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
    pub issue_number: u32,
    pub issue_id: String,           // GraphQL node ID
    pub project_item_id: Option<String>, // For Projects v2
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
- Can add new mappings
- Can retrieve mappings by task ID
- Can find task ID by issue number
- Can mark tasks as archived
- Can remove mappings

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

- Use JSON for human readability (could switch to bincode for performance)
- Store in `.taskguard/` directory (gitignored by default)
- Keep mappings for archived tasks (needed for sync)
- Consider adding timestamps for staleness detection
- Future: Add file locking for concurrent access

## Session Handoff Template

### What Changed
- [Created src/github/mapper.rs with TaskIssueMapper]
- [Added IssueMapping type with archived tracking]
- [Implemented persistence with JSON serialization]

### Causality Impact
- **Load → Memory**: Mappings loaded from disk into HashMap
- **Modify → Save**: Changes written back to JSON file
- **Archive → Mark**: Archive operations update is_archived flag
- **Sync → Query**: Sync operations query mappings for correlation

### Dependencies Unblocked
- github-infra-004: Mutations (needs mapper to prevent duplicate issues)
- github-infra-005: Queries (needs mapper to correlate data)

### Next Task Context
The mutations module (github-infra-004) will use the mapper to:
1. Check if task already has a GitHub issue before creating
2. Update the mapping when creating new issues
3. Mark issues as archived when tasks are archived
4. Keep mapping in sync with GitHub state
