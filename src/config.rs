use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::task::Task;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub project: ProjectConfig,
    pub settings: SettingsConfig,
    pub git: GitConfig,
    pub ai: AiConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    pub areas: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsConfig {
    pub statuses: Vec<String>,
    pub priorities: Vec<String>,
    pub complexity_scale: String,
    pub default_estimate_unit: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitConfig {
    pub auto_add_tasks: bool,
    pub auto_commit_on_status_change: bool,
    pub commit_message_template: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiConfig {
    pub enabled: bool,
    pub claude_code_integration: bool,
    pub auto_suggestions: bool,
    pub complexity_analysis: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            project: ProjectConfig {
                name: "My Project".to_string(),
                version: "1.0.0".to_string(),
                areas: vec![
                    "setup".to_string(),
                    "backend".to_string(),
                    "frontend".to_string(),
                    "api".to_string(),
                    "auth".to_string(),
                    "testing".to_string(),
                    "deployment".to_string(),
                ],
            },
            settings: SettingsConfig {
                statuses: vec![
                    "todo".to_string(),
                    "doing".to_string(),
                    "review".to_string(),
                    "done".to_string(),
                    "blocked".to_string(),
                ],
                priorities: vec![
                    "low".to_string(),
                    "medium".to_string(),
                    "high".to_string(),
                    "critical".to_string(),
                ],
                complexity_scale: "1-10".to_string(),
                default_estimate_unit: "hours".to_string(),
            },
            git: GitConfig {
                auto_add_tasks: true,
                auto_commit_on_status_change: false,
                commit_message_template: "Task {{id}}: {{action}} - {{title}}".to_string(),
            },
            ai: AiConfig {
                enabled: true,
                claude_code_integration: true,
                auto_suggestions: true,
                complexity_analysis: true,
            },
        }
    }
}

impl Config {
    pub fn load_or_default<P: AsRef<Path>>(config_path: P) -> Result<Self> {
        if config_path.as_ref().exists() {
            let content = fs::read_to_string(&config_path).with_context(|| {
                format!(
                    "Failed to read config file: {}",
                    config_path.as_ref().display()
                )
            })?;

            toml::from_str(&content).context("Failed to parse config file")
        } else {
            Ok(Self::default())
        }
    }

    pub fn save<P: AsRef<Path>>(&self, config_path: P) -> Result<()> {
        let content = toml::to_string_pretty(self).context("Failed to serialize config")?;

        fs::write(&config_path, content).with_context(|| {
            format!(
                "Failed to write config file: {}",
                config_path.as_ref().display()
            )
        })?;

        Ok(())
    }
}

pub fn find_taskguard_root() -> Option<PathBuf> {
    let mut current = std::env::current_dir().ok()?;

    loop {
        let taskguard_dir = current.join(".taskguard");
        if taskguard_dir.exists() && taskguard_dir.is_dir() {
            return Some(current);
        }

        if !current.pop() {
            break;
        }
    }

    None
}

pub fn get_tasks_dir() -> Result<PathBuf> {
    let root =
        find_taskguard_root().context("Not in a TaskGuard project. Run 'taskguard init' first.")?;
    Ok(root.join("tasks"))
}

pub fn get_config_path() -> Result<PathBuf> {
    let root =
        find_taskguard_root().context("Not in a TaskGuard project. Run 'taskguard init' first.")?;
    Ok(root.join(".taskguard").join("config.toml"))
}

pub fn get_archive_dir() -> Result<PathBuf> {
    let root =
        find_taskguard_root().context("Not in a TaskGuard project. Run 'taskguard init' first.")?;
    Ok(root.join(".taskguard").join("archive"))
}

/// Load tasks from both active and archive directories
pub fn load_all_tasks() -> Result<Vec<Task>> {
    let mut tasks = Vec::new();

    // Load from tasks/
    let tasks_dir = get_tasks_dir()?;
    if tasks_dir.exists() {
        tasks.extend(load_tasks_from_dir(&tasks_dir)?);
    }

    // Load from .taskguard/archive/
    let archive_dir = get_archive_dir()?;
    if archive_dir.exists() {
        tasks.extend(load_tasks_from_dir(&archive_dir)?);
    }

    Ok(tasks)
}

/// Load tasks from a specific directory (helper function)
pub fn load_tasks_from_dir(dir: &Path) -> Result<Vec<Task>> {
    let mut tasks = Vec::new();

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
    {
        match Task::from_file(entry.path()) {
            Ok(task) => tasks.push(task),
            Err(_) => continue,
        }
    }

    Ok(tasks)
}
