use crate::task::TaskStatus;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Represents the mapping between a TaskGuard task and GitHub issue/project item
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IssueMapping {
    pub task_id: String,
    pub issue_number: i64,
    pub issue_id: String,
    pub project_item_id: String,
    pub synced_at: String,
    pub is_archived: bool,
}

/// Manages persistent mappings between TaskGuard tasks and GitHub entities
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TaskIssueMapper {
    mappings: Vec<IssueMapping>,
    #[serde(skip)]
    file_path: Option<PathBuf>,
}

impl TaskIssueMapper {
    /// Create a new mapper with the default storage path
    pub fn new() -> Result<Self, std::io::Error> {
        let root = crate::config::find_taskguard_root()
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Not in a TaskGuard project. Run 'taskguard init' first.",
                )
            })?;

        let file_path = root.join(".taskguard").join("github-mapping.json");

        let mut mapper = Self {
            mappings: Vec::new(),
            file_path: Some(file_path.clone()),
        };

        // Try to load existing mappings
        if file_path.exists() {
            mapper.load()?;
        }

        Ok(mapper)
    }

    /// Create a mapper with a custom file path (useful for testing)
    pub fn with_path(path: PathBuf) -> Self {
        Self {
            mappings: Vec::new(),
            file_path: Some(path),
        }
    }

    /// Load mappings from the JSON file
    pub fn load(&mut self) -> Result<(), std::io::Error> {
        let path = self.file_path.as_ref().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "No file path set")
        })?;

        if !path.exists() {
            return Ok(()); // No file yet, that's fine
        }

        let content = fs::read_to_string(path)?;
        let loaded: Self = serde_json::from_str(&content).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
        })?;

        self.mappings = loaded.mappings;
        Ok(())
    }

    /// Save mappings to the JSON file
    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = self.file_path.as_ref().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "No file path set")
        })?;

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Add a new mapping
    pub fn add_mapping(&mut self, mapping: IssueMapping) -> Result<(), std::io::Error> {
        // Check for duplicates
        if self.mappings.iter().any(|m| m.task_id == mapping.task_id) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                format!("Mapping for task {} already exists", mapping.task_id),
            ));
        }

        self.mappings.push(mapping);
        self.save()
    }

    /// Update an existing mapping
    pub fn update_mapping(&mut self, mapping: IssueMapping) -> Result<(), std::io::Error> {
        let pos = self
            .mappings
            .iter()
            .position(|m| m.task_id == mapping.task_id)
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Mapping for task {} not found", mapping.task_id),
                )
            })?;

        self.mappings[pos] = mapping;
        self.save()
    }

    /// Remove a mapping by task ID
    pub fn remove_mapping(&mut self, task_id: &str) -> Result<(), std::io::Error> {
        let initial_len = self.mappings.len();
        self.mappings.retain(|m| m.task_id != task_id);

        if self.mappings.len() == initial_len {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Mapping for task {} not found", task_id),
            ));
        }

        self.save()
    }

    /// Get a mapping by task ID
    pub fn get_by_task_id(&self, task_id: &str) -> Option<&IssueMapping> {
        self.mappings.iter().find(|m| m.task_id == task_id)
    }

    /// Get a mapping by issue number
    pub fn get_by_issue_number(&self, issue_number: i64) -> Option<&IssueMapping> {
        self.mappings.iter().find(|m| m.issue_number == issue_number)
    }

    /// Get a mapping by project item ID
    pub fn get_by_project_item_id(&self, project_item_id: &str) -> Option<&IssueMapping> {
        self.mappings
            .iter()
            .find(|m| m.project_item_id == project_item_id)
    }

    /// Get all active (non-archived) mappings
    pub fn get_active_mappings(&self) -> Vec<&IssueMapping> {
        self.mappings.iter().filter(|m| !m.is_archived).collect()
    }

    /// Get all archived mappings
    pub fn get_archived_mappings(&self) -> Vec<&IssueMapping> {
        self.mappings.iter().filter(|m| m.is_archived).collect()
    }

    /// Get all mappings
    pub fn get_all_mappings(&self) -> &[IssueMapping] {
        &self.mappings
    }

    /// Archive a task mapping
    pub fn archive_mapping(&mut self, task_id: &str) -> Result<(), std::io::Error> {
        let mapping = self
            .mappings
            .iter_mut()
            .find(|m| m.task_id == task_id)
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Mapping for task {} not found", task_id),
                )
            })?;

        mapping.is_archived = true;
        self.save()
    }

    /// Unarchive a task mapping
    pub fn unarchive_mapping(&mut self, task_id: &str) -> Result<(), std::io::Error> {
        let mapping = self
            .mappings
            .iter_mut()
            .find(|m| m.task_id == task_id)
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Mapping for task {} not found", task_id),
                )
            })?;

        mapping.is_archived = false;
        self.save()
    }

    /// Find the best matching status option from available GitHub project columns
    ///
    /// This function implements priority-based matching to handle real-world column names.
    /// For example, TaskStatus::Doing will try "In progress" before "Doing" to match
    /// common GitHub Projects v2 column naming conventions.
    ///
    /// # Arguments
    /// * `status` - The TaskGuard status to convert
    /// * `available_options` - List of (option_id, option_name) tuples from GitHub
    ///
    /// # Returns
    /// The option_id of the best matching column, or None if no match found
    pub fn find_best_status_option(
        status: &TaskStatus,
        available_options: &[(String, String)],
    ) -> Option<String> {
        let priority_names = get_status_priority_names(status);

        // Create a case-insensitive lookup map
        let mut option_map: HashMap<String, String> = HashMap::new();
        for (id, name) in available_options {
            option_map.insert(name.to_lowercase(), id.clone());
        }

        // Try each priority name in order
        for priority_name in priority_names {
            if let Some(option_id) = option_map.get(&priority_name.to_lowercase()) {
                return Some(option_id.clone());
            }
        }

        None
    }

    /// Convert a GitHub column name to a TaskGuard status
    ///
    /// Uses pattern matching to handle various common column naming conventions.
    /// Defaults to Todo for unknown columns.
    ///
    /// # Arguments
    /// * `column_name` - The GitHub Projects v2 column name
    ///
    /// # Returns
    /// The corresponding TaskStatus
    pub fn github_column_to_status(column_name: &str) -> TaskStatus {
        let normalized = column_name.to_lowercase();

        if normalized.contains("done") || normalized.contains("complete") {
            TaskStatus::Done
        } else if normalized.contains("review") {
            TaskStatus::Review
        } else if normalized.contains("progress") || normalized.contains("doing") || normalized.contains("working") {
            TaskStatus::Doing
        } else if normalized.contains("blocked") {
            TaskStatus::Blocked
        } else {
            // Default to Todo for "Backlog", "Ready", "To Do", or unknown columns
            TaskStatus::Todo
        }
    }
}

