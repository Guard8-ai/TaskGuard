use chrono::Utc;
use std::fs;
use taskguard::github::mapper::{IssueMapping, TaskIssueMapper};
use taskguard::task::TaskStatus;
use tempfile::TempDir;

fn create_test_mapping(task_id: &str, issue_number: i64, archived: bool) -> IssueMapping {
    IssueMapping {
        task_id: task_id.to_string(),
        issue_number,
        issue_id: format!("issue_id_{}", issue_number),
        project_item_id: format!("project_item_{}", issue_number),
        synced_at: Utc::now().to_rfc3339(),
        is_archived: archived,
    }
}

#[test]
fn test_crud_operations() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test-mapping.json");
    let mut mapper = TaskIssueMapper::with_path(file_path.clone());

    // Test add
    let mapping1 = create_test_mapping("task-001", 1, false);
    mapper.add_mapping(mapping1.clone()).unwrap();
    assert_eq!(mapper.get_all_mappings().len(), 1);

    // Test add duplicate (should fail)
    let duplicate = create_test_mapping("task-001", 2, false);
    assert!(mapper.add_mapping(duplicate).is_err());

    // Test get by task_id
    let found = mapper.get_by_task_id("task-001").unwrap();
    assert_eq!(found.issue_number, 1);
    assert_eq!(found.issue_id, "issue_id_1");
    assert_eq!(found.project_item_id, "project_item_1");

    // Test update
    let mut updated_mapping = mapping1.clone();
    updated_mapping.is_archived = true;
    mapper.update_mapping(updated_mapping).unwrap();

    let found = mapper.get_by_task_id("task-001").unwrap();
    assert!(found.is_archived);

    // Test remove
    mapper.remove_mapping("task-001").unwrap();
    assert_eq!(mapper.get_all_mappings().len(), 0);

    // Test remove non-existent (should fail)
    assert!(mapper.remove_mapping("task-999").is_err());
}

#[test]
fn test_lookup_operations() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test-mapping.json");
    let mut mapper = TaskIssueMapper::with_path(file_path);

    // Add multiple mappings
    mapper
        .add_mapping(create_test_mapping("task-001", 1, false))
        .unwrap();
    mapper
        .add_mapping(create_test_mapping("task-002", 2, false))
        .unwrap();
    mapper
        .add_mapping(create_test_mapping("task-003", 3, true))
        .unwrap();

    // Test get by issue number
    let found = mapper.get_by_issue_number(2).unwrap();
    assert_eq!(found.task_id, "task-002");

    // Test get by project item id
    let found = mapper.get_by_project_item_id("project_item_1").unwrap();
    assert_eq!(found.task_id, "task-001");

    // Test get by non-existent issue number
    assert!(mapper.get_by_issue_number(999).is_none());

    // Test get by non-existent project item id
    assert!(mapper.get_by_project_item_id("non_existent").is_none());
}

#[test]
fn test_archive_operations() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test-mapping.json");
    let mut mapper = TaskIssueMapper::with_path(file_path);

    // Add mappings
    mapper
        .add_mapping(create_test_mapping("task-001", 1, false))
        .unwrap();
    mapper
        .add_mapping(create_test_mapping("task-002", 2, false))
        .unwrap();
    mapper
        .add_mapping(create_test_mapping("task-003", 3, true))
        .unwrap();

    // Test get active mappings
    let active = mapper.get_active_mappings();
    assert_eq!(active.len(), 2);
    assert!(active.iter().any(|m| m.task_id == "task-001"));
    assert!(active.iter().any(|m| m.task_id == "task-002"));

    // Test get archived mappings
    let archived = mapper.get_archived_mappings();
    assert_eq!(archived.len(), 1);
    assert_eq!(archived[0].task_id, "task-003");

    // Test archive
    mapper.archive_mapping("task-001").unwrap();
    assert_eq!(mapper.get_active_mappings().len(), 1);
    assert_eq!(mapper.get_archived_mappings().len(), 2);

    // Test unarchive
    mapper.unarchive_mapping("task-001").unwrap();
    assert_eq!(mapper.get_active_mappings().len(), 2);
    assert_eq!(mapper.get_archived_mappings().len(), 1);

    // Test archive non-existent (should fail)
    assert!(mapper.archive_mapping("task-999").is_err());
}

