use anyhow::Result;
use git2::Repository;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;
use taskguard::commands::{init, create, validate, sync, lint, ai};
use taskguard::task::{Task, TaskStatus, Priority};
use chrono::Utc;

/// Test fixture for end-to-end TaskGuard workflows
struct TaskGuardTestProject {
    _temp_dir: TempDir,
    project_path: PathBuf,
    tasks_dir: PathBuf,
    taskguard_dir: PathBuf,
}

impl TaskGuardTestProject {
    fn new() -> Result<Self> {
        let temp_dir = tempfile::tempdir()?;
        let project_path = temp_dir.path().to_path_buf();
        let tasks_dir = project_path.join("tasks");
        let taskguard_dir = project_path.join(".taskguard");

        Ok(TaskGuardTestProject {
            _temp_dir: temp_dir,
            project_path,
            tasks_dir,
            taskguard_dir,
        })
    }

    fn set_current_dir(&self) -> Result<()> {
        std::env::set_current_dir(&self.project_path)?;
        Ok(())
    }

    fn init_git_repo(&self) -> Result<Repository> {
        let repo = Repository::init(&self.project_path)?;
        let mut config = repo.config()?;
        config.set_str("user.name", "Test User")?;
        config.set_str("user.email", "test@example.com")?;
        Ok(repo)
    }

    fn add_git_commit(&self, repo: &Repository, message: &str) -> Result<()> {
        // Create a test file
        let file_path = self.project_path.join("test.txt");
        fs::write(&file_path, format!("content {}", chrono::Utc::now().timestamp()))?;

        // Stage the file
        let mut index = repo.index()?;
        index.add_path(std::path::Path::new("test.txt"))?;
        index.write()?;

        // Create commit
        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;
        let signature = repo.signature()?;

        let parent_commit = if let Ok(head) = repo.head() {
            Some(head.peel_to_commit()?)
        } else {
            None
        };

        let parents: Vec<&git2::Commit> = if let Some(ref parent) = parent_commit {
            vec![parent]
        } else {
            vec![]
        };

        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &parents,
        )?;

        Ok(())
    }

    fn create_task_manually(&self, area: &str, id: &str, title: &str, status: TaskStatus, dependencies: Vec<String>) -> Result<()> {
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
        };

        let area_dir = self.tasks_dir.join(area);
        if !area_dir.exists() {
            fs::create_dir_all(&area_dir)?;
        }

        let file_path = area_dir.join(format!("{}.md", id));
        task.save_to_file(&file_path)?;
        Ok(())
    }
}

// =============================================================================
// COMPLETE PROJECT LIFECYCLE TESTS
// =============================================================================

#[test]
fn test_complete_project_lifecycle() -> Result<()> {
    let project = TaskGuardTestProject::new()?;
    project.set_current_dir()?;

    // 1. Initialize project
    init::run()?;
    assert!(project.taskguard_dir.exists(), "Should create .taskguard directory");
    assert!(project.tasks_dir.exists(), "Should create tasks directory");

    // 2. Create initial tasks
    create::run("Setup development environment".to_string(), Some("setup".to_string()), Some("high".to_string()))?;
    create::run("Implement user authentication".to_string(), Some("backend".to_string()), Some("medium".to_string()))?;
    create::run("Build login form".to_string(), Some("frontend".to_string()), Some("medium".to_string()))?;

    // 3. Validate initial state
    validate::run()?;

    // 4. Simulate work progress with Git commits
    let repo = project.init_git_repo()?;
    project.add_git_commit(&repo, "Initial project setup")?;
    project.add_git_commit(&repo, "Start working on setup-001")?;
    project.add_git_commit(&repo, "Complete setup-001 configuration")?;

    // 5. Run sync to analyze Git activity
    sync::run(50, false, false, false)?;

    // 6. Run lint to analyze task quality
    lint::run(false, None)?;

    // 7. Use AI to get recommendations
    ai::run("What should I work on next?".to_string())?;

    Ok(())
}

