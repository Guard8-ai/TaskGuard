use chrono::Utc;
use std::path::PathBuf;
use taskguard::analysis::{IssueCategory, Severity, TaskAnalyzer};
use taskguard::task::{Priority, Task, TaskStatus};

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
        file_path: PathBuf::from(format!("tasks/test/{}.md", id)),
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
    assert!(
        analysis.complexity_score < 3.0,
        "Simple task should have low complexity"
    );
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
        Some("2 days".to_string()),                                       // Large estimate
        Some(8),                                                          // High manual complexity
    );

    let analysis = analyzer.analyze_task(&complex_task);
    assert!(
        analysis.complexity_score > 6.0,
        "Complex task should have high complexity score"
    );
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
    assert!(
        analysis.quality_score > 8.0,
        "High quality task should have high quality score"
    );

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
    assert!(
        analysis.quality_score < 7.0,
        "Low quality task should have low quality score"
    );
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
    assert!(
        analyzer.estimate_to_complexity_points("2 days")
            > analyzer.estimate_to_complexity_points("4 hours")
    );
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
        vec![
            "dep1".to_string(),
            "dep2".to_string(),
            "dep3".to_string(),
            "dep4".to_string(),
            "dep5".to_string(),
            "dep6".to_string(),
        ],
        None,
        Some(9),
    );

    let analysis = analyzer.analyze_task(&complex_task);

    // Should have complexity-related issues
    let complexity_issues: Vec<_> = analysis
        .issues
        .iter()
        .filter(|issue| matches!(issue.category, IssueCategory::Complexity))
        .collect();

    assert!(
        !complexity_issues.is_empty(),
        "Should detect complexity issues"
    );

    // Should have warnings about high complexity
    let warnings: Vec<_> = analysis
        .issues
        .iter()
        .filter(|issue| matches!(issue.severity, Severity::Warning))
        .collect();

    assert!(
        !warnings.is_empty(),
        "Should have warnings for complex task"
    );
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
    let structure_issues: Vec<_> = analysis
        .issues
        .iter()
        .filter(|issue| matches!(issue.category, IssueCategory::Structure))
        .collect();

    assert!(
        !structure_issues.is_empty(),
        "Should detect structure issues"
    );
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
        file_path: PathBuf::from("tasks/test/test-008.md"),
    };

    let analysis = analyzer.analyze_task(&incomplete_task);

    // Should detect completeness issues
    let completeness_issues: Vec<_> = analysis
        .issues
        .iter()
        .filter(|issue| matches!(issue.category, IssueCategory::Completeness))
        .collect();

    assert!(
        !completeness_issues.is_empty(),
        "Should detect completeness issues"
    );
    assert!(
        completeness_issues.len() >= 3,
        "Should detect missing estimate, tags, and brief content"
    );
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
    let dependency_issues: Vec<_> = analysis
        .issues
        .iter()
        .filter(|issue| matches!(issue.category, IssueCategory::Dependencies))
        .collect();

    assert!(
        !dependency_issues.is_empty(),
        "Should detect dependency issues"
    );
}

#[test]
fn test_analysis_summary() {
    let analyzer = TaskAnalyzer::new();

    let tasks = vec![
        create_test_task("test-1", "Simple", "Simple task", vec![], None, Some(2)),
        create_test_task(
            "test-2",
            "Complex",
            &format!("{}{}", "x".repeat(3000), "- [ ] Task\n".repeat(25)),
            vec!["dep1".to_string(), "dep2".to_string(), "dep3".to_string()],
            Some("5 days".to_string()),
            Some(9),
        ),
        create_test_task(
            "test-3",
            "Medium",
            "Medium complexity task",
            vec![],
            Some("4h".to_string()),
            Some(5),
        ),
    ];

    let analyses = analyzer.analyze_all_tasks(&tasks);
    let summary = analyzer.generate_summary(&analyses);

    assert_eq!(summary.total_tasks, 3);
    assert!(summary.avg_complexity_score > 0.0);
    assert!(summary.avg_quality_score > 0.0);
    assert!(summary.high_complexity_count >= 1); // The complex task should be flagged
}

