use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskYaml {
    pub id: String,
    pub title: String,
    pub status: TaskStatus,
    pub priority: Priority,
    pub tags: Vec<String>,
    pub dependencies: Vec<String>,
    pub assignee: Option<String>,
    pub created: DateTime<Utc>,
    pub estimate: Option<String>,
    pub complexity: Option<u8>,
    pub area: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    #[serde(default = "default_status")]
    pub status: TaskStatus,
    #[serde(default = "default_priority")]
    pub priority: Priority,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub dependencies: Vec<String>,
    pub assignee: Option<String>,
    #[serde(default = "default_created")]
    pub created: DateTime<Utc>,
    pub estimate: Option<String>,
    pub complexity: Option<u8>,
    pub area: String,
    #[serde(skip)]
    pub content: String,
}

fn default_status() -> TaskStatus {
    TaskStatus::Todo
}

fn default_priority() -> Priority {
    Priority::Medium
}

fn default_created() -> DateTime<Utc> {
    Utc::now()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    #[serde(rename = "todo")]
    Todo,
    #[serde(rename = "doing")]
    Doing,
    #[serde(rename = "review")]
    Review,
    #[serde(rename = "done")]
    Done,
    #[serde(rename = "blocked")]
    Blocked,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Todo => write!(f, "todo"),
            TaskStatus::Doing => write!(f, "doing"),
            TaskStatus::Review => write!(f, "review"),
            TaskStatus::Done => write!(f, "done"),
            TaskStatus::Blocked => write!(f, "blocked"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "critical")]
    Critical,
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Low => write!(f, "low"),
            Priority::Medium => write!(f, "medium"),
            Priority::High => write!(f, "high"),
            Priority::Critical => write!(f, "critical"),
        }
    }
}

impl Task {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read task file: {}", path.as_ref().display()))?;

        Self::parse_content(&content)
    }

    pub fn parse_content(content: &str) -> Result<Self> {
        // Split YAML front-matter from markdown content
        let parts: Vec<&str> = content.splitn(3, "---").collect();

        if parts.len() < 3 {
            return Err(anyhow::anyhow!("Invalid task file format: missing YAML front-matter"));
        }

        let yaml_content = parts[1].trim();
        let markdown_content = parts[2].trim();

        // Parse YAML front-matter
        let mut task: Task = serde_yaml::from_str(yaml_content)
            .with_context(|| format!("Failed to parse YAML front-matter: {}", yaml_content))?;

        // Add markdown content
        task.content = markdown_content.to_string();

        Ok(task)
    }

    pub fn to_file_content(&self) -> Result<String> {
        // Create a copy without the content field for YAML serialization
        let yaml_task = TaskYaml {
            id: self.id.clone(),
            title: self.title.clone(),
            status: self.status.clone(),
            priority: self.priority.clone(),
            tags: self.tags.clone(),
            dependencies: self.dependencies.clone(),
            assignee: self.assignee.clone(),
            created: self.created,
            estimate: self.estimate.clone(),
            complexity: self.complexity,
            area: self.area.clone(),
        };

        let mut yaml = serde_yaml::to_string(&yaml_task)
            .context("Failed to serialize task to YAML")?;

        // Clean up null values in YAML
        yaml = yaml.replace("estimate: null\n", "estimate: ~\n");

        Ok(format!("---\n{}\n---\n\n{}", yaml.trim(), self.content))
    }

    pub fn file_name(&self) -> String {
        format!("{}.md", self.id)
    }

    pub fn is_available(&self, completed_tasks: &[String]) -> bool {
        // Task is available if all dependencies are completed
        self.dependencies.iter().all(|dep| completed_tasks.contains(dep))
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = self.to_file_content()?;

        // Ensure parent directory exists
        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }

        fs::write(&path, content)
            .with_context(|| format!("Failed to write task file: {}", path.as_ref().display()))?;

        Ok(())
    }
}