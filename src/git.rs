use anyhow::{Result, Context};
use git2::{Repository, Commit, FetchOptions, RemoteCallbacks, CertificateCheckStatus};
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

/// Represents sync conflict between local and remote task states
#[derive(Debug)]
pub struct SyncConflict {
    pub task_id: String,
    pub local_status: String,
    pub remote_suggested_status: String,
    pub local_confidence: f32,
    pub remote_confidence: f32,
    pub resolution: ConflictResolution,
}

/// Conflict resolution strategies
#[derive(Debug, PartialEq)]
pub enum ConflictResolution {
    KeepLocal,
    AcceptRemote,
    Interactive,
    NoConflict,
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
    fn get_recent_commits(&self, limit: usize) -> Result<Vec<Commit<'_>>> {
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
    pub fn extract_task_ids(&self, message: &str) -> Vec<String> {
        let mut task_ids = Vec::new();

        // Pattern 1: area-number format (e.g., "setup-001", "backend-002")
        let task_pattern = regex::Regex::new(r"\b([a-zA-Z]+)-(\d{3})\b").unwrap();
        for cap in task_pattern.captures_iter(message) {
            if let (Some(area), Some(number)) = (cap.get(1), cap.get(2)) {
                task_ids.push(format!("{}-{}", area.as_str(), number.as_str()));
            }
        }

        // Pattern 2: Issue/task references (e.g., "#123", "task 456", "issue 123")
        let issue_pattern = regex::Regex::new(r"(?i)(?:\b(?:task|issue)\s*|#)(\d+)\b").unwrap();
        for cap in issue_pattern.captures_iter(message) {
            if let Some(number) = cap.get(1) {
                task_ids.push(format!("task-{}", number.as_str()));
            }
        }

        task_ids
    }

    /// Suggest task status based on commit patterns
    pub fn suggest_status(&self, commits: &[TaskCommit]) -> (Option<String>, f32) {
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

    /// Fetch updates from remote repository with comprehensive error handling
    pub fn fetch_remote(&self, remote_name: &str) -> Result<()> {
        let mut remote = self.repo.find_remote(remote_name)
            .with_context(|| format!("Failed to find remote '{}'. Available remotes: {:?}", remote_name, self.get_remotes().unwrap_or_default()))?;

        let mut callbacks = RemoteCallbacks::new();

        // Handle authentication
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            // Try SSH key from agent first
            if let Some(username) = username_from_url {
                git2::Cred::ssh_key_from_agent(username)
            } else {
                // Fallback to default user
                git2::Cred::ssh_key_from_agent("git")
            }.or_else(|_| {
                // Fallback to username/password prompt would go here
                // For now, return the error
                Err(git2::Error::from_str("Authentication required but no credentials available"))
            })
        });

        // Progress callback for long operations
        callbacks.push_update_reference(|refname, status| {
            if let Some(msg) = status {
                println!("   Reference {}: {}", refname, msg);
            } else {
                println!("   Updated reference: {}", refname);
            }
            Ok(())
        });

        callbacks.update_tips(|refname, _a, _b| {
            println!("   📥 Updating {}", refname);
            true
        });

