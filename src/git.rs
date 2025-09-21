use anyhow::{Result, Context};
use git2::{Repository, Commit};
use std::path::Path;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Git repository analysis for TaskGuard intelligence features
pub struct GitAnalyzer {
    repo: Repository,
}

/// Represents a commit that potentially relates to a task
#[derive(Debug, Clone)]
pub struct TaskCommit {
    pub oid: String,
    pub message: String,
    pub author: String,
    pub timestamp: DateTime<Utc>,
    pub task_ids: Vec<String>,
}

/// Analysis results for task activity
#[derive(Debug)]
pub struct TaskActivity {
    pub task_id: String,
    pub commits: Vec<TaskCommit>,
    pub last_activity: Option<DateTime<Utc>>,
    pub suggested_status: Option<String>,
    pub confidence: f32,
}

impl GitAnalyzer {
    /// Create a new GitAnalyzer for the given repository path
    pub fn new<P: AsRef<Path>>(repo_path: P) -> Result<Self> {
        let repo = Repository::open(repo_path)
            .context("Failed to open Git repository")?;

        Ok(GitAnalyzer { repo })
    }

    /// Analyze recent commits for task-related activity
    pub fn analyze_task_activity(&self, limit: Option<usize>) -> Result<Vec<TaskActivity>> {
        let commits = self.get_recent_commits(limit.unwrap_or(100))?;
        let task_commits = self.parse_task_commits(commits)?;

        // Group commits by task ID
        let mut task_groups: HashMap<String, Vec<TaskCommit>> = HashMap::new();
        for commit in task_commits {
            for task_id in &commit.task_ids {
                task_groups.entry(task_id.clone())
                    .or_insert_with(Vec::new)
                    .push(commit.clone());
            }
        }

        // Convert to TaskActivity structs with analysis
        let mut activities = Vec::new();
        for (task_id, commits) in task_groups {
            let last_activity = commits.iter()
                .map(|c| c.timestamp)
                .max();

            let (suggested_status, confidence) = self.suggest_status(&commits);

            activities.push(TaskActivity {
                task_id,
                commits,
                last_activity,
                suggested_status,
                confidence,
            });
        }

        // Sort by most recent activity
        activities.sort_by(|a, b| {
            b.last_activity.cmp(&a.last_activity)
        });

        Ok(activities)
    }

    /// Get recent commits from the repository
    fn get_recent_commits(&self, limit: usize) -> Result<Vec<Commit>> {
        let mut revwalk = self.repo.revwalk()
            .context("Failed to create revision walker")?;

        revwalk.push_head()
            .context("Failed to push HEAD to revwalk")?;

        let mut commits = Vec::new();
        for (i, oid) in revwalk.enumerate() {
            if i >= limit {
                break;
            }

            let oid = oid.context("Failed to get commit OID")?;
            let commit = self.repo.find_commit(oid)
                .context("Failed to find commit")?;
            commits.push(commit);
        }

        Ok(commits)
    }

    /// Parse commits to extract task-related information
    fn parse_task_commits(&self, commits: Vec<Commit>) -> Result<Vec<TaskCommit>> {
        let mut task_commits = Vec::new();

        for commit in commits {
            let message = commit.message().unwrap_or("").to_string();
            let task_ids = self.extract_task_ids(&message);

            if !task_ids.is_empty() {
                let author = commit.author().name().unwrap_or("Unknown").to_string();
                let timestamp = DateTime::from_timestamp(commit.time().seconds(), 0)
                    .unwrap_or_else(|| Utc::now());

                task_commits.push(TaskCommit {
                    oid: commit.id().to_string(),
                    message,
                    author,
                    timestamp,
                    task_ids,
                });
            }
        }

        Ok(task_commits)
    }

