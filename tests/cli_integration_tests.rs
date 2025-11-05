use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;
use taskguard::task::{Task, TaskStatus, Priority};
use chrono::Utc;

/// Test fixture for CLI integration testing
struct CLITestProject {
    _temp_dir: TempDir,
    project_path: PathBuf,
    binary_path: PathBuf,
}

impl CLITestProject {
    fn new() -> Result<Self> {
        let temp_dir = tempfile::tempdir()?;
        let project_path = temp_dir.path().to_path_buf();

        // Find the TaskGuard binary using absolute path from CARGO_MANIFEST_DIR
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .unwrap_or_else(|_| ".".to_string());
        let manifest_path = PathBuf::from(manifest_dir);

        let binary_path = if let Ok(target_dir) = std::env::var("CARGO_TARGET_DIR") {
            PathBuf::from(target_dir).join("debug").join("taskguard")
        } else {
            manifest_path.join("target").join("debug").join("taskguard")
        };

        // If debug binary doesn't exist, try release
        let binary_path = if binary_path.exists() {
            binary_path
        } else {
            if let Ok(target_dir) = std::env::var("CARGO_TARGET_DIR") {
                PathBuf::from(target_dir).join("release").join("taskguard")
            } else {
                manifest_path.join("target").join("release").join("taskguard")
            }
        };

        Ok(CLITestProject {
            _temp_dir: temp_dir,
            project_path,
            binary_path,
        })
    }

    fn run_command(&self, args: &[&str]) -> Result<(String, String, i32)> {
        let output = Command::new(&self.binary_path)
            .args(args)
            .current_dir(&self.project_path)
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code().unwrap_or(-1);

        Ok((stdout, stderr, exit_code))
    }

    fn create_task_file(&self, area: &str, id: &str, title: &str, status: TaskStatus, dependencies: Vec<String>) -> Result<()> {
        let tasks_dir = self.project_path.join("tasks");
        let area_dir = tasks_dir.join(area);
        if !area_dir.exists() {
            fs::create_dir_all(&area_dir)?;
        }

        let file_path = area_dir.join(format!("{}.md", id));
        let task = Task {
            id: id.to_string(),
            title: title.to_string(),
            status,
            priority: Priority::Medium,
            tags: vec!["test".to_string()],
            dependencies,
            assignee: None,
            created: Utc::now(),
            estimate: Some("4h".to_string()),
            complexity: Some(5),
            area: area.to_string(),
            content: format!("Test content for {}", title),
            file_path: file_path.clone(),
        };

        task.save_to_file(&file_path)?;
        Ok(())
    }

    fn init_git_repo(&self) -> Result<()> {
        Command::new("git")
            .args(&["init"])
            .current_dir(&self.project_path)
            .output()?;

        Command::new("git")
            .args(&["config", "user.name", "Test User"])
            .current_dir(&self.project_path)
            .output()?;

        Command::new("git")
            .args(&["config", "user.email", "test@example.com"])
            .current_dir(&self.project_path)
            .output()?;

        Ok(())
    }

    fn add_git_commit(&self, message: &str) -> Result<()> {
        // Create a test file
        let file_path = self.project_path.join("test.txt");
        fs::write(&file_path, format!("content {}", chrono::Utc::now().timestamp()))?;

        Command::new("git")
            .args(&["add", "test.txt"])
            .current_dir(&self.project_path)
            .output()?;

        Command::new("git")
            .args(&["commit", "-m", message])
            .current_dir(&self.project_path)
            .output()?;

        Ok(())
    }
}

// =============================================================================
// BASIC CLI FUNCTIONALITY TESTS
// =============================================================================

#[test]
fn test_cli_binary_exists() {
    let project = CLITestProject::new().unwrap();
    assert!(project.binary_path.exists(), "TaskGuard binary should exist at {:?}", project.binary_path);
}

