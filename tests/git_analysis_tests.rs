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

// =============================================================================
// COMPREHENSIVE GIT ANALYSIS TESTS FOR PHASE 2-3 FEATURES
// =============================================================================

#[test]
fn test_complex_commit_message_patterns() -> Result<()> {
    let temp_repo = TestRepo::new()?;
    let analyzer = GitAnalyzer::new(&temp_repo.repo_path)?;

    let complex_patterns = vec![
        ("feat(backend-001): add user authentication endpoints", vec!["backend-001"]),
        ("fix: resolve issue in frontend-002 and backend-003", vec!["frontend-002", "backend-003"]),
        ("docs: update API documentation for auth-001", vec!["auth-001"]),
        ("refactor(setup-001): improve database configuration", vec!["setup-001"]),
        ("test: add unit tests for backend-001 and integration tests for api-002", vec!["backend-001", "api-002"]),
        ("chore: update dependencies, affects backend-001, frontend-002", vec!["backend-001", "frontend-002"]),
        ("Merge pull request #123: Feature backend-001 user auth", vec!["backend-001"]),
        ("WIP: working on frontend-001, backend-002, and testing-003", vec!["frontend-001", "backend-002", "testing-003"]),
        ("hotfix: critical bug in backend-001 affecting auth-002", vec!["backend-001", "auth-002"]),
        ("release: v1.2.0 - includes backend-001, frontend-001, api-001", vec!["backend-001", "frontend-001", "api-001"]),
    ];

    for (message, expected_ids) in complex_patterns {
        let ids = analyzer.extract_task_ids(message);
        println!("Testing: '{}' -> {:?}", message, ids);

        for expected_id in expected_ids {
            assert!(ids.contains(&expected_id.to_string()),
                "Should extract '{}' from message: '{}'", expected_id, message);
        }
    }

    Ok(())
}

#[test]
fn test_status_suggestion_comprehensive_patterns() -> Result<()> {
    let temp_repo = TestRepo::new()?;
    let analyzer = GitAnalyzer::new(&temp_repo.repo_path)?;

    let test_scenarios = vec![
        // Completion patterns
        (vec!["Complete backend-001 implementation", "Finish backend-001 feature", "Done with backend-001"], "done", 0.8),
        (vec!["Implement backend-001", "Add backend-001 functionality", "Complete backend-001"], "done", 0.7),
        (vec!["Fix backend-001", "Resolve backend-001 issue", "Complete backend-001 bugfix"], "done", 0.7),

        // Testing/Review patterns
        (vec!["Add tests for backend-001", "Test backend-001 functionality", "Fix tests for backend-001"], "review", 0.6),
        (vec!["Code review for backend-001", "Review backend-001 implementation", "Refactor backend-001"], "review", 0.6),

        // Work in progress patterns
        (vec!["WIP: backend-001 implementation", "Start backend-001", "Initial backend-001 work"], "doing", 0.7),
        (vec!["Working on backend-001", "Continue backend-001", "Progress on backend-001"], "doing", 0.6),

        // Documentation patterns
        (vec!["Document backend-001", "Add docs for backend-001", "Update backend-001 documentation"], "review", 0.5),
    ];

    for (messages, expected_status, min_confidence) in test_scenarios {
        let commits: Vec<TaskCommit> = messages.iter().enumerate().map(|(i, msg)| {
            TaskCommit {
                oid: format!("commit_{}", i),
                message: msg.to_string(),
                author: "test".to_string(),
                timestamp: Utc::now(),
                task_ids: vec!["backend-001".to_string()],
            }
        }).collect();

        let (status, confidence) = analyzer.suggest_status(&commits);

        assert_eq!(status, Some(expected_status.to_string()),
            "Should suggest '{}' status for messages: {:?}", expected_status, messages);
        assert!(confidence >= min_confidence,
            "Confidence should be at least {} for messages: {:?}, got {}", min_confidence, messages, confidence);
    }

    Ok(())
}

#[test]
fn test_git_analyzer_branch_analysis() -> Result<()> {
    let test_repo = TestRepo::new()?;

    // Create commits on main branch
    test_repo.add_commit("Initial commit")?;
    test_repo.add_commit("Start backend-001 feature")?;

    // Create feature branch
    let head = test_repo.repo.head()?.target().unwrap();
    let commit = test_repo.repo.find_commit(head)?;
    let branch = test_repo.repo.branch("feature/backend-001", &commit, false)?;
    test_repo.repo.set_head(branch.get().name().unwrap())?;
    test_repo.repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;

    // Add commits to feature branch
    test_repo.add_commit("Implement backend-001 core functionality")?;
    test_repo.add_commit("Add tests for backend-001")?;
    test_repo.add_commit("Complete backend-001 implementation")?;

    let analyzer = GitAnalyzer::new(&test_repo.repo_path)?;
    let activities = analyzer.analyze_task_activity(Some(10))?;

    assert!(!activities.is_empty(), "Should find task activities");

    let backend_activity = activities.iter().find(|a| a.task_id == "backend-001");
    assert!(backend_activity.is_some(), "Should find backend-001 activity");

    let activity = backend_activity.unwrap();
    assert!(activity.commits.len() >= 3, "Should find commits from feature branch");

    Ok(())
}