#[test]
fn test_persistence() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test-mapping.json");

    // Create mapper and add mappings
    {
        let mut mapper = TaskIssueMapper::with_path(file_path.clone());
        mapper
            .add_mapping(create_test_mapping("task-001", 1, false))
            .unwrap();
        mapper
            .add_mapping(create_test_mapping("task-002", 2, true))
            .unwrap();
    }

    // Verify file was created
    assert!(file_path.exists());

    // Load mapper from file
    let mut mapper = TaskIssueMapper::with_path(file_path.clone());
    mapper.load().unwrap();

    // Verify mappings were persisted
    assert_eq!(mapper.get_all_mappings().len(), 2);
    assert!(mapper.get_by_task_id("task-001").is_some());
    assert!(mapper.get_by_task_id("task-002").is_some());

    // Verify archived status was persisted
    let task2 = mapper.get_by_task_id("task-002").unwrap();
    assert!(task2.is_archived);
}

#[test]
fn test_json_serialization() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test-mapping.json");
    let mut mapper = TaskIssueMapper::with_path(file_path.clone());

    let mapping = create_test_mapping("task-001", 42, false);
    mapper.add_mapping(mapping).unwrap();

    // Read the JSON file directly
    let json_content = fs::read_to_string(&file_path).unwrap();

    // Verify it's valid JSON and contains expected fields
    assert!(json_content.contains("task-001"));
    assert!(json_content.contains("issue_id_42"));
    assert!(json_content.contains("project_item_42"));
    assert!(json_content.contains("\"issue_number\": 42"));
    assert!(json_content.contains("\"is_archived\": false"));

    // Verify it's pretty-printed (has indentation)
    assert!(json_content.contains("  "));
}

#[test]
fn test_status_conversion_exact_match() {
    let options = vec![
        ("opt1".to_string(), "Backlog".to_string()),
        ("opt2".to_string(), "In progress".to_string()),
        ("opt3".to_string(), "In review".to_string()),
        ("opt4".to_string(), "Done".to_string()),
        ("opt5".to_string(), "Blocked".to_string()),
    ];

    // Test exact matches
    assert_eq!(
        TaskIssueMapper::find_best_status_option(&TaskStatus::Todo, &options),
        Some("opt1".to_string())
    );
    assert_eq!(
        TaskIssueMapper::find_best_status_option(&TaskStatus::Doing, &options),
        Some("opt2".to_string())
    );
    assert_eq!(
        TaskIssueMapper::find_best_status_option(&TaskStatus::Review, &options),
        Some("opt3".to_string())
    );
    assert_eq!(
        TaskIssueMapper::find_best_status_option(&TaskStatus::Done, &options),
        Some("opt4".to_string())
    );
    assert_eq!(
        TaskIssueMapper::find_best_status_option(&TaskStatus::Blocked, &options),
        Some("opt5".to_string())
    );
}

#[test]
fn test_status_conversion_case_insensitive() {
    let options = vec![
        ("opt1".to_string(), "BACKLOG".to_string()),
        ("opt2".to_string(), "in PROGRESS".to_string()),
        ("opt3".to_string(), "In Review".to_string()),
        ("opt4".to_string(), "DONE".to_string()),
    ];

    // All should match case-insensitively
    assert_eq!(
        TaskIssueMapper::find_best_status_option(&TaskStatus::Todo, &options),
        Some("opt1".to_string())
    );
    assert_eq!(
        TaskIssueMapper::find_best_status_option(&TaskStatus::Doing, &options),
        Some("opt2".to_string())
    );
    assert_eq!(
        TaskIssueMapper::find_best_status_option(&TaskStatus::Review, &options),
        Some("opt3".to_string())
    );
    assert_eq!(
        TaskIssueMapper::find_best_status_option(&TaskStatus::Done, &options),
        Some("opt4".to_string())
    );
}