#[test]
fn test_dependency_workflow() -> Result<()> {
    let project = TaskGuardTestProject::new()?;
    project.set_current_dir()?;

    // Initialize project
    init::run()?;

    // Create tasks with dependency chain
    project.create_task_manually("setup", "setup-001", "Environment Setup", TaskStatus::Todo, vec![])?;
    project.create_task_manually("backend", "backend-001", "Database Schema", TaskStatus::Todo, vec!["setup-001".to_string()])?;
    project.create_task_manually("backend", "backend-002", "User API", TaskStatus::Todo, vec!["backend-001".to_string()])?;
    project.create_task_manually("frontend", "frontend-001", "User Interface", TaskStatus::Todo, vec!["backend-002".to_string()])?;

    // Initial validation should show only setup-001 as available
    validate::run()?;

    // Complete setup-001
    let setup_task_path = project.tasks_dir.join("setup").join("setup-001.md");
    let mut setup_task = Task::from_file(&setup_task_path)?;
    setup_task.status = TaskStatus::Done;
    setup_task.save_to_file(&setup_task_path)?;

    // Now backend-001 should be available
    validate::run()?;

    // AI should recommend backend-001
    ai::run("What should I work on next?".to_string())?;

    Ok(())
}

// =============================================================================
// GIT INTEGRATION WORKFLOW TESTS
// =============================================================================

#[test]
fn test_git_analysis_workflow() -> Result<()> {
    let project = TaskGuardTestProject::new()?;
    project.set_current_dir()?;

    // Initialize project and Git
    init::run()?;
    let repo = project.init_git_repo()?;

    // Create tasks
    create::run("Fix authentication bug".to_string(), Some("backend".to_string()), Some("high".to_string()))?;
    create::run("Add user registration".to_string(), Some("backend".to_string()), Some("medium".to_string()))?;

    // Simulate development workflow with Git commits
    project.add_git_commit(&repo, "Start working on backend-001 authentication fix")?;
    std::thread::sleep(std::time::Duration::from_millis(100));
    project.add_git_commit(&repo, "WIP: implementing auth validation in backend-001")?;
    std::thread::sleep(std::time::Duration::from_millis(100));
    project.add_git_commit(&repo, "Fix tests for backend-001 authentication")?;
    std::thread::sleep(std::time::Duration::from_millis(100));
    project.add_git_commit(&repo, "Complete backend-001 authentication feature")?;

    // Run sync to analyze Git activity
    sync::run(10, true, false, false)?; // Verbose mode

    // Git analysis should suggest status changes
    Ok(())
}