#[test]
fn test_help_command() -> Result<()> {
    let project = CLITestProject::new()?;
    let (stdout, _stderr, exit_code) = project.run_command(&["--help"])?;

    assert_eq!(exit_code, 0, "Help command should succeed");
    assert!(stdout.contains("taskguard"), "Help should mention taskguard");
    assert!(stdout.contains("USAGE") || stdout.contains("Usage"), "Help should show usage");

    Ok(())
}

#[test]
fn test_version_command() -> Result<()> {
    let project = CLITestProject::new()?;
    let (stdout, _stderr, exit_code) = project.run_command(&["--version"])?;

    assert_eq!(exit_code, 0, "Version command should succeed");
    assert!(stdout.contains("taskguard") || stdout.contains("0.1.0"), "Version should show app info");

    Ok(())
}

// =============================================================================
// INIT COMMAND TESTS
// =============================================================================

#[test]
fn test_init_command() -> Result<()> {
    let project = CLITestProject::new()?;
    let (stdout, stderr, exit_code) = project.run_command(&["init"])?;

    assert_eq!(exit_code, 0, "Init should succeed. stdout: {}, stderr: {}", stdout, stderr);
    assert!(stdout.contains("initialized") || stdout.contains("Created"), "Should show initialization message");

    // Check that directories were created
    assert!(project.project_path.join(".taskguard").exists(), "Should create .taskguard directory");
    assert!(project.project_path.join("tasks").exists(), "Should create tasks directory");
    assert!(project.project_path.join(".taskguard/config.toml").exists(), "Should create config file");

    Ok(())
}

#[test]
fn test_init_already_initialized() -> Result<()> {
    let project = CLITestProject::new()?;

    // First init
    let (_, _, exit_code) = project.run_command(&["init"])?;
    assert_eq!(exit_code, 0, "First init should succeed");

    // Second init
    let (stdout, _stderr, exit_code) = project.run_command(&["init"])?;
    // Should either succeed (idempotent) or fail gracefully
    assert!(exit_code == 0 || exit_code == 1, "Second init should handle already initialized state");

    if exit_code == 1 {
        assert!(stdout.contains("already") || stdout.contains("exists"), "Should indicate already initialized");
    }

    Ok(())
}

// =============================================================================
// LIST COMMAND TESTS
// =============================================================================

#[test]
fn test_list_empty_project() -> Result<()> {
    let project = CLITestProject::new()?;
    project.run_command(&["init"])?;

    let (stdout, _stderr, exit_code) = project.run_command(&["list"])?;

    assert_eq!(exit_code, 0, "List should succeed even with no tasks");
    assert!(stdout.contains("0") || stdout.contains("No tasks") || stdout.contains("empty"),
            "Should indicate no tasks found");

    Ok(())
}

#[test]
fn test_list_with_tasks() -> Result<()> {
    let project = CLITestProject::new()?;
    project.run_command(&["init"])?;

    // Create test tasks
    project.create_task_file("backend", "backend-001", "Test Backend Task", TaskStatus::Todo, vec![])?;
    project.create_task_file("frontend", "frontend-001", "Test Frontend Task", TaskStatus::Doing, vec![])?;

    let (stdout, _stderr, exit_code) = project.run_command(&["list"])?;

    assert_eq!(exit_code, 0, "List should succeed");
    assert!(stdout.contains("backend-001"), "Should show backend task");
    assert!(stdout.contains("frontend-001"), "Should show frontend task");
    assert!(stdout.contains("Test Backend Task"), "Should show task titles");

    Ok(())
}

#[test]
fn test_list_filter_by_area() -> Result<()> {
    let project = CLITestProject::new()?;
    project.run_command(&["init"])?;

    project.create_task_file("backend", "backend-001", "Backend Task", TaskStatus::Todo, vec![])?;
    project.create_task_file("frontend", "frontend-001", "Frontend Task", TaskStatus::Todo, vec![])?;

    let (stdout, _stderr, exit_code) = project.run_command(&["list", "--area", "backend"])?;

    assert_eq!(exit_code, 0, "List with area filter should succeed");
    assert!(stdout.contains("backend-001"), "Should show backend task");
    assert!(!stdout.contains("frontend-001"), "Should not show frontend task");

    Ok(())
}