#[test]
fn test_status_conversion_priority() {
    // Test that higher priority names are matched first
    let options_todo = vec![
        ("opt1".to_string(), "Ready".to_string()), // Lower priority
        ("opt2".to_string(), "Backlog".to_string()), // Higher priority
    ];
    assert_eq!(
        TaskIssueMapper::find_best_status_option(&TaskStatus::Todo, &options_todo),
        Some("opt2".to_string()) // Should pick Backlog over Ready
    );

    let options_doing = vec![
        ("opt1".to_string(), "Working".to_string()), // Lower priority
        ("opt2".to_string(), "In progress".to_string()), // Higher priority
    ];
    assert_eq!(
        TaskIssueMapper::find_best_status_option(&TaskStatus::Doing, &options_doing),
        Some("opt2".to_string()) // Should pick "In progress" over "Working"
    );
}

#[test]
fn test_status_conversion_no_match() {
    let options = vec![
        ("opt1".to_string(), "Custom Status".to_string()),
        ("opt2".to_string(), "Another Status".to_string()),
    ];

    // Should return None when no match found
    assert_eq!(
        TaskIssueMapper::find_best_status_option(&TaskStatus::Todo, &options),
        None
    );
}

#[test]
fn test_status_conversion_alternative_names() {
    // Test fallback names
    let todo_options = vec![
        ("opt1".to_string(), "To Do".to_string()), // Alternative name
    ];
    assert_eq!(
        TaskIssueMapper::find_best_status_option(&TaskStatus::Todo, &todo_options),
        Some("opt1".to_string())
    );

    let doing_options = vec![
        ("opt1".to_string(), "Doing".to_string()), // Alternative name
    ];
    assert_eq!(
        TaskIssueMapper::find_best_status_option(&TaskStatus::Doing, &doing_options),
        Some("opt1".to_string())
    );

    let done_options = vec![
        ("opt1".to_string(), "Completed".to_string()), // Alternative name
    ];
    assert_eq!(
        TaskIssueMapper::find_best_status_option(&TaskStatus::Done, &done_options),
        Some("opt1".to_string())
    );
}

#[test]
fn test_github_column_to_status_basic() {
    assert_eq!(
        TaskIssueMapper::github_column_to_status("Done"),
        TaskStatus::Done
    );
    assert_eq!(
        TaskIssueMapper::github_column_to_status("Completed"),
        TaskStatus::Done
    );
    assert_eq!(
        TaskIssueMapper::github_column_to_status("Complete"),
        TaskStatus::Done
    );
}

#[test]
fn test_github_column_to_status_case_insensitive() {
    assert_eq!(
        TaskIssueMapper::github_column_to_status("DONE"),
        TaskStatus::Done
    );
    assert_eq!(
        TaskIssueMapper::github_column_to_status("in progress"),
        TaskStatus::Doing
    );
    assert_eq!(
        TaskIssueMapper::github_column_to_status("IN REVIEW"),
        TaskStatus::Review
    );
    assert_eq!(
        TaskIssueMapper::github_column_to_status("BLOCKED"),
        TaskStatus::Blocked
    );
}

#[test]
fn test_github_column_to_status_pattern_matching() {
    // Test partial matches
    assert_eq!(
        TaskIssueMapper::github_column_to_status("In Progress - Sprint 1"),
        TaskStatus::Doing
    );
    assert_eq!(
        TaskIssueMapper::github_column_to_status("Code Review"),
        TaskStatus::Review
    );
    assert_eq!(
        TaskIssueMapper::github_column_to_status("Working on it"),
        TaskStatus::Doing
    );
}

#[test]
fn test_github_column_to_status_default() {
    // Unknown columns should default to Todo
    assert_eq!(
        TaskIssueMapper::github_column_to_status("Backlog"),
        TaskStatus::Todo
    );
    assert_eq!(
        TaskIssueMapper::github_column_to_status("Ready"),
        TaskStatus::Todo
    );
    assert_eq!(
        TaskIssueMapper::github_column_to_status("Unknown Status"),
        TaskStatus::Todo
    );
    assert_eq!(
        TaskIssueMapper::github_column_to_status(""),
        TaskStatus::Todo
    );
}