        // Handle certificate verification
        callbacks.certificate_check(|_cert, _valid| {
            // For now, accept all certificates
            // In production, you might want stricter validation
            Ok(CertificateCheckStatus::CertificateOk)
        });

        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);

        println!("🌐 Fetching from remote '{}'...", remote_name);

        match remote.fetch(&[] as &[&str], Some(&mut fetch_options), None) {
            Ok(()) => {
                println!("   ✅ Fetch completed successfully");
                Ok(())
            },
            Err(e) => {
                let error_msg = match e.class() {
                    git2::ErrorClass::Net => "Network error: Check your internet connection and repository URL",
                    git2::ErrorClass::Ssh => "SSH authentication error: Check your SSH keys and permissions",
                    git2::ErrorClass::Http => "HTTP error: Check repository URL and access permissions",
                    git2::ErrorClass::Ssl => "SSL/TLS error: Check certificate configuration",
                    git2::ErrorClass::Repository => "Repository error: Check if remote repository exists",
                    _ => "Unknown Git error occurred",
                };

                Err(anyhow::anyhow!("{}: {}", error_msg, e))
            }
        }
    }

    /// Analyze remote task activity by comparing with local commits
    pub fn analyze_remote_task_activity(&self, remote_name: &str, limit: Option<usize>) -> Result<Vec<TaskActivity>> {
        // First, ensure we have the latest remote data
        if let Err(e) = self.fetch_remote(remote_name) {
            eprintln!("⚠️  Warning: Failed to fetch from remote: {}", e);
            eprintln!("    Proceeding with locally cached remote data...");
        }

        // Get remote tracking branch commits
        let remote_commits = self.get_remote_commits(remote_name, limit.unwrap_or(100))?;
        let remote_task_commits = self.parse_task_commits(remote_commits)?;

        // Group commits by task ID for remote analysis
        let mut remote_task_groups: HashMap<String, Vec<TaskCommit>> = HashMap::new();
        for commit in remote_task_commits {
            for task_id in &commit.task_ids {
                remote_task_groups.entry(task_id.clone())
                    .or_insert_with(Vec::new)
                    .push(commit.clone());
            }
        }

        // Convert to TaskActivity structs with analysis
        let mut remote_activities = Vec::new();
        for (task_id, commits) in remote_task_groups {
            let last_activity = commits.iter()
                .map(|c| c.timestamp)
                .max();

            let (suggested_status, confidence) = self.suggest_status(&commits);

            remote_activities.push(TaskActivity {
                task_id,
                commits,
                last_activity,
                suggested_status,
                confidence,
            });
        }

        // Sort by most recent activity
        remote_activities.sort_by(|a, b| {
            b.last_activity.cmp(&a.last_activity)
        });

        Ok(remote_activities)
    }

    /// Get commits from remote tracking branch
    fn get_remote_commits(&self, remote_name: &str, limit: usize) -> Result<Vec<Commit<'_>>> {
        let remote_branch_name = format!("{}/master", remote_name); // Assuming master branch
        let remote_ref = self.repo.find_reference(&format!("refs/remotes/{}", remote_branch_name))
            .or_else(|_| self.repo.find_reference(&format!("refs/remotes/{}/main", remote_name)))
            .context("Failed to find remote tracking branch")?;

        let remote_oid = remote_ref.target()
            .context("Failed to get remote branch target")?;

        let mut revwalk = self.repo.revwalk()
            .context("Failed to create revision walker for remote")?;

        revwalk.push(remote_oid)
            .context("Failed to push remote OID to revwalk")?;

        let mut commits = Vec::new();
        for (i, oid) in revwalk.enumerate() {
            if i >= limit {
                break;
            }

            let oid = oid.context("Failed to get remote commit OID")?;
            let commit = self.repo.find_commit(oid)
                .context("Failed to find remote commit")?;
            commits.push(commit);
        }

        Ok(commits)
    }

    /// Compare local and remote task activities to detect conflicts
    pub fn detect_sync_conflicts(&self, local_activities: &[TaskActivity], remote_activities: &[TaskActivity]) -> Vec<SyncConflict> {
        let mut conflicts = Vec::new();

        // Create lookup maps for efficient comparison
        let local_map: HashMap<String, &TaskActivity> = local_activities.iter()
            .map(|a| (a.task_id.clone(), a))
            .collect();

        let remote_map: HashMap<String, &TaskActivity> = remote_activities.iter()
            .map(|a| (a.task_id.clone(), a))
            .collect();

        // Find all unique task IDs from both local and remote
        let mut all_task_ids = std::collections::HashSet::new();
        for activity in local_activities {
            all_task_ids.insert(activity.task_id.clone());
        }
        for activity in remote_activities {
            all_task_ids.insert(activity.task_id.clone());
        }

        for task_id in all_task_ids {
            let local_activity = local_map.get(&task_id);
            let remote_activity = remote_map.get(&task_id);

            let resolution = match (local_activity, remote_activity) {
                (Some(local), Some(remote)) => {
                    // Both have suggestions - check for conflict
                    if let (Some(local_status), Some(remote_status)) = (&local.suggested_status, &remote.suggested_status) {
                        if local_status != remote_status {
                            // Conflict detected - use confidence to suggest resolution
                            if remote.confidence > local.confidence * 1.2 {
                                ConflictResolution::AcceptRemote
                            } else if local.confidence > remote.confidence * 1.2 {
                                ConflictResolution::KeepLocal
                            } else {
                                ConflictResolution::Interactive
                            }
                        } else {
                            ConflictResolution::NoConflict
                        }
                    } else {
                        ConflictResolution::NoConflict
                    }
                },
                (Some(_), None) => ConflictResolution::KeepLocal, // Only local has activity
                (None, Some(_)) => ConflictResolution::AcceptRemote, // Only remote has activity
                (None, None) => continue, // No activity for this task
            };

            if resolution != ConflictResolution::NoConflict {
                conflicts.push(SyncConflict {
                    task_id,
                    local_status: local_activity
                        .and_then(|a| a.suggested_status.clone())
                        .unwrap_or_else(|| "no local activity".to_string()),
                    remote_suggested_status: remote_activity
                        .and_then(|a| a.suggested_status.clone())
                        .unwrap_or_else(|| "no remote activity".to_string()),
                    local_confidence: local_activity.map(|a| a.confidence).unwrap_or(0.0),
                    remote_confidence: remote_activity.map(|a| a.confidence).unwrap_or(0.0),
                    resolution,
                });
            }
        }

        conflicts
    }

    /// Get list of available remotes
    pub fn get_remotes(&self) -> Result<Vec<String>> {
        Ok(self.repo.remotes()?
            .iter()
            .filter_map(|name| name.map(|s| s.to_string()))
            .collect())
    }
}

// Unit tests are in tests/git_analysis_tests.rs