#[test]
fn test_list_filter_by_status() -> Result<()> {
    let project = CLITestProject::new()?;
    project.run_command(&["init"])?;

    project.create_task_file("backend", "backend-001", "Todo Task", TaskStatus::Todo, vec![])?;
    project.create_task_file("backend", "backend-002", "Done Task", TaskStatus::Done, vec![])?;

    let (stdout, _stderr, exit_code) = project.run_command(&["list", "--status", "todo"])?;

    assert_eq!(exit_code, 0, "List with status filter should succeed");
    assert!(stdout.contains("backend-001"), "Should show todo task");
    assert!(!stdout.contains("backend-002"), "Should not show done task");

    Ok(())
}

// =============================================================================
// CREATE COMMAND TESTS
// =============================================================================

#[test]
fn test_create_basic_task() -> Result<()> {
    let project = CLITestProject::new()?;
    project.run_command(&["init"])?;

    let (stdout, _stderr, exit_code) = project.run_command(&[
        "create",
        "--title", "Test Task Creation",
        "--area", "backend",
        "--priority", "high"
    ])?;

    assert_eq!(exit_code, 0, "Create should succeed");
    assert!(stdout.contains("Created") || stdout.contains("backend"), "Should confirm task creation");

    // Verify task file was created
    let backend_dir = project.project_path.join("tasks/backend");
    assert!(backend_dir.exists(), "Backend directory should be created");

    let task_files: Vec<_> = fs::read_dir(&backend_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "md"))
        .collect();

    assert_eq!(task_files.len(), 1, "Should create exactly one task file");

    Ok(())
}

#[test]
fn test_create_without_init() -> Result<()> {
    let project = CLITestProject::new()?;

    let (stdout, stderr, exit_code) = project.run_command(&[
        "create",
        "--title", "Test Task",
        "--area", "backend"
    ])?;

    assert_ne!(exit_code, 0, "Create should fail without init");
    assert!(stdout.contains("init") || stderr.contains("init") ||
            stdout.contains("not in") || stderr.contains("not in"),
            "Should indicate need to run init first");

    Ok(())
}

#[test]
fn test_create_minimum_required_args() -> Result<()> {
    let project = CLITestProject::new()?;
    project.run_command(&["init"])?;

    let (stdout, _stderr, exit_code) = project.run_command(&[
        "create",
        "--title", "Minimum Task"
    ])?;

    assert_eq!(exit_code, 0, "Create with just title should succeed");
    assert!(stdout.contains("Created") || stdout.contains("setup"), "Should use default area");

    Ok(())
}

// =============================================================================
// VALIDATE COMMAND TESTS
// =============================================================================

#[test]
fn test_validate_empty_project() -> Result<()> {
    let project = CLITestProject::new()?;
    project.run_command(&["init"])?;

    let (stdout, _stderr, exit_code) = project.run_command(&["validate"])?;

    assert_eq!(exit_code, 0, "Validate should succeed on empty project");
    assert!(stdout.contains("0") || stdout.contains("No tasks") || stdout.contains("issues"),
            "Should indicate no tasks to validate");

    Ok(())
}

#[test]
fn test_validate_with_dependencies() -> Result<()> {
    let project = CLITestProject::new()?;
    project.run_command(&["init"])?;

    // Create tasks with dependencies
    project.create_task_file("setup", "setup-001", "Setup Task", TaskStatus::Done, vec![])?;
    project.create_task_file("backend", "backend-001", "Backend Task", TaskStatus::Todo, vec!["setup-001".to_string()])?;
    project.create_task_file("frontend", "frontend-001", "Frontend Task", TaskStatus::Todo, vec!["backend-001".to_string()])?;

    let (stdout, _stderr, exit_code) = project.run_command(&["validate"])?;

    assert_eq!(exit_code, 0, "Validate should succeed");
    assert!(stdout.contains("backend-001"), "Should show available task");
    assert!(stdout.contains("frontend-001"), "Should show blocked task");
    assert!(stdout.contains("Available") || stdout.contains("Blocked") || stdout.contains("dependencies"),
            "Should show dependency analysis");

    Ok(())
}