#[test]
fn test_bidirectional_status_conversion() {
    // Test round-trip conversion where possible
    let options = vec![
        ("opt1".to_string(), "Backlog".to_string()),
        ("opt2".to_string(), "In progress".to_string()),
        ("opt3".to_string(), "In review".to_string()),
        ("opt4".to_string(), "Done".to_string()),
        ("opt5".to_string(), "Blocked".to_string()),
    ];

    // TaskGuard -> GitHub -> TaskGuard
    let doing_option =
        TaskIssueMapper::find_best_status_option(&TaskStatus::Doing, &options).unwrap();
    let github_column = options
        .iter()
        .find(|(id, _)| id == &doing_option)
        .unwrap()
        .1
        .clone();
    let back_to_taskguard = TaskIssueMapper::github_column_to_status(&github_column);
    assert_eq!(back_to_taskguard, TaskStatus::Doing);

    // Test another round trip
    let review_option =
        TaskIssueMapper::find_best_status_option(&TaskStatus::Review, &options).unwrap();
    let github_column = options
        .iter()
        .find(|(id, _)| id == &review_option)
        .unwrap()
        .1
        .clone();
    let back_to_taskguard = TaskIssueMapper::github_column_to_status(&github_column);
    assert_eq!(back_to_taskguard, TaskStatus::Review);
}

#[test]
fn test_real_world_github_columns() {
    // Test with actual GitHub Projects v2 default columns
    let real_options = vec![
        ("option1".to_string(), "Backlog".to_string()),
        ("option2".to_string(), "Ready".to_string()),
        ("option3".to_string(), "In progress".to_string()),
        ("option4".to_string(), "In review".to_string()),
        ("option5".to_string(), "Done".to_string()),
    ];

    // Verify all TaskGuard statuses can be mapped
    assert!(TaskIssueMapper::find_best_status_option(&TaskStatus::Todo, &real_options).is_some());
    assert!(TaskIssueMapper::find_best_status_option(&TaskStatus::Doing, &real_options).is_some());
    assert!(TaskIssueMapper::find_best_status_option(&TaskStatus::Review, &real_options).is_some());
    assert!(TaskIssueMapper::find_best_status_option(&TaskStatus::Done, &real_options).is_some());

    // Blocked should fall back to Backlog if no Blocked column exists
    assert!(
        TaskIssueMapper::find_best_status_option(&TaskStatus::Blocked, &real_options).is_some()
    );
}

#[test]
fn test_multiple_mappings_persistence() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test-mapping.json");

    // Create multiple mappings
    {
        let mut mapper = TaskIssueMapper::with_path(file_path.clone());
        for i in 1..=10 {
            let mapping = create_test_mapping(
                &format!("task-{:03}", i),
                i,
                i % 3 == 0, // Archive every 3rd task
            );
            mapper.add_mapping(mapping).unwrap();
        }
    }

    // Load and verify
    let mut mapper = TaskIssueMapper::with_path(file_path);
    mapper.load().unwrap();

    assert_eq!(mapper.get_all_mappings().len(), 10);
    assert_eq!(mapper.get_active_mappings().len(), 7);
    assert_eq!(mapper.get_archived_mappings().len(), 3);

    // Verify specific mappings
    assert!(mapper.get_by_task_id("task-001").is_some());
    assert!(mapper.get_by_issue_number(5).is_some());
    assert!(mapper.get_by_project_item_id("project_item_10").is_some());
}

#[test]
fn test_update_non_existent_mapping() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test-mapping.json");
    let mut mapper = TaskIssueMapper::with_path(file_path);

    let mapping = create_test_mapping("task-999", 999, false);
    assert!(mapper.update_mapping(mapping).is_err());
}

#[test]
fn test_empty_mapper() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test-mapping.json");
    let mapper = TaskIssueMapper::with_path(file_path);

    assert_eq!(mapper.get_all_mappings().len(), 0);
    assert_eq!(mapper.get_active_mappings().len(), 0);
    assert_eq!(mapper.get_archived_mappings().len(), 0);
    assert!(mapper.get_by_task_id("any").is_none());
    assert!(mapper.get_by_issue_number(1).is_none());
    assert!(mapper.get_by_project_item_id("any").is_none());
}
