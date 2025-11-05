use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;
use taskguard::commands::ai::{AIAgent};
use taskguard::task::{Task, TaskStatus, Priority};
use chrono::Utc;

/// Test fixture for creating a temporary TaskGuard project
struct TestProject {
    _temp_dir: TempDir,
    project_path: PathBuf,
    tasks_dir: PathBuf,
}

impl TestProject {
    fn new() -> Result<Self> {
        let temp_dir = tempfile::tempdir()?;
        let project_path = temp_dir.path().to_path_buf();

        // Create TaskGuard directory structure
        let taskguard_dir = project_path.join(".taskguard");
        let tasks_dir = project_path.join("tasks");

        fs::create_dir(&taskguard_dir)?;
        fs::create_dir(&tasks_dir)?;
        fs::create_dir(tasks_dir.join("backend"))?;
        fs::create_dir(tasks_dir.join("frontend"))?;
        fs::create_dir(tasks_dir.join("testing"))?;

        // Create basic config
        let config_content = r#"
[project]
name = "Test Project"
areas = ["backend", "frontend", "testing", "setup"]

[settings]
statuses = ["todo", "doing", "review", "done", "blocked"]
priorities = ["low", "medium", "high", "critical"]
"#;
        fs::write(taskguard_dir.join("config.toml"), config_content)?;

        Ok(TestProject {
            _temp_dir: temp_dir,
            project_path,
            tasks_dir,
        })
    }

    fn create_task_file(&self, area: &str, id: &str, title: &str, status: TaskStatus, priority: Priority, dependencies: Vec<String>) -> Result<()> {
        let file_path = self.tasks_dir.join(area).join(format!("{}.md", id));
        let task = Task {
            id: id.to_string(),
            title: title.to_string(),
            status,
            priority,
            tags: vec!["test".to_string()],
            dependencies,
            assignee: None,
            created: Utc::now(),
            estimate: None,
            complexity: Some(5),
            area: area.to_string(),
            content: format!("Test task content for {}", title),
            file_path: file_path.clone(),
        };

        task.save_to_file(&file_path)?;
        Ok(())
    }

    fn set_current_dir(&self) -> Result<()> {
        std::env::set_current_dir(&self.project_path)?;
        Ok(())
    }
}

// =============================================================================
// AI AGENT INITIALIZATION TESTS
// =============================================================================

#[test]
fn test_ai_agent_initialization() -> Result<()> {
    let test_project = TestProject::new()?;
    test_project.set_current_dir()?;

    let _ai_agent = AIAgent::new()?;
    // Should initialize successfully even without Git repo
    Ok(())
}

#[test]
fn test_ai_agent_initialization_without_taskguard() {
    let temp_dir = tempfile::tempdir().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();

    let result = AIAgent::new();
    // Should still work, just without TaskGuard-specific features
    assert!(result.is_ok());
}

// =============================================================================
// PATTERN RECOGNITION TESTS
// =============================================================================

#[test]
fn test_task_creation_pattern_recognition() -> Result<()> {
    let test_project = TestProject::new()?;
    test_project.set_current_dir()?;

    let ai_agent = AIAgent::new()?;

    let test_inputs = vec![
        "Create a task for implementing user authentication",
        "Add a new feature for API endpoints",
        "I need to build a login component",
        "We should implement database migrations",
        "Can you create a task to add tests for the user service?",
    ];

    for input in test_inputs {
        let response = ai_agent.process_natural_language(input)?;
        assert!(response.contains("create"), "Should recognize task creation pattern in: {}", input);
        assert!(response.contains("taskguard create"), "Should provide CLI command suggestion");
    }

    Ok(())
}

#[test]
fn test_status_inquiry_pattern_recognition() -> Result<()> {
    let test_project = TestProject::new()?;
    test_project.set_current_dir()?;

    // Create some test tasks
    test_project.create_task_file("backend", "backend-001", "User Auth", TaskStatus::Todo, Priority::High, vec![])?;
    test_project.create_task_file("frontend", "frontend-001", "Login UI", TaskStatus::Doing, Priority::Medium, vec![])?;
    test_project.create_task_file("testing", "testing-001", "Auth Tests", TaskStatus::Done, Priority::Low, vec![])?;

    let ai_agent = AIAgent::new()?;

    let test_inputs = vec![
        "What's the current status of the project?",
        "Show me all tasks",
        "List what we have",
        "What tasks are available?",
        "Give me an overview",
    ];

    for input in test_inputs {
        let response = ai_agent.process_natural_language(input)?;
        assert!(response.contains("Status Overview") || response.contains("Total tasks"),
                "Should recognize status inquiry in: {}", input);
        assert!(response.contains("3"), "Should show total task count");
    }

    Ok(())
}