    /// Extract task IDs from commit message using common patterns
    fn extract_task_ids(&self, message: &str) -> Vec<String> {
        let mut task_ids = Vec::new();

        // Pattern 1: area-number format (e.g., "setup-001", "backend-002")
        let task_pattern = regex::Regex::new(r"\b([a-zA-Z]+)-(\d{3})\b").unwrap();
        for cap in task_pattern.captures_iter(message) {
            if let (Some(area), Some(number)) = (cap.get(1), cap.get(2)) {
                task_ids.push(format!("{}-{}", area.as_str(), number.as_str()));
            }
        }

        // Pattern 2: Issue/task references (e.g., "#123", "task 456")
        let issue_pattern = regex::Regex::new(r"(?i)\b(?:task|issue|#)\s*(\d+)\b").unwrap();
        for cap in issue_pattern.captures_iter(message) {
            if let Some(number) = cap.get(1) {
                task_ids.push(format!("task-{}", number.as_str()));
            }
        }

        task_ids
    }

    /// Suggest task status based on commit patterns
    fn suggest_status(&self, commits: &[TaskCommit]) -> (Option<String>, f32) {
        if commits.is_empty() {
            return (None, 0.0);
        }

        let recent_messages: Vec<&str> = commits.iter()
            .take(5) // Look at 5 most recent commits
            .map(|c| c.message.as_str())
            .collect();

        // Analyze commit message patterns for status hints
        let mut indicators = HashMap::new();

        for message in &recent_messages {
            let lower = message.to_lowercase();

            // Completion indicators
            if lower.contains("complete") || lower.contains("finish") || lower.contains("done") {
                *indicators.entry("done").or_insert(0.0) += 0.8;
            }

            // Testing/review indicators
            if lower.contains("test") || lower.contains("fix") || lower.contains("bug") {
                *indicators.entry("review").or_insert(0.0) += 0.6;
            }

            // Work in progress indicators
            if lower.contains("wip") || lower.contains("progress") || lower.contains("implement") {
                *indicators.entry("doing").or_insert(0.0) += 0.7;
            }

            // Initial work indicators
            if lower.contains("start") || lower.contains("initial") || lower.contains("begin") {
                *indicators.entry("doing").or_insert(0.0) += 0.5;
            }
        }

        // Find the highest confidence suggestion
        let (status, confidence) = indicators.into_iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(("doing", 0.3));

        (Some(status.to_string()), confidence)
    }

    /// Get repository statistics
    pub fn get_repo_stats(&self) -> Result<HashMap<String, String>> {
        let mut stats = HashMap::new();

        // Get current branch
        let head = self.repo.head()?;
        let branch_name = head.shorthand().unwrap_or("unknown").to_string();
        stats.insert("current_branch".to_string(), branch_name);

        // Get total commit count (approximate)
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;
        let commit_count = revwalk.count();
        stats.insert("total_commits".to_string(), commit_count.to_string());

        // Get repository state
        let state = match self.repo.state() {
            git2::RepositoryState::Clean => "clean",
            git2::RepositoryState::Merge => "merging",
            git2::RepositoryState::Revert => "reverting",
            git2::RepositoryState::CherryPick => "cherry-picking",
            git2::RepositoryState::Bisect => "bisecting",
            git2::RepositoryState::Rebase => "rebasing",
            _ => "unknown",
        };
        stats.insert("state".to_string(), state.to_string());

        Ok(stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_task_ids() {
        let analyzer = GitAnalyzer::new(".").unwrap();

        let message1 = "Fix authentication bug in backend-001";
        let ids1 = analyzer.extract_task_ids(message1);
        assert_eq!(ids1, vec!["backend-001"]);

        let message2 = "Complete setup-003 and start frontend-001";
        let ids2 = analyzer.extract_task_ids(message2);
        assert!(ids2.contains(&"setup-003".to_string()));
        assert!(ids2.contains(&"frontend-001".to_string()));

        let message3 = "Fix issue #123 and task 456";
        let ids3 = analyzer.extract_task_ids(message3);
        assert!(ids3.contains(&"task-123".to_string()));
        assert!(ids3.contains(&"task-456".to_string()));
    }
}