#[test]
fn test_validate_circular_dependencies() -> Result<()> {
    let project = CLITestProject::new()?;
    project.run_command(&["init"])?;

    // Create circular dependency: A -> B -> A
    project.create_task_file("backend", "backend-001", "Task A", TaskStatus::Todo, vec!["backend-002".to_string()])?;
    project.create_task_file("backend", "backend-002", "Task B", TaskStatus::Todo, vec!["backend-001".to_string()])?;

    let (stdout, stderr, exit_code) = project.run_command(&["validate"])?;

    // Should detect the circular dependency
    assert!(stdout.contains("circular") || stderr.contains("circular") ||
            stdout.contains("cycle") || stderr.contains("cycle") ||
            exit_code != 0, "Should detect circular dependencies");

    Ok(())
}

// =============================================================================
// SYNC COMMAND TESTS (GIT INTEGRATION)
// =============================================================================

#[test]
fn test_sync_without_git() -> Result<()> {
    let project = CLITestProject::new()?;
    project.run_command(&["init"])?;

    let (stdout, stderr, exit_code) = project.run_command(&["sync"])?;

    assert_ne!(exit_code, 0, "Sync should fail without Git repository");
    assert!(stdout.contains("Git") || stderr.contains("Git") ||
            stdout.contains("repository") || stderr.contains("repository"),
            "Should indicate Git repository required");

    Ok(())
}

#[test]
fn test_sync_with_git_no_commits() -> Result<()> {
    let project = CLITestProject::new()?;
    project.run_command(&["init"])?;
    project.init_git_repo()?;

    let (stdout, _stderr, exit_code) = project.run_command(&["sync"])?;

    // Should handle empty Git repository gracefully
    assert_eq!(exit_code, 0, "Sync should handle empty Git repo");
    assert!(stdout.contains("No") || stdout.contains("0") || stdout.contains("activity"),
            "Should indicate no activity found");

    Ok(())
}

#[test]
fn test_sync_with_task_commits() -> Result<()> {
    let project = CLITestProject::new()?;
    project.run_command(&["init"])?;
    project.init_git_repo()?;

    // Create task and commits
    project.create_task_file("backend", "backend-001", "Backend Task", TaskStatus::Todo, vec![])?;
    project.add_git_commit("Start work on backend-001")?;
    project.add_git_commit("Complete backend-001 implementation")?;

    let (stdout, _stderr, exit_code) = project.run_command(&["sync"])?;

    assert_eq!(exit_code, 0, "Sync should succeed");
    assert!(stdout.contains("backend-001"), "Should find task activity");
    assert!(stdout.contains("2") || stdout.contains("commits"), "Should show commit count");

    Ok(())
}

#[test]
fn test_sync_verbose_mode() -> Result<()> {
    let project = CLITestProject::new()?;
    project.run_command(&["init"])?;
    project.init_git_repo()?;

    project.create_task_file("backend", "backend-001", "Test Task", TaskStatus::Todo, vec![])?;
    project.add_git_commit("Work on backend-001")?;

    let (stdout, _stderr, exit_code) = project.run_command(&["sync", "--verbose"])?;

    assert_eq!(exit_code, 0, "Sync verbose should succeed");
    assert!(stdout.contains("backend-001"), "Should show task activity");
    // Verbose mode should show more details
    assert!(stdout.len() > 100, "Verbose output should be substantial");

    Ok(())
}

// =============================================================================
// LINT COMMAND TESTS
// =============================================================================

#[test]
fn test_lint_empty_project() -> Result<()> {
    let project = CLITestProject::new()?;
    project.run_command(&["init"])?;

    let (stdout, _stderr, exit_code) = project.run_command(&["lint"])?;

    assert_eq!(exit_code, 0, "Lint should succeed on empty project");
    assert!(stdout.contains("0") || stdout.contains("No tasks") || stdout.contains("analyzed"),
            "Should handle empty project");

    Ok(())
}