#[test]
fn test_next_task_recommendation_pattern() -> Result<()> {
    let test_project = TestProject::new()?;
    test_project.set_current_dir()?;

    // Create tasks with different priorities
    test_project.create_task_file("backend", "backend-001", "High Priority Task", TaskStatus::Todo, Priority::High, vec![])?;
    test_project.create_task_file("frontend", "frontend-001", "Medium Priority Task", TaskStatus::Todo, Priority::Medium, vec![])?;
    test_project.create_task_file("testing", "testing-001", "Low Priority Task", TaskStatus::Todo, Priority::Low, vec![])?;

    let ai_agent = AIAgent::new()?;

    let test_inputs = vec![
        "What should I work on next?",
        "What's ready to work on?",
        "Recommend a task for me",
        "What can I do now?",
        "What's available?",
    ];

    for input in test_inputs {
        let response = ai_agent.process_natural_language(input)?;
        assert!(response.contains("Recommended Next Task") || response.contains("backend-001"),
                "Should recommend high priority task for: {}", input);
        assert!(response.contains("High"), "Should show priority information");
    }

    Ok(())
}

#[test]
fn test_completion_announcement_pattern() -> Result<()> {
    let test_project = TestProject::new()?;
    test_project.set_current_dir()?;

    let ai_agent = AIAgent::new()?;

    let test_inputs = vec![
        "I just finished the authentication work",
        "Completed the user login feature",
        "Done with backend-001",
        "Finished implementing the API",
        "Just built the frontend component",
    ];

    for input in test_inputs {
        let response = ai_agent.process_natural_language(input)?;
        assert!(response.contains("Great job") || response.contains("completed") || response.contains("Next steps"),
                "Should recognize completion in: {}", input);
        assert!(response.contains("validate") || response.contains("sync"),
                "Should suggest next steps");
    }

    Ok(())
}

#[test]
fn test_dependency_query_pattern() -> Result<()> {
    let test_project = TestProject::new()?;
    test_project.set_current_dir()?;

    // Create tasks with dependencies
    test_project.create_task_file("backend", "backend-001", "Setup Database", TaskStatus::Done, Priority::High, vec![])?;
    test_project.create_task_file("backend", "backend-002", "User API", TaskStatus::Todo, Priority::High, vec!["backend-001".to_string()])?;
    test_project.create_task_file("frontend", "frontend-001", "Login Form", TaskStatus::Todo, Priority::Medium, vec!["backend-002".to_string()])?;

    let ai_agent = AIAgent::new()?;

    let test_inputs = vec![
        "What tasks are blocked?",
        "Show me dependencies",
        "What's waiting for other tasks?",
        "Which tasks depend on others?",
        "What are the prerequisites?",
    ];

    for input in test_inputs {
        let response = ai_agent.process_natural_language(input)?;
        assert!(response.contains("Dependency Analysis") || response.contains("blocked"),
                "Should show dependency analysis for: {}", input);
        assert!(response.contains("frontend-001"), "Should show blocked tasks");
    }

    Ok(())
}

// =============================================================================
// TASK PRIORITY AND COMPLEXITY ANALYSIS TESTS
// =============================================================================

#[test]
fn test_task_prioritization_logic() -> Result<()> {
    let test_project = TestProject::new()?;
    test_project.set_current_dir()?;

    // Create tasks with different priorities and complexities
    test_project.create_task_file("backend", "backend-001", "Critical Bug Fix", TaskStatus::Todo, Priority::Critical, vec![])?;
    test_project.create_task_file("backend", "backend-002", "High Priority Feature", TaskStatus::Todo, Priority::High, vec![])?;
    test_project.create_task_file("backend", "backend-003", "Medium Priority Task", TaskStatus::Todo, Priority::Medium, vec![])?;
    test_project.create_task_file("backend", "backend-004", "Low Priority Enhancement", TaskStatus::Todo, Priority::Low, vec![])?;

    let ai_agent = AIAgent::new()?;
    let response = ai_agent.process_natural_language("What should I work on next?")?;

    // Should recommend the critical priority task first
    assert!(response.contains("backend-001") || response.contains("Critical"),
            "Should prioritize critical tasks first");

    Ok(())
}