#[test]
fn test_commit_timestamp_analysis() -> Result<()> {
    let test_repo = TestRepo::new()?;

    // Add commits with different intervals
    test_repo.add_commit("Start backend-001")?;
    std::thread::sleep(std::time::Duration::from_millis(100));

    test_repo.add_commit("Continue backend-001")?;
    std::thread::sleep(std::time::Duration::from_millis(100));

    test_repo.add_commit("Complete backend-001")?;

    let analyzer = GitAnalyzer::new(&test_repo.repo_path)?;
    let activities = analyzer.analyze_task_activity(Some(10))?;

    assert_eq!(activities.len(), 1, "Should find one task activity");

    let activity = &activities[0];
    assert_eq!(activity.task_id, "backend-001");
    assert_eq!(activity.commits.len(), 3, "Should have 3 commits");
    assert!(activity.last_activity.is_some(), "Should have last activity timestamp");

    // Commits should be ordered by timestamp (most recent first)
    for i in 0..activity.commits.len()-1 {
        assert!(activity.commits[i].timestamp >= activity.commits[i+1].timestamp,
            "Commits should be ordered by timestamp (newest first)");
    }

    Ok(())
}

#[test]
fn test_multiple_authors_analysis() -> Result<()> {
    let test_repo = TestRepo::new()?;

    // Configure different authors for different commits
    test_repo.add_commit("Initial work on backend-001 by Alice")?;

    // Change author
    let mut config = test_repo.repo.config()?;
    config.set_str("user.name", "Bob Developer")?;
    config.set_str("user.email", "bob@example.com")?;

    test_repo.add_commit("Continue backend-001 by Bob")?;

    // Change author again
    config.set_str("user.name", "Charlie Tester")?;
    config.set_str("user.email", "charlie@example.com")?;

    test_repo.add_commit("Test backend-001 by Charlie")?;

    let analyzer = GitAnalyzer::new(&test_repo.repo_path)?;
    let activities = analyzer.analyze_task_activity(Some(10))?;

    assert_eq!(activities.len(), 1, "Should consolidate all commits under one task");

    let activity = &activities[0];
    assert_eq!(activity.commits.len(), 3, "Should have commits from all authors");

    // Check that different authors are represented
    let authors: std::collections::HashSet<_> = activity.commits.iter()
        .map(|c| &c.author)
        .collect();
    assert!(authors.len() > 1, "Should have multiple authors: {:?}", authors);

    Ok(())
}

#[test]
fn test_git_repository_statistics() -> Result<()> {
    let test_repo = TestRepo::new()?;

    // Add some commits
    test_repo.add_commit("Initial commit")?;
    test_repo.add_commit("Feature work on backend-001")?;
    test_repo.add_commit("Bugfix for frontend-002")?;

    let analyzer = GitAnalyzer::new(&test_repo.repo_path)?;
    let stats = analyzer.get_repo_stats()?;

    // Check expected statistics
    assert!(stats.contains_key("current_branch"), "Should include current branch");
    assert!(stats.contains_key("total_commits"), "Should include total commits");
    assert!(stats.contains_key("state"), "Should include repository state");

    let total_commits: usize = stats.get("total_commits").unwrap().parse()?;
    assert!(total_commits >= 3, "Should count all commits");

    let current_branch = stats.get("current_branch").unwrap();
    assert!(current_branch.contains("main") || current_branch.contains("master"),
        "Should be on main/master branch");

    Ok(())
}

#[test]
fn test_large_repository_performance() -> Result<()> {
    let test_repo = TestRepo::new()?;

    // Create many commits to test performance
    for i in 1..=100 {
        let message = if i % 10 == 0 {
            format!("Complete milestone {} for backend-{:03}", i/10, i)
        } else {
            format!("Work on backend-{:03}", i)
        };
        test_repo.add_commit(&message)?;
    }

    let analyzer = GitAnalyzer::new(&test_repo.repo_path)?;

    let start = std::time::Instant::now();
    let activities = analyzer.analyze_task_activity(Some(200))?; // Analyze more than we have
    let duration = start.elapsed();

    assert!(duration < std::time::Duration::from_secs(5),
        "Large repository analysis should complete within 5 seconds, took {:?}", duration);

    assert!(!activities.is_empty(), "Should find task activities");
    assert!(activities.len() <= 100, "Should not have more activities than task IDs");

    Ok(())
}