#[test]
fn test_lint_with_tasks() -> Result<()> {
    let project = CLITestProject::new()?;
    project.run_command(&["init"])?;

    project.create_task_file("backend", "backend-001", "Good Task", TaskStatus::Todo, vec![])?;

    let (stdout, _stderr, exit_code) = project.run_command(&["lint"])?;

    assert_eq!(exit_code, 0, "Lint should succeed");
    assert!(stdout.contains("analyzed") || stdout.contains("complexity") || stdout.contains("quality"),
            "Should show analysis results");
    assert!(stdout.contains("backend-001"), "Should mention the task");

    Ok(())
}

#[test]
fn test_lint_area_filter() -> Result<()> {
    let project = CLITestProject::new()?;
    project.run_command(&["init"])?;

    project.create_task_file("backend", "backend-001", "Backend Task", TaskStatus::Todo, vec![])?;
    project.create_task_file("frontend", "frontend-001", "Frontend Task", TaskStatus::Todo, vec![])?;

    let (stdout, _stderr, exit_code) = project.run_command(&["lint", "--area", "backend"])?;

    assert_eq!(exit_code, 0, "Lint with area filter should succeed");
    assert!(stdout.contains("backend-001"), "Should include backend task");
    assert!(!stdout.contains("frontend-001"), "Should exclude frontend task");

    Ok(())
}

#[test]
fn test_lint_verbose_mode() -> Result<()> {
    let project = CLITestProject::new()?;
    project.run_command(&["init"])?;

    project.create_task_file("backend", "backend-001", "Test Task", TaskStatus::Todo, vec![])?;

    let (stdout, _stderr, exit_code) = project.run_command(&["lint", "--verbose"])?;

    assert_eq!(exit_code, 0, "Lint verbose should succeed");
    assert!(stdout.contains("backend-001"), "Should analyze the task");
    // Verbose should show more details
    assert!(stdout.len() > 50, "Verbose output should be detailed");

    Ok(())
}

// =============================================================================
// AI COMMAND TESTS
// =============================================================================

#[test]
fn test_ai_basic_interaction() -> Result<()> {
    let project = CLITestProject::new()?;
    project.run_command(&["init"])?;

    let (stdout, _stderr, exit_code) = project.run_command(&["ai", "What should I work on next?"])?;

    assert_eq!(exit_code, 0, "AI command should succeed");
    assert!(stdout.contains("tasks") || stdout.contains("No tasks") || stdout.contains("available"),
            "Should provide meaningful response");
    assert!(!stdout.is_empty(), "Should provide some output");

    Ok(())
}

#[test]
fn test_ai_task_creation_guidance() -> Result<()> {
    let project = CLITestProject::new()?;
    project.run_command(&["init"])?;

    let (stdout, _stderr, exit_code) = project.run_command(&["ai", "Create a task for user authentication"])?;

    assert_eq!(exit_code, 0, "AI task creation should succeed");
    assert!(stdout.contains("create") || stdout.contains("taskguard"), "Should provide creation guidance");
    assert!(stdout.contains("authentication") || stdout.contains("auth"), "Should reference the topic");

    Ok(())
}

#[test]
fn test_ai_status_inquiry() -> Result<()> {
    let project = CLITestProject::new()?;
    project.run_command(&["init"])?;

    project.create_task_file("backend", "backend-001", "Test Task", TaskStatus::Todo, vec![])?;

    let (stdout, _stderr, exit_code) = project.run_command(&["ai", "What's the project status?"])?;

    assert_eq!(exit_code, 0, "AI status inquiry should succeed");
    assert!(stdout.contains("status") || stdout.contains("tasks") || stdout.contains("1"),
            "Should provide status information");
    assert!(stdout.contains("backend-001"), "Should mention existing task");

    Ok(())
}