#[test]
fn test_complexity_analysis_integration() -> Result<()> {
    let test_project = TestProject::new()?;
    test_project.set_current_dir()?;

    let ai_agent = AIAgent::new()?;

    let test_inputs = vec![
        "How complex are my tasks?",
        "Show me task complexity",
        "What's the difficulty level?",
        "How much effort will this take?",
        "Analyze task complexity",
    ];

    for input in test_inputs {
        let response = ai_agent.process_natural_language(input)?;
        assert!(response.contains("Complexity Analysis") || response.contains("complexity"),
                "Should provide complexity analysis for: {}", input);
    }

    Ok(())
}

// =============================================================================
// NATURAL LANGUAGE EXTRACTION TESTS
// =============================================================================

#[test]
fn test_task_title_extraction() -> Result<()> {
    let test_project = TestProject::new()?;
    test_project.set_current_dir()?;

    let ai_agent = AIAgent::new()?;

    let test_cases = vec![
        ("Create a task for implementing user authentication", "implementing user authentication"),
        ("Add a new feature for real-time notifications", "real-time notifications"),
        ("I need to build a dashboard component", "dashboard component"),
        ("Can you create a task to fix the login bug?", "fix the login bug"),
    ];

    for (input, expected_content) in test_cases {
        let response = ai_agent.process_natural_language(input)?;
        assert!(response.to_lowercase().contains(&expected_content.to_lowercase()),
                "Should extract '{}' from: {}", expected_content, input);
    }

    Ok(())
}

#[test]
fn test_area_inference() -> Result<()> {
    let test_project = TestProject::new()?;
    test_project.set_current_dir()?;

    let ai_agent = AIAgent::new()?;

    let test_cases = vec![
        ("Create an API endpoint for users", "backend"),
        ("Build a React component for login", "frontend"),
        ("Add authentication to the server", "auth"),
        ("Write tests for the user service", "testing"),
        ("Set up the database configuration", "setup"),
    ];

    for (input, expected_area) in test_cases {
        let response = ai_agent.process_natural_language(input)?;
        assert!(response.contains(expected_area),
                "Should infer area '{}' from: {}", expected_area, input);
    }

    Ok(())
}

#[test]
fn test_priority_inference() -> Result<()> {
    let test_project = TestProject::new()?;
    test_project.set_current_dir()?;

    let ai_agent = AIAgent::new()?;

    let test_cases = vec![
        ("Create a critical bug fix task", "critical"),
        ("Add a high priority feature", "high"),
        ("This is an important feature", "high"),
        ("Create a low priority enhancement", "low"),
        ("Add a minor improvement", "low"),
        ("Create a regular task", "medium"),
    ];

    for (input, expected_priority) in test_cases {
        let response = ai_agent.process_natural_language(input)?;
        assert!(response.contains(expected_priority),
                "Should infer priority '{}' from: {}", expected_priority, input);
    }

    Ok(())
}

// =============================================================================
// ERROR HANDLING AND EDGE CASES
// =============================================================================

#[test]
fn test_empty_project_handling() -> Result<()> {
    let test_project = TestProject::new()?;
    test_project.set_current_dir()?;

    let ai_agent = AIAgent::new()?;

    let response = ai_agent.process_natural_language("What should I work on next?")?;
    assert!(response.contains("No tasks") || response.contains("available"),
            "Should handle empty project gracefully");

    Ok(())
}

#[test]
fn test_all_tasks_blocked_scenario() -> Result<()> {
    let test_project = TestProject::new()?;
    test_project.set_current_dir()?;

    // Create tasks where all depend on a non-existent task
    test_project.create_task_file("backend", "backend-001", "Task 1", TaskStatus::Todo, Priority::High, vec!["missing-task".to_string()])?;
    test_project.create_task_file("backend", "backend-002", "Task 2", TaskStatus::Todo, Priority::Medium, vec!["missing-task".to_string()])?;

    let ai_agent = AIAgent::new()?;
    let response = ai_agent.process_natural_language("What should I work on next?")?;

    assert!(response.contains("No tasks") || response.contains("blocked"),
            "Should handle all-blocked scenario");

    Ok(())
}

