use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use taskguard::commands::import_md::{self, ImportOptions};
use taskguard::task::{Priority, Task};
use tempfile::TempDir;

fn setup_test_env() -> Result<(TempDir, PathBuf)> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().to_path_buf();

    // Initialize TaskGuard directory structure
    let taskguard_dir = project_dir.join(".taskguard");
    fs::create_dir(&taskguard_dir)?;

    let tasks_dir = project_dir.join("tasks");
    fs::create_dir(&tasks_dir)?;

    // Create config file
    let config_content = r#"
[project]
name = "Test Project"
version = "0.1.0"
areas = ["github", "causality", "testing", "import"]

[settings]
statuses = ["todo", "doing", "review", "done", "blocked"]
priorities = ["low", "medium", "high", "critical"]
complexity_scale = "1-10"
default_estimate_unit = "hours"

[git]
auto_add_tasks = true
auto_commit_on_status_change = false
commit_message_template = "Task {{id}}: {{action}} - {{title}}"

[ai]
enabled = true
claude_code_integration = true
auto_suggestions = true
complexity_analysis = true
"#;
    fs::write(taskguard_dir.join("config.toml"), config_content)?;

    // Change to project directory
    std::env::set_current_dir(&project_dir)?;

    Ok((temp_dir, project_dir))
}

fn create_test_markdown(content: &str, project_dir: &Path) -> Result<PathBuf> {
    let temp_file = project_dir.join("test_import.md");
    fs::write(&temp_file, content)?;
    Ok(temp_file)
}

#[test]
fn test_import_single_fix_section() -> Result<()> {
    let (_temp_dir, project_dir) = setup_test_env()?;

    let markdown = r#"
# Test Document

## Some Context

This is background information.

### Fix #1: Update Archive to Close GitHub Issues

**Priority:** HIGH
**Effort:** 4 hours
**Location:** src/commands/archive.rs

Archive command should close GitHub issues when archiving completed tasks.

### Implementation
- Check for GitHub integration
- Load mapping file
- Close issues via GraphQL API
"#;

    let file_path = create_test_markdown(markdown, &project_dir)?;

    let options = ImportOptions {
        area: Some("github".to_string()),
        prefix: Some("test-fix".to_string()),
        dry_run: false,
        start_number: None,
        tags: vec![],
        priority_override: None,
    };

    import_md::run(file_path.clone(), options)?;

    // Verify task file was created
    let task_file = project_dir.join("tasks/github/test-fix-001.md");
    assert!(task_file.exists(), "Task file should be created");

    // Read and verify task
    let task = Task::from_file(&task_file)?;
    assert_eq!(task.id, "test-fix-001");
    assert_eq!(task.title, "Update Archive to Close GitHub Issues");
    assert_eq!(task.priority, Priority::High);
    assert_eq!(task.estimate, Some("4h".to_string()));
    assert!(task.content.contains("src/commands/archive.rs"));

    fs::remove_file(file_path)?;
    Ok(())
}

#[test]
fn test_import_multiple_sections() -> Result<()> {
    let (_temp_dir, project_dir) = setup_test_env()?;

    let markdown = r#"
### Fix #1: First Fix

**Priority:** HIGH

Content for first fix.

### Fix #2: Second Fix

**Priority:** MEDIUM

Content for second fix.

### Fix #3: Third Fix

Content for third fix.
"#;

    let file_path = create_test_markdown(markdown, &project_dir)?;

    let options = ImportOptions {
        area: Some("testing".to_string()),
        prefix: Some("multi".to_string()),
        dry_run: false,
        start_number: None,
        tags: vec![],
        priority_override: None,
    };

    import_md::run(file_path.clone(), options)?;

    // Verify all three tasks were created
    let task1 = Task::from_file(&project_dir.join("tasks/testing/multi-001.md"))?;
    let task2 = Task::from_file(&project_dir.join("tasks/testing/multi-002.md"))?;
    let task3 = Task::from_file(&project_dir.join("tasks/testing/multi-003.md"))?;

    assert_eq!(task1.id, "multi-001");
    assert_eq!(task2.id, "multi-002");
    assert_eq!(task3.id, "multi-003");

    assert_eq!(task1.priority, Priority::High);
    assert_eq!(task2.priority, Priority::Medium);
    assert_eq!(task3.priority, Priority::Medium); // Default

    fs::remove_file(file_path)?;
    Ok(())
}