// =============================================================================
// COMPREHENSIVE COMPLEXITY ANALYSIS TESTS FOR PHASE 2-3 FEATURES
// =============================================================================

#[test]
fn test_comprehensive_task_complexity_scenarios() {
    let analyzer = TaskAnalyzer::new();

    // Scenario 1: Minimal task
    let minimal_task = create_test_task(
        "minimal-001",
        "Fix typo",
        "Fix a small typo in the documentation.",
        vec![],
        Some("5 minutes".to_string()),
        Some(1),
    );

    let analysis = analyzer.analyze_task(&minimal_task);
    assert!(
        analysis.complexity_score < 2.0,
        "Minimal task should have very low complexity"
    );
    assert!(
        analysis.quality_score < 6.0,
        "Brief task should have lower quality score"
    );

    // Scenario 2: Well-structured medium task
    let medium_task_content = r#"
## Context
This task involves implementing a user authentication system using modern security practices.

## Objectives
- Implement secure user login functionality
- Add session management
- Ensure proper error handling

## Tasks
- [ ] Set up authentication middleware
- [ ] Create login endpoint
- [ ] Implement session storage
- [ ] Add input validation
- [ ] Write comprehensive tests

## Acceptance Criteria
✅ Users can authenticate with email/password
✅ Sessions are properly managed
✅ Invalid attempts are handled gracefully

## Technical Notes
- Use JWT for session tokens
- Hash passwords with bcrypt
- Implement rate limiting
"#;

    let medium_task = create_test_task(
        "auth-001",
        "Implement User Authentication System",
        medium_task_content,
        vec!["setup-001".to_string()],
        Some("8 hours".to_string()),
        Some(6),
    );

    let analysis = analyzer.analyze_task(&medium_task);
    assert!(
        analysis.complexity_score >= 4.0 && analysis.complexity_score <= 8.0,
        "Well-structured medium task should have moderate complexity"
    );
    assert!(
        analysis.quality_score >= 8.0,
        "Well-structured task should have high quality"
    );

    // Scenario 3: Overly complex task
    let complex_content = format!(
        "{}{}{}{}{}",
        "This is an extremely complex task that involves multiple systems, extensive coordination, and deep technical knowledge. ",
        "It requires understanding of distributed systems, microservices architecture, database optimization, frontend frameworks, testing strategies, DevOps practices, security considerations, and performance optimization. ".repeat(20),
        "\n\n## Subtasks\n",
        "- [ ] Subtask\n".repeat(50),
        "\n\n## Dependencies\nThis task depends on completing a complex chain of prerequisites."
    );

    let complex_task = create_test_task(
        "mega-001",
        "Redesign Entire System Architecture with Full Stack Implementation",
        &complex_content,
        vec![
            "dep1".to_string(),
            "dep2".to_string(),
            "dep3".to_string(),
            "dep4".to_string(),
            "dep5".to_string(),
            "dep6".to_string(),
            "dep7".to_string(),
        ],
        Some("3 months".to_string()),
        Some(10),
    );

    let analysis = analyzer.analyze_task(&complex_task);
    assert!(
        analysis.complexity_score > 8.0,
        "Overly complex task should have very high complexity"
    );
    assert!(
        !analysis.issues.is_empty(),
        "Complex task should have quality issues"
    );
}