#[test]
fn test_complex_git_scenario() -> Result<()> {
    let project = TaskGuardTestProject::new()?;
    project.set_current_dir()?;

    init::run()?;
    let repo = project.init_git_repo()?;

    // Create multiple tasks
    create::run("Database migration".to_string(), Some("backend".to_string()), Some("high".to_string()))?;
    create::run("API endpoints".to_string(), Some("backend".to_string()), Some("medium".to_string()))?;
    create::run("Frontend components".to_string(), Some("frontend".to_string()), Some("medium".to_string()))?;

    // Simulate complex development with multiple tasks
    let commit_scenarios = vec![
        "Initial work on backend-001 database schema",
        "Continue backend-001 migration scripts",
        "Start frontend-001 component structure",
        "Fix bug in backend-001 migration",
        "Complete backend-001 database migration",
        "WIP: backend-002 API routing",
        "Tests for frontend-001 components",
        "Finish frontend-001 user interface",
        "Code review fixes for backend-002",
        "Complete backend-002 API implementation",
    ];

    for commit_msg in commit_scenarios {
        project.add_git_commit(&repo, commit_msg)?;
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    // Analyze the complex Git history
    sync::run(20, true, false, false)?;

    Ok(())
}

// =============================================================================
// TASK QUALITY AND COMPLEXITY WORKFLOW TESTS
// =============================================================================

#[test]
fn test_task_quality_improvement_workflow() -> Result<()> {
    let project = TaskGuardTestProject::new()?;
    project.set_current_dir()?;

    init::run()?;

    // Create tasks with varying quality levels

    // High quality task
    let high_quality_content = r#"
## Context
This task implements a comprehensive user authentication system using JWT tokens.

## Objectives
- Secure user login and logout functionality
- Token-based session management
- Password hashing and validation

## Tasks
- [ ] Install JWT and bcrypt dependencies
- [ ] Create authentication middleware
- [ ] Implement login endpoint
- [ ] Add logout functionality
- [ ] Write comprehensive tests

## Acceptance Criteria
✅ Users can log in with email/password
✅ JWT tokens are generated and validated
✅ Passwords are securely hashed
✅ Session management works correctly
"#;

    let high_quality_task = Task {
        id: "backend-001".to_string(),
        title: "Implement User Authentication System".to_string(),
        status: TaskStatus::Todo,
        priority: Priority::High,
        tags: vec!["auth".to_string(), "security".to_string()],
        dependencies: vec![],
        assignee: Some("developer".to_string()),
        created: Utc::now(),
        estimate: Some("8 hours".to_string()),
        complexity: Some(6),
        area: "backend".to_string(),
        content: high_quality_content.to_string(),
    };

    let backend_dir = project.tasks_dir.join("backend");
    fs::create_dir_all(&backend_dir)?;
    high_quality_task.save_to_file(&backend_dir.join("backend-001.md"))?;

    // Low quality task
    let low_quality_task = Task {
        id: "frontend-001".to_string(),
        title: "UI stuff".to_string(),
        status: TaskStatus::Todo,
        priority: Priority::Medium,
        tags: vec![],
        dependencies: vec![],
        assignee: None,
        created: Utc::now(),
        estimate: None,
        complexity: None,
        area: "frontend".to_string(),
        content: "Make the UI better.".to_string(),
    };

    let frontend_dir = project.tasks_dir.join("frontend");
    fs::create_dir_all(&frontend_dir)?;
    low_quality_task.save_to_file(&frontend_dir.join("frontend-001.md"))?;

    // Run lint analysis
    lint::run(true, None)?; // Verbose mode

    // Should identify quality issues
    Ok(())
}

#[test]
fn test_complexity_analysis_workflow() -> Result<()> {
    let project = TaskGuardTestProject::new()?;
    project.set_current_dir()?;

    init::run()?;

    // Create tasks with different complexity levels
    let simple_task = Task {
        id: "simple-001".to_string(),
        title: "Update README".to_string(),
        status: TaskStatus::Todo,
        priority: Priority::Low,
        tags: vec!["docs".to_string()],
        dependencies: vec![],
        assignee: None,
        created: Utc::now(),
        estimate: Some("30 minutes".to_string()),
        complexity: Some(2),
        area: "docs".to_string(),
        content: "Update the project README with new installation instructions.".to_string(),
    };

    let complex_task_content = format!(
        "{}{}{}",
        "A very complex task requiring extensive planning and implementation. ".repeat(100),
        "\n\n## Tasks\n",
        "- [ ] Subtask\n".repeat(30)
    );

    let complex_task = Task {
        id: "complex-001".to_string(),
        title: "Redesign entire system architecture".to_string(),
        status: TaskStatus::Todo,
        priority: Priority::Critical,
        tags: vec!["architecture".to_string(), "refactor".to_string(), "complex".to_string()],
        dependencies: vec!["simple-001".to_string(), "other-001".to_string(), "dep-003".to_string()],
        assignee: Some("senior-dev".to_string()),
        created: Utc::now(),
        estimate: Some("3 months".to_string()),
        complexity: Some(10),
        area: "architecture".to_string(),
        content: complex_task_content,
    };

    // Create directories and save tasks
    let docs_dir = project.tasks_dir.join("docs");
    let arch_dir = project.tasks_dir.join("architecture");
    fs::create_dir_all(&docs_dir)?;
    fs::create_dir_all(&arch_dir)?;

    simple_task.save_to_file(&docs_dir.join("simple-001.md"))?;
    complex_task.save_to_file(&arch_dir.join("complex-001.md"))?;

    // Run complexity analysis
    lint::run(true, None)?;

    // AI should understand complexity differences
    ai::run("How complex are my tasks?".to_string())?;
    ai::run("What should I work on next?".to_string())?; // Should consider complexity

    Ok(())
}

// =============================================================================
// AI-ASSISTED WORKFLOW TESTS
// =============================================================================

#[test]
fn test_ai_guided_development_workflow() -> Result<()> {
    let project = TaskGuardTestProject::new()?;
    project.set_current_dir()?;

    init::run()?;

    // Simulate AI-guided development process

    // 1. AI helps create tasks
    ai::run("Create a task for implementing user authentication".to_string())?;
    ai::run("Add a task for building the frontend login form".to_string())?;
    ai::run("Create a testing task for the auth system".to_string())?;

    // Actually create the tasks based on AI suggestions
    create::run("User Authentication API".to_string(), Some("backend".to_string()), Some("high".to_string()))?;
    create::run("Login Form Component".to_string(), Some("frontend".to_string()), Some("medium".to_string()))?;
    create::run("Authentication Tests".to_string(), Some("testing".to_string()), Some("medium".to_string()))?;

    // Set up dependencies
    project.create_task_manually("testing", "testing-001", "Authentication Tests", TaskStatus::Todo, vec!["backend-001".to_string(), "frontend-001".to_string()])?;

    // 2. AI helps with task prioritization
    ai::run("What should I work on next?".to_string())?;

    // 3. AI provides status updates
    ai::run("Show me the current project status".to_string())?;

    // 4. Simulate completion and AI guidance
    ai::run("I just finished the authentication API".to_string())?;

    // 5. AI helps with dependency analysis
    ai::run("What tasks are now available?".to_string())?;
    ai::run("What's blocked by dependencies?".to_string())?;

    Ok(())
}

#[test]
fn test_ai_error_handling_workflow() -> Result<()> {
    let project = TaskGuardTestProject::new()?;
    project.set_current_dir()?;

    init::run()?;

    // Test AI handling of various edge cases
    ai::run("".to_string())?; // Empty input
    ai::run("asdfghjkl random gibberish".to_string())?; // Nonsense input
    ai::run("What should I work on next?".to_string())?; // No tasks available

    // Create a task and test more scenarios
    create::run("Test Task".to_string(), Some("backend".to_string()), Some("medium".to_string()))?;

    ai::run("Show me tasks in non-existent area".to_string())?;
    ai::run("What should I work on in the year 3000?".to_string())?; // Temporal confusion

    Ok(())
}

// =============================================================================
// PERFORMANCE AND SCALABILITY TESTS
// =============================================================================

#[test]
fn test_large_project_workflow() -> Result<()> {
    let project = TaskGuardTestProject::new()?;
    project.set_current_dir()?;

    init::run()?;

    // Create a large number of tasks to test performance
    let areas = ["backend", "frontend", "api", "auth", "testing", "deployment", "docs"];

    for area in &areas {
        for i in 1..=20 {
            let task_id = format!("{}-{:03}", area, i);
            let title = format!("Task {} in {}", i, area);
            let deps = if i > 1 { vec![format!("{}-{:03}", area, i - 1)] } else { vec![] };

            project.create_task_manually(area, &task_id, &title, TaskStatus::Todo, deps)?;
        }
    }

    // Test performance of various operations on large project
    let start = std::time::Instant::now();
    validate::run()?;
    let validate_duration = start.elapsed();

    let start = std::time::Instant::now();
    lint::run(false, None)?;
    let lint_duration = start.elapsed();

    let start = std::time::Instant::now();
    ai::run("What should I work on next?".to_string())?;
    let ai_duration = start.elapsed();

    // Performance should be reasonable even with many tasks
    assert!(validate_duration < std::time::Duration::from_secs(5),
            "Validation should complete within 5 seconds for large project");
    assert!(lint_duration < std::time::Duration::from_secs(10),
            "Lint analysis should complete within 10 seconds for large project");
    assert!(ai_duration < std::time::Duration::from_secs(3),
            "AI processing should complete within 3 seconds for large project");

    Ok(())
}

#[test]
fn test_complex_dependency_chains() -> Result<()> {
    let project = TaskGuardTestProject::new()?;
    project.set_current_dir()?;

    init::run()?;

    // Create complex dependency scenarios

    // Linear chain: A -> B -> C -> D
    project.create_task_manually("setup", "setup-001", "Foundation", TaskStatus::Done, vec![])?;
    project.create_task_manually("backend", "backend-001", "Core API", TaskStatus::Todo, vec!["setup-001".to_string()])?;
    project.create_task_manually("backend", "backend-002", "Advanced API", TaskStatus::Todo, vec!["backend-001".to_string()])?;
    project.create_task_manually("frontend", "frontend-001", "UI Layer", TaskStatus::Todo, vec!["backend-002".to_string()])?;

    // Diamond dependency: A -> B,C -> D
    project.create_task_manually("auth", "auth-001", "Auth Base", TaskStatus::Todo, vec!["setup-001".to_string()])?;
    project.create_task_manually("auth", "auth-002", "Login", TaskStatus::Todo, vec!["auth-001".to_string()])?;
    project.create_task_manually("auth", "auth-003", "Registration", TaskStatus::Todo, vec!["auth-001".to_string()])?;
    project.create_task_manually("auth", "auth-004", "Full Auth System", TaskStatus::Todo, vec!["auth-002".to_string(), "auth-003".to_string()])?;

    // Multiple dependencies
    project.create_task_manually("integration", "integration-001", "Full Integration", TaskStatus::Todo,
        vec!["frontend-001".to_string(), "auth-004".to_string(), "backend-002".to_string()])?;

    // Test dependency resolution
    validate::run()?;
    ai::run("What's blocked by dependencies?".to_string())?;
    ai::run("What can I work on right now?".to_string())?;

    // Complete some tasks and see cascade effect
    let backend_task_path = project.tasks_dir.join("backend").join("backend-001.md");
    let mut backend_task = Task::from_file(&backend_task_path)?;
    backend_task.status = TaskStatus::Done;
    backend_task.save_to_file(&backend_task_path)?;

    validate::run()?;
    ai::run("What's now available after completing backend-001?".to_string())?;

    Ok(())
}

// =============================================================================
// CROSS-FEATURE INTEGRATION TESTS
// =============================================================================

#[test]
fn test_git_sync_lint_ai_integration() -> Result<()> {
    let project = TaskGuardTestProject::new()?;
    project.set_current_dir()?;

    init::run()?;
    let repo = project.init_git_repo()?;

    // Create tasks with quality issues
    let poor_quality_content = "Brief task description.";
    let good_quality_content = r#"
## Context
Well-structured task with clear objectives.

## Objectives
- Clear goal 1
- Clear goal 2

## Tasks
- [ ] Specific action 1
- [ ] Specific action 2

## Acceptance Criteria
✅ Criteria 1
✅ Criteria 2
"#;

    let poor_task = Task {
        id: "backend-001".to_string(),
        title: "Fix stuff".to_string(),
        status: TaskStatus::Todo,
        priority: Priority::Medium,
        tags: vec![],
        dependencies: vec![],
        assignee: None,
        created: Utc::now(),
        estimate: None,
        complexity: None,
        area: "backend".to_string(),
        content: poor_quality_content.to_string(),
    };

    let good_task = Task {
        id: "backend-002".to_string(),
        title: "Implement User Service".to_string(),
        status: TaskStatus::Todo,
        priority: Priority::High,
        tags: vec!["backend".to_string(), "service".to_string()],
        dependencies: vec![],
        assignee: Some("developer".to_string()),
        created: Utc::now(),
        estimate: Some("6 hours".to_string()),
        complexity: Some(7),
        area: "backend".to_string(),
        content: good_quality_content.to_string(),
    };

    let backend_dir = project.tasks_dir.join("backend");
    fs::create_dir_all(&backend_dir)?;
    poor_task.save_to_file(&backend_dir.join("backend-001.md"))?;
    good_task.save_to_file(&backend_dir.join("backend-002.md"))?;

    // 1. Lint identifies quality issues
    lint::run(true, None)?;

    // 2. Git commits reference tasks
    project.add_git_commit(&repo, "Start work on backend-001 bug fix")?;
    project.add_git_commit(&repo, "WIP: backend-002 user service implementation")?;
    project.add_git_commit(&repo, "Complete backend-001 fixes")?;

    // 3. Sync analyzes Git activity
    sync::run(10, true, false, false)?;

    // 4. AI integrates all information
    ai::run("What's the quality of my tasks?".to_string())?;
    ai::run("Based on Git activity, what should I work on?".to_string())?;
    ai::run("Show me tasks that need improvement".to_string())?;

    Ok(())
}

#[test]
fn test_complete_feature_development_cycle() -> Result<()> {
    let project = TaskGuardTestProject::new()?;
    project.set_current_dir()?;

    // Initialize everything
    init::run()?;
    let repo = project.init_git_repo()?;

    // 1. Planning phase - AI helps create task structure
    ai::run("Create a task for user authentication system".to_string())?;

    // Actually create the planned tasks
    create::run("Authentication System Setup".to_string(), Some("setup".to_string()), Some("high".to_string()))?;
    create::run("Backend Authentication API".to_string(), Some("backend".to_string()), Some("high".to_string()))?;
    create::run("Frontend Login Components".to_string(), Some("frontend".to_string()), Some("medium".to_string()))?;
    create::run("Authentication Tests".to_string(), Some("testing".to_string()), Some("medium".to_string()))?;
    create::run("Documentation Update".to_string(), Some("docs".to_string()), Some("low".to_string()))?;

    // Set up realistic dependencies
    project.create_task_manually("backend", "backend-001", "Backend Auth API", TaskStatus::Todo, vec!["setup-001".to_string()])?;
    project.create_task_manually("frontend", "frontend-001", "Login Components", TaskStatus::Todo, vec!["backend-001".to_string()])?;
    project.create_task_manually("testing", "testing-001", "Auth Tests", TaskStatus::Todo, vec!["frontend-001".to_string()])?;
    project.create_task_manually("docs", "docs-001", "Documentation", TaskStatus::Todo, vec!["testing-001".to_string()])?;

    // 2. Development phase - work through tasks with Git tracking

    // Complete setup
    project.add_git_commit(&repo, "Initial project setup for setup-001")?;
    project.add_git_commit(&repo, "Complete setup-001 environment configuration")?;

    let setup_path = project.tasks_dir.join("setup").join("setup-001.md");
    let mut setup_task = Task::from_file(&setup_path)?;
    setup_task.status = TaskStatus::Done;
    setup_task.save_to_file(&setup_path)?;

    // Work on backend
    project.add_git_commit(&repo, "Start backend-001 authentication implementation")?;
    project.add_git_commit(&repo, "WIP: backend-001 JWT token handling")?;
    project.add_git_commit(&repo, "Add tests for backend-001 auth endpoints")?;
    project.add_git_commit(&repo, "Complete backend-001 authentication API")?;

    let backend_path = project.tasks_dir.join("backend").join("backend-001.md");
    let mut backend_task = Task::from_file(&backend_path)?;
    backend_task.status = TaskStatus::Done;
    backend_task.save_to_file(&backend_path)?;

    // 3. Analysis phase - understand progress
    validate::run()?; // Check what's now available
    sync::run(10, true, false, false)?; // Analyze Git activity
    lint::run(true, None)?; // Check task quality

    // 4. AI provides guidance
    ai::run("What should I work on next?".to_string())?;
    ai::run("How is the authentication feature progressing?".to_string())?;
    ai::run("What's left to complete the auth system?".to_string())?;

    // 5. Continue development cycle
    project.add_git_commit(&repo, "Start frontend-001 login form")?;
    project.add_git_commit(&repo, "Complete frontend-001 user interface")?;

    let frontend_path = project.tasks_dir.join("frontend").join("frontend-001.md");
    let mut frontend_task = Task::from_file(&frontend_path)?;
    frontend_task.status = TaskStatus::Done;
    frontend_task.save_to_file(&frontend_path)?;

    // Final analysis
    validate::run()?;
    sync::run(20, false, false, false)?;
    ai::run("Show me the final project status".to_string())?;

    Ok(())
}