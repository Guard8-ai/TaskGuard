use serde::{Deserialize, Serialize};

/// GitHub Issue representation
/// Issues are used to populate Projects v2 boards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubIssue {
    /// GraphQL node ID
    pub id: String,
    /// Issue number (i64 for GraphQL compatibility)
    pub number: i64,
    /// Issue title
    pub title: String,
    /// Issue state: "OPEN", "CLOSED"
    pub state: String,
    /// Issue body/description
    pub body: Option<String>,
    /// Label names
    pub labels: Vec<String>,
    /// Assignee usernames
    pub assignees: Vec<String>,
}

/// Projects v2 Item - PRIMARY FOCUS
/// Represents an item on a GitHub Projects v2 board
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectItem {
    /// Project item ID
    pub id: String,
    /// Associated issue ID
    pub issue_id: String,
    /// Parent project ID
    pub project_id: String,
    /// Status field value
    pub status: String,
    /// Custom field values
    pub field_values: Vec<FieldValue>,
}

/// Custom field value for Projects v2
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldValue {
    /// Field ID
    pub field_id: String,
    /// Field value
    pub value: String,
}

/// Projects v2 Status Update (2025 API)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectV2StatusUpdate {
    /// Status update ID
    pub id: String,
    /// Status value
    pub status: String,
    /// Optional description
    pub body: Option<String>,
    /// Creation timestamp
    pub created_at: String,
}

/// GitHub configuration for TaskGuard integration
/// Authentication is handled via `gh` CLI - no token stored here
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubConfig {
    /// Repository owner (username or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Projects v2 project number
    pub project_number: i64,
}

/// Mapping between TaskGuard tasks and GitHub entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskMapping {
    /// TaskGuard task ID
    pub task_id: String,
    /// GitHub issue number
    pub issue_number: i64,
    /// GitHub Projects v2 item ID
    pub project_item_id: String,
    /// Last sync timestamp (ISO 8601)
    pub last_synced: String,
}