#[test]
fn test_task_quality_dimensions() {
    let analyzer = TaskAnalyzer::new();

    // Test different quality dimensions

    // 1. Structure quality
    let good_structure = r#"
## Context
Clear context provided here.

## Objectives
- Specific objective 1
- Specific objective 2

## Tasks
- [ ] Actionable task 1
- [ ] Actionable task 2

## Acceptance Criteria
✅ Measurable criteria 1
✅ Measurable criteria 2
"#;

    let poor_structure = "Just some random text without any clear structure or organization.";

    let structured_task = create_test_task(
        "struct-001",
        "Structured Task",
        good_structure,
        vec![],
        None,
        None,
    );
    let unstructured_task = create_test_task(
        "unstruct-001",
        "Unstructured Task",
        poor_structure,
        vec![],
        None,
        None,
    );

    let structured_analysis = analyzer.analyze_task(&structured_task);
    let unstructured_analysis = analyzer.analyze_task(&unstructured_task);

    assert!(
        structured_analysis.quality_score > unstructured_analysis.quality_score,
        "Structured task should have higher quality"
    );

    // 2. Completeness quality
    let complete_task = Task {
        id: "complete-001".to_string(),
        title: "Complete Task with All Information".to_string(),
        status: TaskStatus::Todo,
        priority: Priority::High,
        tags: vec!["backend".to_string(), "api".to_string(), "auth".to_string()],
        dependencies: vec!["setup-001".to_string()],
        assignee: Some("developer".to_string()),
        created: Utc::now(),
        estimate: Some("4 hours".to_string()),
        complexity: Some(5),
        area: "backend".to_string(),
        content: good_structure.to_string(),
        file_path: PathBuf::from("tasks/backend/complete-001.md"),
    };

    let incomplete_task = Task {
        id: "incomplete-001".to_string(),
        title: "Task".to_string(),
        status: TaskStatus::Todo,
        priority: Priority::Medium,
        tags: vec![],
        dependencies: vec![],
        assignee: None,
        created: Utc::now(),
        estimate: None,
        complexity: None,
        area: "misc".to_string(),
        content: "Brief.".to_string(),
        file_path: PathBuf::from("tasks/misc/incomplete-001.md"),
    };

    let complete_analysis = analyzer.analyze_task(&complete_task);
    let incomplete_analysis = analyzer.analyze_task(&incomplete_task);

    assert!(
        complete_analysis.quality_score > incomplete_analysis.quality_score,
        "Complete task should have higher quality"
    );

    // Count completeness issues
    let completeness_issues = incomplete_analysis
        .issues
        .iter()
        .filter(|issue| matches!(issue.category, IssueCategory::Completeness))
        .count();
    assert!(
        completeness_issues >= 3,
        "Should detect multiple completeness issues"
    );
}

#[test]
fn test_complexity_factors_weighting() {
    let analyzer = TaskAnalyzer::new();

    // Test individual complexity factors

    // 1. Content length factor
    let short_content = "Brief task description.";
    let long_content = "Very long task description. ".repeat(200);

    let short_task = create_test_task("short-001", "Short Task", short_content, vec![], None, None);
    let long_task = create_test_task("long-001", "Long Task", &long_content, vec![], None, None);

    let short_analysis = analyzer.analyze_task(&short_task);
    let long_analysis = analyzer.analyze_task(&long_task);

    assert!(
        long_analysis.complexity_score > short_analysis.complexity_score,
        "Longer content should increase complexity"
    );

    // 2. Dependencies factor
    let no_deps = create_test_task(
        "nodep-001",
        "No Dependencies",
        "Task with no dependencies",
        vec![],
        None,
        None,
    );
    let many_deps = create_test_task(
        "manydep-001",
        "Many Dependencies",
        "Task with many dependencies",
        vec![
            "dep1".to_string(),
            "dep2".to_string(),
            "dep3".to_string(),
            "dep4".to_string(),
            "dep5".to_string(),
        ],
        None,
        None,
    );

    let no_deps_analysis = analyzer.analyze_task(&no_deps);
    let many_deps_analysis = analyzer.analyze_task(&many_deps);

    assert!(
        many_deps_analysis.complexity_score > no_deps_analysis.complexity_score,
        "More dependencies should increase complexity"
    );

    // 3. Subtasks factor
    let no_subtasks = create_test_task(
        "nosub-001",
        "No Subtasks",
        "Simple task without subtasks",
        vec![],
        None,
        None,
    );
    let many_subtasks_content =
        format!("Task with many subtasks:\n{}", "- [ ] Subtask\n".repeat(20));
    let many_subtasks = create_test_task(
        "manysub-001",
        "Many Subtasks",
        &many_subtasks_content,
        vec![],
        None,
        None,
    );

    let no_subtasks_analysis = analyzer.analyze_task(&no_subtasks);
    let many_subtasks_analysis = analyzer.analyze_task(&many_subtasks);

    assert!(
        many_subtasks_analysis.complexity_score > no_subtasks_analysis.complexity_score,
        "More subtasks should increase complexity"
    );
}

