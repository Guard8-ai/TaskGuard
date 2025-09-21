use taskguard::analysis::{TaskAnalyzer, Severity, IssueCategory};
use taskguard::task::{Task, TaskStatus, Priority};
use chrono::Utc;

fn create_test_task(
    id: &str,
    title: &str,
    content: &str,
    dependencies: Vec<String>,
    estimate: Option<String>,
    complexity: Option<u8>,
) -> Task {
    Task {
        id: id.to_string(),
        title: title.to_string(),
        status: TaskStatus::Todo,
        priority: Priority::Medium,
        tags: vec!["test".to_string()],
        dependencies,
        assignee: None,
        created: Utc::now(),
        estimate,
        complexity,
        area: "test".to_string(),
        content: content.to_string(),
    }
}

#[test]
fn test_complexity_scoring_basic() {
    let analyzer = TaskAnalyzer::new();

    // Simple task should have low complexity
    let simple_task = create_test_task(
        "test-001",
        "Simple Task",
        "A simple task with minimal content.",
        vec![],
        None,
        None,
    );

    let analysis = analyzer.analyze_task(&simple_task);
    assert!(analysis.complexity_score < 3.0, "Simple task should have low complexity");
}

#[test]
fn test_complexity_scoring_high() {
    let analyzer = TaskAnalyzer::new();

    // Complex task with many factors
    let complex_content = format!(
        "{}{}{}",
        "A very complex task with a lot of content. ".repeat(50), // Long content
        "\n\n## Tasks\n",
        "- [ ] Task item\n".repeat(15) // Many task items
    );

    let complex_task = create_test_task(
        "test-002",
        "Complex Task",
        &complex_content,
        vec!["dep1".to_string(), "dep2".to_string(), "dep3".to_string()], // Many dependencies
        Some("2 days".to_string()), // Large estimate
        Some(8), // High manual complexity
    );

    let analysis = analyzer.analyze_task(&complex_task);
    assert!(analysis.complexity_score > 6.0, "Complex task should have high complexity score");
}

#[test]
fn test_quality_scoring() {
    let analyzer = TaskAnalyzer::new();

    // High quality task
    let high_quality_task = create_test_task(
        "test-003",
        "High Quality Task",
        r#"
## Context
This is a well-structured task with clear context.

## Objectives
- Clear objective 1
- Clear objective 2

## Tasks
- [ ] Well defined task 1
- [ ] Well defined task 2

## Acceptance Criteria
✅ Criteria 1: Must be met
✅ Criteria 2: Must also be met
        "#,
        vec![],
        Some("4 hours".to_string()),
        None,
    );

    let analysis = analyzer.analyze_task(&high_quality_task);
    assert!(analysis.quality_score > 8.0, "High quality task should have high quality score");

    // Low quality task
    let low_quality_task = create_test_task(
        "test-004",
        "Low Quality Task",
        "Very brief.",
        vec![],
        None,
        None,
    );

    let analysis = analyzer.analyze_task(&low_quality_task);
    assert!(analysis.quality_score < 7.0, "Low quality task should have low quality score");
}

#[test]
fn test_task_item_counting() {
    let analyzer = TaskAnalyzer::new();

    let content_with_tasks = r#"
Some content here.

## Tasks
- [ ] Task 1
- [x] Completed task
- [ ] Task 3
* [ ] Task 4
* [x] Another completed task

More content.
    "#;

    let task = create_test_task(
        "test-005",
        "Task with Items",
        content_with_tasks,
        vec![],
        None,
        None,
    );

    let count = analyzer.count_task_items(&task.content);
    assert_eq!(count, 5, "Should count all task items correctly");
}

#[test]
fn test_estimate_parsing() {
    let analyzer = TaskAnalyzer::new();

    // Test hour estimates
    assert!(analyzer.estimate_to_complexity_points("4h") > 0.0);
    assert!(analyzer.estimate_to_complexity_points("8 hours") > 0.0);

    // Test day estimates
    assert!(analyzer.estimate_to_complexity_points("2 days") > analyzer.estimate_to_complexity_points("4 hours"));
    assert!(analyzer.estimate_to_complexity_points("1d") > 0.0);

    // Test unknown formats
    assert!(analyzer.estimate_to_complexity_points("unknown") > 0.0);
}