#[test]
fn test_import_mixed_issues_and_fixes() -> Result<()> {
    let (_temp_dir, project_dir) = setup_test_env()?;

    let markdown = r#"
### ❌ Issue #1: Archive Command Problem

This is an issue.

### ❌ Issue #2: Clean Command Problem

Another issue.

### Fix #1: Solve Archive Problem

This fixes issue #1.

### Fix #2: Solve Clean Problem

This fixes issue #2.
"#;

    let file_path = create_test_markdown(markdown, &project_dir)?;

    let options = ImportOptions {
        area: Some("testing".to_string()),
        prefix: None, // Let it auto-detect
        dry_run: false,
        start_number: None,
        tags: vec![],
        priority_override: None,
    };

    import_md::run(file_path.clone(), options)?;

    // Verify tasks have type-specific prefixes
    let issue1 = Task::from_file(&project_dir.join("tasks/testing/issue-001.md"))?;
    let issue2 = Task::from_file(&project_dir.join("tasks/testing/issue-002.md"))?;
    let fix1 = Task::from_file(&project_dir.join("tasks/testing/fix-001.md"))?;
    let fix2 = Task::from_file(&project_dir.join("tasks/testing/fix-002.md"))?;

    assert_eq!(issue1.id, "issue-001");
    assert_eq!(issue2.id, "issue-002");
    assert_eq!(fix1.id, "fix-001");
    assert_eq!(fix2.id, "fix-002");

    fs::remove_file(file_path)?;
    Ok(())
}

#[test]
fn test_import_with_dependencies() -> Result<()> {
    let (_temp_dir, project_dir) = setup_test_env()?;

    let markdown = r#"
### Fix #1: Base Fix

No dependencies.

### Fix #2: Dependent Fix

**Depends on:** Fix #1

This depends on fix 1.

### Fix #3: Multiple Dependencies

**Dependencies:** [fix-001, fix-002]

This depends on both.
"#;

    let file_path = create_test_markdown(markdown, &project_dir)?;

    let options = ImportOptions {
        area: Some("testing".to_string()),
        prefix: Some("dep".to_string()),
        dry_run: false,
        start_number: None,
        tags: vec![],
        priority_override: None,
    };

    import_md::run(file_path.clone(), options)?;

    let task1 = Task::from_file(&project_dir.join("tasks/testing/dep-001.md"))?;
    let task2 = Task::from_file(&project_dir.join("tasks/testing/dep-002.md"))?;
    let task3 = Task::from_file(&project_dir.join("tasks/testing/dep-003.md"))?;

    assert_eq!(task1.dependencies, Vec::<String>::new());
    assert_eq!(task2.dependencies, vec!["dep-001"]);
    assert_eq!(task3.dependencies, vec!["fix-001", "fix-002"]);

    fs::remove_file(file_path)?;
    Ok(())
}

#[test]
fn test_import_dry_run() -> Result<()> {
    let (_temp_dir, project_dir) = setup_test_env()?;

    let markdown = r#"
### Fix #1: Test Fix

Content here.
"#;

    let file_path = create_test_markdown(markdown, &project_dir)?;

    let options = ImportOptions {
        area: Some("testing".to_string()),
        prefix: Some("dry".to_string()),
        dry_run: true, // Dry run mode
        start_number: None,
        tags: vec![],
        priority_override: None,
    };

    import_md::run(file_path.clone(), options)?;

    // Verify no task file was created
    let task_file = project_dir.join("tasks/testing/dry-001.md");
    assert!(
        !task_file.exists(),
        "Task file should not be created in dry-run mode"
    );

    fs::remove_file(file_path)?;
    Ok(())
}

#[test]
fn test_import_with_priority_override() -> Result<()> {
    let (_temp_dir, project_dir) = setup_test_env()?;

    let markdown = r#"
### Fix #1: Test Fix

**Priority:** LOW

Content here.
"#;

    let file_path = create_test_markdown(markdown, &project_dir)?;

    let options = ImportOptions {
        area: Some("testing".to_string()),
        prefix: Some("override".to_string()),
        dry_run: false,
        start_number: None,
        tags: vec![],
        priority_override: Some(Priority::Critical), // Override to critical
    };

    import_md::run(file_path.clone(), options)?;

    let task = Task::from_file(&project_dir.join("tasks/testing/override-001.md"))?;
    assert_eq!(task.priority, Priority::Critical); // Should use override, not LOW

    fs::remove_file(file_path)?;
    Ok(())
}

