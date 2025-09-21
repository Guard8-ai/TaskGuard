use anyhow::Result;
use chrono::Utc;
use git2::Repository;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;
use taskguard::git::{GitAnalyzer, TaskCommit};

struct TestRepo {
    _temp_dir: TempDir,
    repo_path: PathBuf,
    repo: Repository,
}

impl TestRepo {
    fn new() -> Result<Self> {
        let temp_dir = tempfile::tempdir()?;
        let repo_path = temp_dir.path().to_path_buf();

        // Initialize git repo
        let repo = Repository::init(&repo_path)?;

        // Configure git user (required for commits)
        let mut config = repo.config()?;
        config.set_str("user.name", "Test User")?;
        config.set_str("user.email", "test@example.com")?;

        Ok(TestRepo {
            _temp_dir: temp_dir,
            repo_path,
            repo,
        })
    }

    fn add_commit(&self, message: &str) -> Result<()> {
        // Create a test file
        let file_path = self.repo_path.join("test.txt");
        fs::write(&file_path, "test content")?;

        // Stage the file
        let mut index = self.repo.index()?;
        index.add_path(std::path::Path::new("test.txt"))?;
        index.write()?;

        // Create commit
        let tree_id = index.write_tree()?;
        let tree = self.repo.find_tree(tree_id)?;
        let signature = self.repo.signature()?;

        let parent_commit = if let Ok(head) = self.repo.head() {
            Some(head.peel_to_commit()?)
        } else {
            None
        };

        let parents: Vec<&git2::Commit> = if let Some(ref parent) = parent_commit {
            vec![parent]
        } else {
            vec![]
        };

        self.repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &parents,
        )?;

        Ok(())
    }
}

#[test]
fn test_extract_task_ids_area_number_format() {
    let temp_repo = TestRepo::new().unwrap();
    let analyzer = GitAnalyzer::new(&temp_repo.repo_path).unwrap();

    let message1 = "Fix authentication bug in backend-001";
    let ids1 = analyzer.extract_task_ids(message1);
    assert_eq!(ids1, vec!["backend-001"]);

    let message2 = "Complete setup-003 and start frontend-001";
    let ids2 = analyzer.extract_task_ids(message2);
    assert!(ids2.contains(&"setup-003".to_string()));
    assert!(ids2.contains(&"frontend-001".to_string()));
    assert_eq!(ids2.len(), 2);
}

#[test]
fn test_extract_task_ids_issue_format() {
    let temp_repo = TestRepo::new().unwrap();
    let analyzer = GitAnalyzer::new(&temp_repo.repo_path).unwrap();

    let message1 = "Fix issue #123 and task 456";
    let ids1 = analyzer.extract_task_ids(message1);
    println!("Message: '{}', Found IDs: {:?}", message1, ids1);
    // The regex pattern expects "#123" to match as "task-123"
    assert!(ids1.contains(&"task-123".to_string()));
    assert!(ids1.contains(&"task-456".to_string()));

    let message2 = "Resolve Task 789";
    let ids2 = analyzer.extract_task_ids(message2);
    assert!(ids2.contains(&"task-789".to_string()));
}

#[test]
fn test_extract_task_ids_no_matches() {
    let temp_repo = TestRepo::new().unwrap();
    let analyzer = GitAnalyzer::new(&temp_repo.repo_path).unwrap();

    let message = "Just a regular commit message with no task references";
    let ids = analyzer.extract_task_ids(message);
    assert!(ids.is_empty());
}

#[test]
fn test_extract_task_ids_mixed_case() {
    let temp_repo = TestRepo::new().unwrap();
    let analyzer = GitAnalyzer::new(&temp_repo.repo_path).unwrap();

    let message = "Fix Backend-001 and FRONTEND-002";
    let ids = analyzer.extract_task_ids(message);
    assert!(ids.contains(&"Backend-001".to_string()));
    assert!(ids.contains(&"FRONTEND-002".to_string()));
}

#[test]
fn test_suggest_status_completion_indicators() {
    let temp_repo = TestRepo::new().unwrap();
    let analyzer = GitAnalyzer::new(&temp_repo.repo_path).unwrap();

    let commits = vec![
        TaskCommit {
            oid: "abc123".to_string(),
            message: "Complete authentication feature".to_string(),
            author: "test".to_string(),
            timestamp: Utc::now(),
            task_ids: vec!["auth-001".to_string()],
        }
    ];

    let (status, confidence) = analyzer.suggest_status(&commits);
    assert_eq!(status, Some("done".to_string()));
    assert!(confidence > 0.5);
}