#[test]
fn test_estimate_complexity_conversion() {
    let analyzer = TaskAnalyzer::new();

    let estimate_tests = vec![
        ("30m", 0.5),
        ("1h", 1.0),
        ("2 hours", 2.0),
        ("4h", 4.0),
        ("1d", 8.0), // Assuming 8 hours per day
        ("1 day", 8.0),
        ("3 days", 24.0),
        ("1w", 40.0), // Assuming 5 days per week
        ("1 week", 40.0),
        ("2 weeks", 80.0),
        ("1 month", 160.0), // Assuming ~4 weeks
    ];

    for (estimate_str, expected_range) in estimate_tests {
        let points = analyzer.estimate_to_complexity_points(estimate_str);
        assert!(points > 0.0, "Should parse estimate: {}", estimate_str);

        // Allow some tolerance in the conversion
        let tolerance = expected_range * 0.5;
        assert!(
            points >= expected_range - tolerance && points <= expected_range + tolerance,
            "Estimate '{}' should convert to ~{} points, got {}",
            estimate_str,
            expected_range,
            points
        );
    }

    // Test unknown/invalid estimates
    let unknown_estimates = vec!["unknown", "tbd", "???", "invalid format"];
    for unknown in unknown_estimates {
        let points = analyzer.estimate_to_complexity_points(unknown);
        assert!(
            points > 0.0,
            "Should provide default points for unknown estimate: {}",
            unknown
        );
    }
}

#[test]
fn test_issue_categorization_and_severity() {
    let analyzer = TaskAnalyzer::new();

    // Create tasks designed to trigger specific issue categories

    // Complexity issues
    let huge_content = format!("{}{}", "x".repeat(5000), "- [ ] Task\n".repeat(40));
    let complexity_task = create_test_task(
        "complex-001",
        "Overly Complex Task",
        &huge_content,
        vec![
            "dep1".to_string(),
            "dep2".to_string(),
            "dep3".to_string(),
            "dep4".to_string(),
            "dep5".to_string(),
            "dep6".to_string(),
            "dep7".to_string(),
            "dep8".to_string(),
        ],
        Some("6 months".to_string()),
        Some(10),
    );

    let analysis = analyzer.analyze_task(&complexity_task);

    // Should have multiple issue categories
    let categories: std::collections::HashSet<_> = analysis
        .issues
        .iter()
        .map(|issue| &issue.category)
        .collect();

    assert!(
        categories.contains(&IssueCategory::Complexity),
        "Should detect complexity issues"
    );

    let complexity_issues: Vec<_> = analysis
        .issues
        .iter()
        .filter(|issue| matches!(issue.category, IssueCategory::Complexity))
        .collect();
    assert!(
        !complexity_issues.is_empty(),
        "Should have complexity issues"
    );

    // Check severity distribution
    let error_count = analysis
        .issues
        .iter()
        .filter(|issue| matches!(issue.severity, Severity::Error))
        .count();
    let warning_count = analysis
        .issues
        .iter()
        .filter(|issue| matches!(issue.severity, Severity::Warning))
        .count();
    let info_count = analysis
        .issues
        .iter()
        .filter(|issue| matches!(issue.severity, Severity::Info))
        .count();

    assert!(
        error_count + warning_count + info_count > 0,
        "Should have issues of various severities"
    );

    // Structure issues
    let poor_structure_task = create_test_task(
        "poor-001",
        "Poorly Structured Task",
        "This is just a wall of text without any clear organization or structure or sections or headers or anything that would make it easy to understand what needs to be done or how to approach the work.",
        vec![],
        None,
        None,
    );

    let structure_analysis = analyzer.analyze_task(&poor_structure_task);
    let structure_issues: Vec<_> = structure_analysis
        .issues
        .iter()
        .filter(|issue| matches!(issue.category, IssueCategory::Structure))
        .collect();
    assert!(
        !structure_issues.is_empty(),
        "Should detect structure issues"
    );
}