/// Get priority-ordered list of column names to try for a given status
fn get_status_priority_names(status: &TaskStatus) -> Vec<String> {
    match status {
        TaskStatus::Todo => vec![
            "Backlog".to_string(),
            "Todo".to_string(),
            "To Do".to_string(),
            "Ready".to_string(),
        ],
        TaskStatus::Doing => vec![
            "In progress".to_string(),
            "In Progress".to_string(),
            "Doing".to_string(),
            "Working".to_string(),
        ],
        TaskStatus::Review => vec![
            "In review".to_string(),
            "In Review".to_string(),
            "Review".to_string(),
            "Reviewing".to_string(),
        ],
        TaskStatus::Done => vec![
            "Done".to_string(),
            "Completed".to_string(),
            "Complete".to_string(),
        ],
        TaskStatus::Blocked => vec![
            "Blocked".to_string(),
            "Backlog".to_string(), // Fallback to backlog if no blocked column
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::fs;

    fn create_test_mapping(task_id: &str, issue_number: i64) -> IssueMapping {
        IssueMapping {
            task_id: task_id.to_string(),
            issue_number,
            issue_id: format!("issue_{}", issue_number),
            project_item_id: format!("item_{}", issue_number),
            synced_at: Utc::now().to_rfc3339(),
            is_archived: false,
        }
    }

    fn create_test_mapper() -> TaskIssueMapper {
        let temp_dir = std::env::temp_dir()
            .join(format!("taskguard_test_{}", std::process::id()))
            .join(".taskguard");
        fs::create_dir_all(&temp_dir).unwrap();
        let temp_path = temp_dir.join("github-mapping.json");
        TaskIssueMapper::with_path(temp_path)
    }

    #[test]
    fn test_add_mapping() {
        let mut mapper = create_test_mapper();
        let mapping = create_test_mapping("task-001", 42);

        mapper.add_mapping(mapping.clone()).unwrap();
        assert_eq!(mapper.mappings.len(), 1);
        assert_eq!(mapper.get_by_task_id("task-001").unwrap(), &mapping);
    }

    #[test]
    fn test_add_duplicate_mapping() {
        let mut mapper = create_test_mapper();
        let mapping = create_test_mapping("task-001", 42);

        mapper.add_mapping(mapping.clone()).unwrap();
        let result = mapper.add_mapping(mapping);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_mapping() {
        let mut mapper = create_test_mapper();
        let mut mapping = create_test_mapping("task-001", 42);
        mapper.add_mapping(mapping.clone()).unwrap();

        mapping.is_archived = true;
        mapper.update_mapping(mapping.clone()).unwrap();

        let updated = mapper.get_by_task_id("task-001").unwrap();
        assert!(updated.is_archived);
    }

    #[test]
    fn test_remove_mapping() {
        let mut mapper = create_test_mapper();
        let mapping = create_test_mapping("task-001", 42);
        mapper.add_mapping(mapping).unwrap();

        mapper.remove_mapping("task-001").unwrap();
        assert_eq!(mapper.mappings.len(), 0);
    }

    #[test]
    fn test_get_by_issue_number() {
        let mut mapper = create_test_mapper();
        let mapping = create_test_mapping("task-001", 42);
        mapper.add_mapping(mapping.clone()).unwrap();

        let found = mapper.get_by_issue_number(42).unwrap();
        assert_eq!(found.task_id, "task-001");
    }

    #[test]
    fn test_get_by_project_item_id() {
        let mut mapper = create_test_mapper();
        let mapping = create_test_mapping("task-001", 42);
        mapper.add_mapping(mapping).unwrap();

        let found = mapper.get_by_project_item_id("item_42").unwrap();
        assert_eq!(found.task_id, "task-001");
    }

    #[test]
    fn test_archive_unarchive() {
        let mut mapper = create_test_mapper();
        let mapping = create_test_mapping("task-001", 42);
        mapper.add_mapping(mapping).unwrap();

        mapper.archive_mapping("task-001").unwrap();
        assert_eq!(mapper.get_archived_mappings().len(), 1);
        assert_eq!(mapper.get_active_mappings().len(), 0);

        mapper.unarchive_mapping("task-001").unwrap();
        assert_eq!(mapper.get_archived_mappings().len(), 0);
        assert_eq!(mapper.get_active_mappings().len(), 1);
    }

    #[test]
    fn test_find_best_status_option() {
        let options = vec![
            ("opt1".to_string(), "Backlog".to_string()),
            ("opt2".to_string(), "In progress".to_string()),
            ("opt3".to_string(), "Done".to_string()),
        ];

        // Test exact match
        let result = TaskIssueMapper::find_best_status_option(&TaskStatus::Doing, &options);
        assert_eq!(result, Some("opt2".to_string()));

        // Test case-insensitive match
        let options_mixed_case = vec![
            ("opt1".to_string(), "BACKLOG".to_string()),
            ("opt2".to_string(), "IN PROGRESS".to_string()),
        ];
        let result = TaskIssueMapper::find_best_status_option(&TaskStatus::Doing, &options_mixed_case);
        assert_eq!(result, Some("opt2".to_string()));
    }

    #[test]
    fn test_find_best_status_option_priority() {
        // When multiple matches exist, should prefer higher priority
        let options = vec![
            ("opt1".to_string(), "Working".to_string()),
            ("opt2".to_string(), "In progress".to_string()), // Higher priority
        ];

        let result = TaskIssueMapper::find_best_status_option(&TaskStatus::Doing, &options);
        assert_eq!(result, Some("opt2".to_string()));
    }

    #[test]
    fn test_github_column_to_status() {
        assert_eq!(
            TaskIssueMapper::github_column_to_status("Done"),
            TaskStatus::Done
        );
        assert_eq!(
            TaskIssueMapper::github_column_to_status("Completed"),
            TaskStatus::Done
        );
        assert_eq!(
            TaskIssueMapper::github_column_to_status("In Progress"),
            TaskStatus::Doing
        );
        assert_eq!(
            TaskIssueMapper::github_column_to_status("in progress"),
            TaskStatus::Doing
        );
        assert_eq!(
            TaskIssueMapper::github_column_to_status("Review"),
            TaskStatus::Review
        );
        assert_eq!(
            TaskIssueMapper::github_column_to_status("Blocked"),
            TaskStatus::Blocked
        );
        assert_eq!(
            TaskIssueMapper::github_column_to_status("Backlog"),
            TaskStatus::Todo
        );
        assert_eq!(
            TaskIssueMapper::github_column_to_status("Unknown Column"),
            TaskStatus::Todo
        );
    }
}