#[test]
fn test_complexity_issues_detection() {
    let analyzer = TaskAnalyzer::new();

    // Task with high complexity that should trigger warnings
    let very_long_content = "x".repeat(3000);
    let many_tasks = "- [ ] Task\n".repeat(25);
    let content = format!("{}\n{}", very_long_content, many_tasks);

    let complex_task = create_test_task(
        "test-006",
        "Complex Task",
        &content,
        vec!["dep1".to_string(), "dep2".to_string(), "dep3".to_string(), "dep4".to_string(), "dep5".to_string(), "dep6".to_string()],
        None,
        Some(9),
    );

    let analysis = analyzer.analyze_task(&complex_task);

    // Should have complexity-related issues
    let complexity_issues: Vec<_> = analysis.issues.iter()
        .filter(|issue| matches!(issue.category, IssueCategory::Complexity))
        .collect();

    assert!(!complexity_issues.is_empty(), "Should detect complexity issues");

    // Should have warnings about high complexity
    let warnings: Vec<_> = analysis.issues.iter()
        .filter(|issue| matches!(issue.severity, Severity::Warning))
        .collect();

    assert!(!warnings.is_empty(), "Should have warnings for complex task");
}

#[test]
fn test_structure_issues_detection() {
    let analyzer = TaskAnalyzer::new();

    // Task without proper structure
    let poor_structure_task = create_test_task(
        "test-007",
        "Poorly Structured Task",
        "This task has a lot of content but no clear structure or objectives defined anywhere in the content. It's just a long rambling description without any clear sections or acceptance criteria.",
        vec![],
        None,
        None,
    );

    let analysis = analyzer.analyze_task(&poor_structure_task);

    // Should detect structure issues
    let structure_issues: Vec<_> = analysis.issues.iter()
        .filter(|issue| matches!(issue.category, IssueCategory::Structure))
        .collect();

    assert!(!structure_issues.is_empty(), "Should detect structure issues");
}

#[test]
fn test_completeness_issues_detection() {
    let analyzer = TaskAnalyzer::new();

    // Task missing various information
    let incomplete_task = Task {
        id: "test-008".to_string(),
        title: "Incomplete Task".to_string(),
        status: TaskStatus::Todo,
        priority: Priority::Medium,
        tags: vec![], // No tags
        dependencies: vec![],
        assignee: None,
        created: Utc::now(),
        estimate: None, // No estimate
        complexity: None,
        area: "test".to_string(),
        content: "Brief.".to_string(), // Very brief content
    };

    let analysis = analyzer.analyze_task(&incomplete_task);

    // Should detect completeness issues
    let completeness_issues: Vec<_> = analysis.issues.iter()
        .filter(|issue| matches!(issue.category, IssueCategory::Completeness))
        .collect();

    assert!(!completeness_issues.is_empty(), "Should detect completeness issues");
    assert!(completeness_issues.len() >= 3, "Should detect missing estimate, tags, and brief content");
}

#[test]
fn test_dependency_issues_detection() {
    let analyzer = TaskAnalyzer::new();

    // Task with too many dependencies
    let many_deps = (1..=10).map(|i| format!("dep-{:03}", i)).collect();

    let dependency_heavy_task = create_test_task(
        "test-009",
        "Dependency Heavy Task",
        "A task with too many dependencies.",
        many_deps,
        None,
        None,
    );

    let analysis = analyzer.analyze_task(&dependency_heavy_task);

    // Should detect dependency issues
    let dependency_issues: Vec<_> = analysis.issues.iter()
        .filter(|issue| matches!(issue.category, IssueCategory::Dependencies))
        .collect();

    assert!(!dependency_issues.is_empty(), "Should detect dependency issues");
}

#[test]
fn test_analysis_summary() {
    let analyzer = TaskAnalyzer::new();

    let tasks = vec![
        create_test_task("test-1", "Simple", "Simple task", vec![], None, Some(2)),
        create_test_task("test-2", "Complex", &format!("{}{}", "x".repeat(3000), "- [ ] Task\n".repeat(25)), vec!["dep1".to_string(), "dep2".to_string(), "dep3".to_string()], Some("5 days".to_string()), Some(9)),
        create_test_task("test-3", "Medium", "Medium complexity task", vec![], Some("4h".to_string()), Some(5)),
    ];

    let analyses = analyzer.analyze_all_tasks(&tasks);
    let summary = analyzer.generate_summary(&analyses);

    assert_eq!(summary.total_tasks, 3);
    assert!(summary.avg_complexity_score > 0.0);
    assert!(summary.avg_quality_score > 0.0);
    assert!(summary.high_complexity_count >= 1); // The complex task should be flagged
}