#[test]
fn test_task_recommendations() {
    let analyzer = TaskAnalyzer::new();

    // Test that analysis provides actionable recommendations
    let problematic_task = Task {
        id: "prob-001".to_string(),
        title: "vague".to_string(),
        status: TaskStatus::Todo,
        priority: Priority::Medium,
        tags: vec![],
        dependencies: vec![],
        assignee: None,
        created: Utc::now(),
        estimate: None,
        complexity: None,
        area: "misc".to_string(),
        content: "do stuff".to_string(),
        file_path: PathBuf::from("tasks/misc/prob-001.md"),
    };

    let analysis = analyzer.analyze_task(&problematic_task);

    assert!(
        !analysis.suggestions.is_empty(),
        "Should provide suggestions for improvement"
    );

    // Suggestions should be actionable
    for suggestion in &analysis.suggestions {
        assert!(!suggestion.is_empty(), "Suggestions should not be empty");
        assert!(suggestion.len() > 10, "Suggestions should be meaningful");
    }
}

#[test]
fn test_batch_analysis_performance() {
    let analyzer = TaskAnalyzer::new();

    // Create many tasks for performance testing
    let mut tasks = Vec::new();
    for i in 1..=100 {
        let content = if i % 10 == 0 {
            format!(
                "Complex task #{} with detailed content: {}",
                i,
                "Some detailed content. ".repeat(50)
            )
        } else {
            format!("Simple task #{}", i)
        };

        let task = create_test_task(
            &format!("task-{:03}", i),
            &format!("Task {}", i),
            &content,
            if i > 1 {
                vec![format!("task-{:03}", i - 1)]
            } else {
                vec![]
            },
            Some(format!("{}h", i % 8 + 1)),
            Some((i % 10 + 1) as u8),
        );
        tasks.push(task);
    }

    let start = std::time::Instant::now();
    let analyses = analyzer.analyze_all_tasks(&tasks);
    let duration = start.elapsed();

    assert!(
        duration < std::time::Duration::from_secs(10),
        "Batch analysis of 100 tasks should complete within 10 seconds, took {:?}",
        duration
    );

    assert_eq!(analyses.len(), 100, "Should analyze all tasks");

    let start = std::time::Instant::now();
    let summary = analyzer.generate_summary(&analyses);
    let summary_duration = start.elapsed();

    assert!(
        summary_duration < std::time::Duration::from_secs(1),
        "Summary generation should be fast"
    );

    assert_eq!(summary.total_tasks, 100, "Summary should include all tasks");
    assert!(
        summary.avg_complexity_score > 0.0,
        "Should calculate average complexity"
    );
    assert!(
        summary.avg_quality_score > 0.0,
        "Should calculate average quality"
    );
}

#[test]
fn test_edge_case_task_content() {
    let analyzer = TaskAnalyzer::new();

    // Test various edge cases

    // Empty content
    let empty_task = create_test_task("empty-001", "Empty Task", "", vec![], None, None);
    let empty_analysis = analyzer.analyze_task(&empty_task);
    assert!(
        empty_analysis.complexity_score >= 0.0,
        "Should handle empty content"
    );

    // Unicode content
    let unicode_content = "Unicode task: 🚀 Deploy 配置 résoudre проблему";
    let unicode_task = create_test_task(
        "unicode-001",
        "Unicode Task",
        unicode_content,
        vec![],
        None,
        None,
    );
    let unicode_analysis = analyzer.analyze_task(&unicode_task);
    assert!(
        unicode_analysis.complexity_score >= 0.0,
        "Should handle unicode content"
    );

    // Very long title
    let long_title = "Very long task title that goes on and on and on ".repeat(10);
    let long_title_task = create_test_task(
        "longtitle-001",
        &long_title,
        "Simple content",
        vec![],
        None,
        None,
    );
    let long_title_analysis = analyzer.analyze_task(&long_title_task);
    assert!(
        long_title_analysis.complexity_score >= 0.0,
        "Should handle long titles"
    );

    // Malformed markdown
    let malformed_markdown = r#"
# Incomplete header
## Another header
- Incomplete list
- [ ] Incomplete checklist
] random bracket
## # Mixed headers
"#;
    let malformed_task = create_test_task(
        "malformed-001",
        "Malformed Task",
        malformed_markdown,
        vec![],
        None,
        None,
    );
    let malformed_analysis = analyzer.analyze_task(&malformed_task);
    assert!(
        malformed_analysis.complexity_score >= 0.0,
        "Should handle malformed markdown"
    );
}