#[test]
fn test_suggest_status_testing_indicators() {
    let temp_repo = TestRepo::new().unwrap();
    let analyzer = GitAnalyzer::new(&temp_repo.repo_path).unwrap();

    let commits = vec![
        TaskCommit {
            oid: "abc123".to_string(),
            message: "Fix tests for user authentication".to_string(),
            author: "test".to_string(),
            timestamp: Utc::now(),
            task_ids: vec!["auth-001".to_string()],
        }
    ];

    let (status, confidence) = analyzer.suggest_status(&commits);
    assert_eq!(status, Some("review".to_string()));
    assert!(confidence > 0.5);
}

#[test]
fn test_suggest_status_work_in_progress() {
    let temp_repo = TestRepo::new().unwrap();
    let analyzer = GitAnalyzer::new(&temp_repo.repo_path).unwrap();

    let commits = vec![
        TaskCommit {
            oid: "abc123".to_string(),
            message: "WIP: implementing user login".to_string(),
            author: "test".to_string(),
            timestamp: Utc::now(),
            task_ids: vec!["auth-001".to_string()],
        }
    ];

    let (status, confidence) = analyzer.suggest_status(&commits);
    assert_eq!(status, Some("doing".to_string()));
    assert!(confidence > 0.5);
}

#[test]
fn test_suggest_status_no_commits() {
    let temp_repo = TestRepo::new().unwrap();
    let analyzer = GitAnalyzer::new(&temp_repo.repo_path).unwrap();

    let commits = vec![];
    let (status, confidence) = analyzer.suggest_status(&commits);
    assert_eq!(status, None);
    assert_eq!(confidence, 0.0);
}

#[test]
fn test_analyze_task_activity_with_commits() -> Result<()> {
    let test_repo = TestRepo::new()?;

    // Add commits with task references
    test_repo.add_commit("Initial setup for backend-001")?;
    test_repo.add_commit("Complete backend-001 implementation")?;
    test_repo.add_commit("Fix bug in frontend-002")?;

    let analyzer = GitAnalyzer::new(&test_repo.repo_path)?;
    let activities = analyzer.analyze_task_activity(Some(10))?;

    // Should find activities for both tasks
    assert_eq!(activities.len(), 2);

    // Check that backend-001 has 2 commits
    let backend_activity = activities.iter().find(|a| a.task_id == "backend-001").unwrap();
    assert_eq!(backend_activity.commits.len(), 2);
    assert!(backend_activity.last_activity.is_some());

    // Check that frontend-002 has 1 commit
    let frontend_activity = activities.iter().find(|a| a.task_id == "frontend-002").unwrap();
    assert_eq!(frontend_activity.commits.len(), 1);

    Ok(())
}

#[test]
fn test_analyze_task_activity_no_task_references() -> Result<()> {
    let test_repo = TestRepo::new()?;

    // Add commits without task references
    test_repo.add_commit("Regular commit without task reference")?;
    test_repo.add_commit("Another normal commit")?;

    let analyzer = GitAnalyzer::new(&test_repo.repo_path)?;
    let activities = analyzer.analyze_task_activity(Some(10))?;

    // Should find no task activities
    assert!(activities.is_empty());

    Ok(())
}

#[test]
fn test_get_repo_stats() -> Result<()> {
    let test_repo = TestRepo::new()?;

    // Add a commit to have some history
    test_repo.add_commit("Initial commit")?;

    let analyzer = GitAnalyzer::new(&test_repo.repo_path)?;
    let stats = analyzer.get_repo_stats()?;

    assert!(stats.contains_key("current_branch"));
    assert!(stats.contains_key("total_commits"));
    assert!(stats.contains_key("state"));

    // Check that we have at least 1 commit
    let commit_count: usize = stats.get("total_commits").unwrap().parse().unwrap();
    assert!(commit_count >= 1);

    Ok(())
}

#[test]
fn test_task_commit_ordering() -> Result<()> {
    let test_repo = TestRepo::new()?;

    // Add commits with timestamps that should be ordered
    test_repo.add_commit("First commit for backend-001")?;
    std::thread::sleep(std::time::Duration::from_millis(100)); // Ensure different timestamps
    test_repo.add_commit("Second commit for backend-001")?;

    let analyzer = GitAnalyzer::new(&test_repo.repo_path)?;
    let activities = analyzer.analyze_task_activity(Some(10))?;

    assert_eq!(activities.len(), 1);
    let activity = &activities[0];
    assert_eq!(activity.commits.len(), 2);

    // Commits should be ordered by most recent first (since we process them in reverse chronological order)
    assert!(activity.commits[0].timestamp >= activity.commits[1].timestamp);

    Ok(())
}