#[test]
fn test_import_with_custom_tags() -> Result<()> {
    let (_temp_dir, project_dir) = setup_test_env()?;

    let markdown = r#"
### Fix #1: Test Fix

Content here.
"#;

    let file_path = create_test_markdown(markdown, &project_dir)?;

    let options = ImportOptions {
        area: Some("testing".to_string()),
        prefix: Some("tags".to_string()),
        dry_run: false,
        start_number: None,
        tags: vec!["urgent".to_string(), "backend".to_string()],
        priority_override: None,
    };

    import_md::run(file_path.clone(), options)?;

    let task = Task::from_file(&project_dir.join("tasks/testing/tags-001.md"))?;
    assert!(task.tags.contains(&"urgent".to_string()));
    assert!(task.tags.contains(&"backend".to_string()));
    assert!(task.tags.contains(&"testing".to_string())); // Area tag
    assert!(task.tags.contains(&"fix".to_string())); // Type tag

    fs::remove_file(file_path)?;
    Ok(())
}

#[test]
fn test_import_no_sections_found() -> Result<()> {
    let (_temp_dir, project_dir) = setup_test_env()?;

    let markdown = r#"
# Regular Markdown

This is just a regular document with no Fix or Issue sections.

## Some Heading

Content without numbered sections.
"#;

    let file_path = create_test_markdown(markdown, &project_dir)?;

    let options = ImportOptions {
        area: Some("testing".to_string()),
        prefix: Some("empty".to_string()),
        dry_run: false,
        start_number: None,
        tags: vec![],
        priority_override: None,
    };

    // Should not error, just report no sections found
    let result = import_md::run(file_path.clone(), options);
    assert!(result.is_ok());

    fs::remove_file(file_path)?;
    Ok(())
}

#[test]
fn test_import_skips_code_blocks_in_dependency_extraction() -> Result<()> {
    let (_temp_dir, project_dir) = setup_test_env()?;

    let markdown = r#"
### Fix #1: Test Fix

**Priority:** HIGH

Implementation:
```rust
// This code mentions dependencies
let mut dependency_issues = Vec::new();
for dep in &task.dependencies {
    // Check dependencies
}
```

This should not create a dependency.
"#;

    let file_path = create_test_markdown(markdown, &project_dir)?;

    let options = ImportOptions {
        area: Some("testing".to_string()),
        prefix: Some("code".to_string()),
        dry_run: false,
        start_number: None,
        tags: vec![],
        priority_override: None,
    };

    import_md::run(file_path.clone(), options)?;

    let task = Task::from_file(&project_dir.join("tasks/testing/code-001.md"))?;
    assert_eq!(
        task.dependencies,
        Vec::<String>::new(),
        "Should not extract dependencies from code blocks"
    );

    fs::remove_file(file_path)?;
    Ok(())
}

#[test]
fn test_complexity_estimation() -> Result<()> {
    let (_temp_dir, project_dir) = setup_test_env()?;

    let short_markdown = r#"
### Fix #1: Short Task

Brief content.
"#;

    let mut long_content = String::from(
        r#"
### Fix #2: Long Task

This is a much longer task with lots of content.
"#,
    );
    long_content.push_str(&"Line of content\n".repeat(60));
    long_content.push_str(
        r#"

```rust
// Code block
fn example() {
    // More lines
}
```

More content after code block.
"#,
    );

    let file1 = create_test_markdown(short_markdown, &project_dir)?;
    let file2 = project_dir.join("test_long.md");
    fs::write(&file2, &long_content)?;

    let options1 = ImportOptions {
        area: Some("testing".to_string()),
        prefix: Some("complex".to_string()),
        dry_run: false,
        start_number: None,
        tags: vec![],
        priority_override: None,
    };

    import_md::run(file1.clone(), options1)?;

    let task1 = Task::from_file(&project_dir.join("tasks/testing/complex-001.md"))?;

    // Short task should have lower complexity
    assert!(task1.complexity.unwrap_or(0) <= 4);

    fs::remove_file(file1)?;
    fs::remove_file(file2)?;
    Ok(())
}