#[test]
fn test_commit_message_edge_cases() -> Result<()> {
    let temp_repo = TestRepo::new()?;
    let analyzer = GitAnalyzer::new(&temp_repo.repo_path)?;

    let edge_cases = vec![
        // Unicode and special characters
        ("🚀 Deploy backend-001 to production 🎉", vec!["backend-001"]),
        ("Fix backend-001: résoudre le problème d'authentification", vec!["backend-001"]),
        ("Update backend-001 配置文件", vec!["backend-001"]),

        // Multiple references with noise
        ("Really long commit message that talks about backend-001 and also mentions frontend-002 in passing while discussing the overall architecture", vec!["backend-001", "frontend-002"]),

        // Case variations
        ("Fix BACKEND-001 and Backend-002", vec!["BACKEND-001", "Backend-002"]),

        // Punctuation handling
        ("backend-001: fix auth, frontend-002: update UI, testing-003: add tests", vec!["backend-001", "frontend-002", "testing-003"]),

        // URLs and file paths
        ("Update backend-001 in /src/backend/auth.rs", vec!["backend-001"]),
        ("See https://github.com/user/repo/issues/backend-001", vec!["backend-001"]),

        // False positives to avoid
        ("Create backup-001 directory", vec![]), // Should not match
        ("Update version to 1.2.3-backend-001", vec![]), // Should not match in version strings
    ];

    for (message, expected_ids) in edge_cases {
        let ids = analyzer.extract_task_ids(message);
        println!("Testing edge case: '{}' -> {:?}", message, ids);

        if expected_ids.is_empty() {
            assert!(ids.is_empty() || !ids.iter().any(|id| id.contains("backend") || id.contains("frontend")),
                "Should not extract false positives from: '{}'", message);
        } else {
            for expected_id in expected_ids {
                assert!(ids.contains(&expected_id.to_string()),
                    "Should extract '{}' from edge case: '{}'", expected_id, message);
            }
        }
    }

    Ok(())
}

#[test]
fn test_merge_commit_analysis() -> Result<()> {
    let test_repo = TestRepo::new()?;

    // Create main branch commits
    test_repo.add_commit("Initial commit")?;

    // Create a feature branch (simulate by just adding commits with merge-like messages)
    test_repo.add_commit("Feature: implement backend-001 authentication")?;
    test_repo.add_commit("Fix: resolve backend-001 validation issue")?;
    test_repo.add_commit("Merge pull request #42 from feature/backend-001")?;
    test_repo.add_commit("Merge branch 'feature/backend-001' into main")?;

    let analyzer = GitAnalyzer::new(&test_repo.repo_path)?;
    let activities = analyzer.analyze_task_activity(Some(10))?;

    assert!(!activities.is_empty(), "Should find task activities");

    let backend_activity = activities.iter().find(|a| a.task_id == "backend-001");
    assert!(backend_activity.is_some(), "Should find backend-001 activity");

    let activity = backend_activity.unwrap();
    assert!(activity.commits.len() >= 2, "Should include both feature and merge commits");

    Ok(())
}

#[test]
fn test_concurrent_git_access_safety() -> Result<()> {
    let test_repo = TestRepo::new()?;

    // Add initial commits
    for i in 1..=10 {
        test_repo.add_commit(&format!("Commit {} for backend-001", i))?;
    }

    let repo_path = test_repo.repo_path.clone();

    // Test concurrent access to the repository
    let handles: Vec<_> = (0..5).map(|thread_id| {
        let path = repo_path.clone();
        std::thread::spawn(move || -> Result<usize> {
            let analyzer = GitAnalyzer::new(&path)?;
            let activities = analyzer.analyze_task_activity(Some(20))?;
            println!("Thread {} analyzed {} activities", thread_id, activities.len());
            Ok(activities.len())
        })
    }).collect();

    // Collect results
    let results: Vec<usize> = handles.into_iter()
        .map(|h| h.join().unwrap().unwrap())
        .collect();

    // All threads should get the same results
    let first_result = results[0];
    for result in &results {
        assert_eq!(*result, first_result, "Concurrent access should give consistent results");
    }

    assert!(first_result > 0, "Should find task activities");

    Ok(())
}