#[test]
fn test_ai_empty_input() -> Result<()> {
    let project = CLITestProject::new()?;
    project.run_command(&["init"])?;

    let (stdout, _stderr, exit_code) = project.run_command(&["ai", ""])?;

    assert_eq!(exit_code, 0, "AI should handle empty input gracefully");
    assert!(!stdout.is_empty(), "Should provide some guidance for empty input");
    assert!(stdout.contains("help") || stdout.contains("guidance") || stdout.contains("assist"),
            "Should offer helpful guidance");

    Ok(())
}

// =============================================================================
// ERROR HANDLING AND EDGE CASES
// =============================================================================

#[test]
fn test_invalid_command() -> Result<()> {
    let project = CLITestProject::new()?;

    let (stdout, stderr, exit_code) = project.run_command(&["nonexistent-command"])?;

    assert_ne!(exit_code, 0, "Invalid command should fail");
    assert!(stdout.contains("help") || stderr.contains("help") ||
            stdout.contains("usage") || stderr.contains("usage") ||
            stdout.contains("subcommand") || stderr.contains("subcommand"),
            "Should provide helpful error message");

    Ok(())
}

#[test]
fn test_missing_required_args() -> Result<()> {
    let project = CLITestProject::new()?;
    project.run_command(&["init"])?;

    let (stdout, stderr, exit_code) = project.run_command(&["create"])?;

    assert_ne!(exit_code, 0, "Create without required args should fail");
    assert!(stdout.contains("required") || stderr.contains("required") ||
            stdout.contains("title") || stderr.contains("title"),
            "Should indicate missing required arguments");

    Ok(())
}

#[test]
fn test_invalid_area_filter() -> Result<()> {
    let project = CLITestProject::new()?;
    project.run_command(&["init"])?;

    let (stdout, _stderr, exit_code) = project.run_command(&["list", "--area", "nonexistent"])?;

    assert_eq!(exit_code, 0, "List with invalid area should succeed but show no results");
    assert!(stdout.contains("0") || stdout.contains("No tasks") || stdout.contains("empty"),
            "Should show no tasks for invalid area");

    Ok(())
}

// =============================================================================
// PERFORMANCE AND STRESS TESTS
// =============================================================================

#[test]
fn test_large_project_performance() -> Result<()> {
    let project = CLITestProject::new()?;
    project.run_command(&["init"])?;

    // Create many tasks to test performance
    for i in 1..=50 {
        project.create_task_file("backend", &format!("backend-{:03}", i), &format!("Task {}", i), TaskStatus::Todo, vec![])?;
    }

    let start = std::time::Instant::now();
    let (stdout, _stderr, exit_code) = project.run_command(&["list"])?;
    let list_duration = start.elapsed();

    assert_eq!(exit_code, 0, "List should succeed with many tasks");
    assert!(stdout.contains("50") || stdout.contains("backend-050"), "Should show all tasks");
    assert!(list_duration < std::time::Duration::from_secs(5), "List should complete within 5 seconds");

    let start = std::time::Instant::now();
    let (_stdout, _stderr, exit_code) = project.run_command(&["validate"])?;
    let validate_duration = start.elapsed();

    assert_eq!(exit_code, 0, "Validate should succeed with many tasks");
    assert!(validate_duration < std::time::Duration::from_secs(10), "Validate should complete within 10 seconds");

    Ok(())
}

#[test]
fn test_command_timeout_handling() -> Result<()> {
    let project = CLITestProject::new()?;
    project.run_command(&["init"])?;

    // Test that commands complete within reasonable time
    let commands = vec![
        vec!["list"],
        vec!["validate"],
        vec!["ai", "help"],
    ];

    for cmd in commands {
        let start = std::time::Instant::now();
        let (_stdout, _stderr, _exit_code) = project.run_command(&cmd)?;
        let duration = start.elapsed();

        assert!(duration < std::time::Duration::from_secs(30),
                "Command {:?} should complete within 30 seconds", cmd);
    }

    Ok(())
}