#[test]
fn test_malformed_input_handling() -> Result<()> {
    let test_project = TestProject::new()?;
    test_project.set_current_dir()?;

    let ai_agent = AIAgent::new()?;

    let malformed_inputs = vec![
        "",
        "   ",
        "askdfjklasjdf random gibberish",
        "🚀🔥💯",
        "SELECT * FROM tasks; DROP TABLE users;",
        "&lt;script&gt;alert('xss')&lt;/script&gt;",
    ];

    for input in malformed_inputs {
        let response = ai_agent.process_natural_language(input)?;
        assert!(!response.is_empty(), "Should provide some response for malformed input: '{}'", input);
        assert!(response.contains("help") || response.contains("guidance"),
                "Should provide helpful guidance for unclear input");
    }

    Ok(())
}

#[test]
fn test_very_long_input_handling() -> Result<()> {
    let test_project = TestProject::new()?;
    test_project.set_current_dir()?;

    let ai_agent = AIAgent::new()?;

    // Create very long input
    let long_input = format!("Create a task for {} please", "x".repeat(10000));

    let start = std::time::Instant::now();
    let response = ai_agent.process_natural_language(&long_input)?;
    let duration = start.elapsed();

    assert!(duration < std::time::Duration::from_secs(5),
            "Should handle long input within reasonable time");
    assert!(!response.is_empty(), "Should provide response for long input");

    Ok(())
}

// =============================================================================
// INTEGRATION WITH OTHER COMPONENTS
// =============================================================================

#[test]
fn test_ai_with_git_integration() -> Result<()> {
    let test_project = TestProject::new()?;
    test_project.set_current_dir()?;

    // Initialize a git repo in the test project
    let repo = git2::Repository::init(&test_project.project_path)?;
    let mut config = repo.config()?;
    config.set_str("user.name", "Test User")?;
    config.set_str("user.email", "test@example.com")?;

    let ai_agent = AIAgent::new()?;

    // Should work with Git integration available
    let response = ai_agent.process_natural_language("What's the project status?")?;
    assert!(!response.is_empty(), "Should work with Git repository present");

    Ok(())
}

#[test]
fn test_ai_task_validation_integration() -> Result<()> {
    let test_project = TestProject::new()?;
    test_project.set_current_dir()?;

    // Create tasks with various dependency states
    test_project.create_task_file("backend", "backend-001", "Foundation", TaskStatus::Done, Priority::High, vec![])?;
    test_project.create_task_file("backend", "backend-002", "Dependent Task", TaskStatus::Todo, Priority::High, vec!["backend-001".to_string()])?;
    test_project.create_task_file("backend", "backend-003", "Blocked Task", TaskStatus::Todo, Priority::Medium, vec!["nonexistent-task".to_string()])?;

    let ai_agent = AIAgent::new()?;

    // Test dependency analysis
    let response = ai_agent.process_natural_language("What tasks are available?")?;
    assert!(response.contains("backend-002"), "Should show available task");

    let blocked_response = ai_agent.process_natural_language("What's blocked?")?;
    assert!(response.contains("backend-003") || blocked_response.contains("backend-003"),
            "Should identify blocked task");

    Ok(())
}

// =============================================================================
// OUTPUT FORMAT AND QUALITY TESTS
// =============================================================================

#[test]
fn test_response_formatting_quality() -> Result<()> {
    let test_project = TestProject::new()?;
    test_project.set_current_dir()?;

    test_project.create_task_file("backend", "backend-001", "Test Task", TaskStatus::Todo, Priority::High, vec![])?;

    let ai_agent = AIAgent::new()?;
    let response = ai_agent.process_natural_language("What should I work on next?")?;

    // Check for proper formatting
    assert!(response.contains("**"), "Should use markdown bold formatting");
    assert!(response.contains("•") || response.contains("-"), "Should use bullet points");
    assert!(response.contains("\n"), "Should have proper line breaks");

    // Check for helpful elements
    assert!(response.contains("taskguard") || response.contains("💡") || response.contains("🎯"),
            "Should include helpful guidance or emojis");

    Ok(())
}

#[test]
fn test_response_actionability() -> Result<()> {
    let test_project = TestProject::new()?;
    test_project.set_current_dir()?;

    let ai_agent = AIAgent::new()?;

    let test_inputs = vec![
        "Create a task for user authentication",
        "What should I work on next?",
        "I finished the backend work",
    ];

    for input in test_inputs {
        let response = ai_agent.process_natural_language(input)?;

        // Should provide actionable guidance
        assert!(response.contains("taskguard") || response.contains("run") || response.contains("command") || response.contains("steps"),
                "Should provide actionable guidance for: {}", input);
    }

    Ok(())
}