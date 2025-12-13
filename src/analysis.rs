use crate::task::Task;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TaskAnalysis {
    pub task_id: String,
    pub complexity_score: f32,
    pub quality_score: f32,
    pub issues: Vec<LintIssue>,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct LintIssue {
    pub severity: Severity,
    pub category: IssueCategory,
    pub message: String,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IssueCategory {
    Complexity,
    Structure,
    Dependencies,
    Completeness,
    Quality,
}

#[derive(Debug)]
pub struct TaskAnalyzer {
    pub complexity_thresholds: ComplexityThresholds,
}

#[derive(Debug)]
pub struct ComplexityThresholds {
    pub max_content_length: usize,
    pub max_task_items: usize,
    pub max_dependencies: usize,
    pub high_complexity_score: f32,
    pub medium_complexity_score: f32,
}

impl Default for ComplexityThresholds {
    fn default() -> Self {
        Self {
            max_content_length: 2000,
            max_task_items: 20,
            max_dependencies: 5,
            high_complexity_score: 7.0,
            medium_complexity_score: 4.0,
        }
    }
}

impl Default for TaskAnalyzer {
    fn default() -> Self {
        Self {
            complexity_thresholds: ComplexityThresholds::default(),
        }
    }
}

impl TaskAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_thresholds(thresholds: ComplexityThresholds) -> Self {
        Self {
            complexity_thresholds: thresholds,
        }
    }

    pub fn analyze_task(&self, task: &Task) -> TaskAnalysis {
        let mut issues = Vec::new();
        let mut suggestions = Vec::new();

        // Calculate complexity score
        let complexity_score = self.calculate_complexity_score(task);

        // Calculate quality score
        let quality_score = self.calculate_quality_score(task);

        // Check for issues
        self.check_complexity_issues(task, complexity_score, &mut issues, &mut suggestions);
        self.check_structure_issues(task, &mut issues, &mut suggestions);
        self.check_completeness_issues(task, &mut issues, &mut suggestions);
        self.check_dependency_issues(task, &mut issues, &mut suggestions);

        TaskAnalysis {
            task_id: task.id.clone(),
            complexity_score,
            quality_score,
            issues,
            suggestions,
        }
    }

    fn calculate_complexity_score(&self, task: &Task) -> f32 {
        let mut score = 0.0;

        // Content length factor (0-3 points)
        let content_length = task.content.len();
        score += (content_length as f32 / 1000.0).min(3.0);

        // Task items factor (0-4 points)
        let task_items = self.count_task_items(&task.content);
        score += (task_items as f32 / 5.0).min(4.0);

        // Dependencies factor (0-2 points)
        score += (task.dependencies.len() as f32 / 3.0).min(2.0);

        // Estimate factor (0-2 points)
        if let Some(estimate) = &task.estimate {
            score += self.estimate_to_complexity_points(estimate);
        }

        // Manual complexity override
        if let Some(manual_complexity) = task.complexity {
            // Weight manual complexity heavily but don't completely override analysis
            score = (score + manual_complexity as f32) / 2.0;
        }

        score.min(10.0)
    }

    fn calculate_quality_score(&self, task: &Task) -> f32 {
        let mut score: f32 = 10.0;

        // Deduct points for missing information
        if task.estimate.is_none() {
            score -= 1.0;
        }

        if task.content.len() < 100 {
            score -= 2.0; // Very short description
        }

        if task.tags.is_empty() {
            score -= 0.5;
        }

        // Check for acceptance criteria
        if !task.content.contains("Acceptance Criteria") &&
           !task.content.contains("## Objectives") {
            score -= 1.5;
        }

        // Check for task breakdown
        if self.count_task_items(&task.content) == 0 {
            score -= 1.0;
        }

        score.max(0.0)
    }

    pub fn count_task_items(&self, content: &str) -> usize {
        content.lines()
            .filter(|line| {
                let trimmed = line.trim();
                trimmed.starts_with("- [ ]") ||
                trimmed.starts_with("- [x]") ||
                trimmed.starts_with("* [ ]") ||
                trimmed.starts_with("* [x]")
            })
            .count()
    }

    pub fn estimate_to_complexity_points(&self, estimate: &str) -> f32 {
        // Parse common estimate formats and convert to complexity points
        // Conversion: 1 hour = 1 point, 1 day = 8 points, 1 week = 40 points, 1 month = 160 points
        let estimate_lower = estimate.to_lowercase();

        // Check for weeks
        if estimate_lower.contains("week") || estimate_lower.ends_with("w") {
            if let Some(weeks) = self.extract_number(&estimate_lower) {
                return (weeks as f32 * 40.0).min(200.0);
            }
        }

        // Check for months
        if estimate_lower.contains("month") {
            if let Some(months) = self.extract_number(&estimate_lower) {
                return (months as f32 * 160.0).min(200.0);
            }
        }

        // Check for days
        if estimate_lower.contains("day") || (estimate_lower.ends_with("d") && !estimate_lower.contains("h")) {
            if let Some(days) = self.extract_number(&estimate_lower) {
                return (days as f32 * 8.0).min(200.0);
            }
        }

        // Check for hours
        if estimate_lower.contains("hour") || estimate_lower.contains("h") {
            if let Some(hours) = self.extract_number(&estimate_lower) {
                return (hours as f32).min(200.0);
            }
        }

        // Check for minutes
        if estimate_lower.contains("minute") || estimate_lower.contains("m") {
            if let Some(minutes) = self.extract_number(&estimate_lower) {
                return (minutes as f32 / 60.0).min(200.0);
            }
        }

        1.0 // Default complexity for unknown estimates
    }

    fn extract_number(&self, text: &str) -> Option<u32> {
        text.chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()
            .ok()
    }

    fn check_complexity_issues(&self, task: &Task, complexity_score: f32, issues: &mut Vec<LintIssue>, suggestions: &mut Vec<String>) {
        if complexity_score > self.complexity_thresholds.high_complexity_score {
            issues.push(LintIssue {
                severity: Severity::Warning,
                category: IssueCategory::Complexity,
                message: format!("Task has high complexity score: {:.1}/10", complexity_score),
                suggestion: Some("Consider breaking this task into smaller, more focused subtasks".to_string()),
            });
            suggestions.push("Break down into smaller tasks based on natural boundaries (setup, implementation, testing)".to_string());
        }

        if task.content.len() > self.complexity_thresholds.max_content_length {
            issues.push(LintIssue {
                severity: Severity::Info,
                category: IssueCategory::Complexity,
                message: format!("Task description is very long ({} characters)", task.content.len()),
                suggestion: Some("Consider moving detailed specifications to separate documentation".to_string()),
            });
        }

        let task_items = self.count_task_items(&task.content);
        if task_items > self.complexity_thresholds.max_task_items {
            issues.push(LintIssue {
                severity: Severity::Warning,
                category: IssueCategory::Complexity,
                message: format!("Task has many subtasks ({} items)", task_items),
                suggestion: Some("Group related subtasks into separate parent tasks".to_string()),
            });
        }
    }

    fn check_structure_issues(&self, task: &Task, issues: &mut Vec<LintIssue>, suggestions: &mut Vec<String>) {
        let content = &task.content;

        // Check for common sections
        let has_context = content.contains("## Context") || content.contains("# Context");
        let has_objectives = content.contains("## Objectives") || content.contains("# Objectives");
        let has_acceptance = content.contains("Acceptance Criteria");

        if !has_context && content.len() > 500 {
            issues.push(LintIssue {
                severity: Severity::Info,
                category: IssueCategory::Structure,
                message: "Consider adding a ## Context section to explain background".to_string(),
                suggestion: None,
            });
        }

        if !has_objectives && !has_acceptance {
            issues.push(LintIssue {
                severity: Severity::Warning,
                category: IssueCategory::Structure,
                message: "Task lacks clear objectives or acceptance criteria".to_string(),
                suggestion: Some("Add ## Objectives or ## Acceptance Criteria section".to_string()),
            });
            suggestions.push("Define clear success criteria for this task".to_string());
        }
    }

    fn check_completeness_issues(&self, task: &Task, issues: &mut Vec<LintIssue>, _suggestions: &mut Vec<String>) {
        if task.estimate.is_none() {
            issues.push(LintIssue {
                severity: Severity::Info,
                category: IssueCategory::Completeness,
                message: "Task has no time estimate".to_string(),
                suggestion: Some("Add an estimate field to help with planning".to_string()),
            });
        }

        if task.tags.is_empty() {
            issues.push(LintIssue {
                severity: Severity::Info,
                category: IssueCategory::Completeness,
                message: "Task has no tags for categorization".to_string(),
                suggestion: Some("Add relevant tags (e.g., backend, frontend, bug, feature)".to_string()),
            });
        }

        if task.content.len() < 100 {
            issues.push(LintIssue {
                severity: Severity::Warning,
                category: IssueCategory::Completeness,
                message: "Task description is very brief".to_string(),
                suggestion: Some("Add more detail about requirements and context".to_string()),
            });
        }
    }

    fn check_dependency_issues(&self, task: &Task, issues: &mut Vec<LintIssue>, suggestions: &mut Vec<String>) {
        if task.dependencies.len() > self.complexity_thresholds.max_dependencies {
            issues.push(LintIssue {
                severity: Severity::Warning,
                category: IssueCategory::Dependencies,
                message: format!("Task has many dependencies ({})", task.dependencies.len()),
                suggestion: Some("Consider if all dependencies are necessary or if some can be moved".to_string()),
            });
            suggestions.push("Review dependency list and consider creating intermediate tasks".to_string());
        }

        // Check for potential circular dependency patterns
        if task.dependencies.iter().any(|dep| dep.starts_with(&task.area)) {
            issues.push(LintIssue {
                severity: Severity::Info,
                category: IssueCategory::Dependencies,
                message: "Task depends on other tasks in the same area".to_string(),
                suggestion: Some("Verify dependency order within the area is logical".to_string()),
            });
        }
    }

    pub fn analyze_all_tasks(&self, tasks: &[Task]) -> Vec<TaskAnalysis> {
        tasks.iter().map(|task| self.analyze_task(task)).collect()
    }

    pub fn generate_summary(&self, analyses: &[TaskAnalysis]) -> AnalysisSummary {
        let total_tasks = analyses.len();
        let mut high_complexity_count = 0;
        let mut total_issues = 0;
        let mut issues_by_category = HashMap::new();

        for analysis in analyses {
            if analysis.complexity_score > self.complexity_thresholds.high_complexity_score {
                high_complexity_count += 1;
            }

            total_issues += analysis.issues.len();

            for issue in &analysis.issues {
                *issues_by_category.entry(format!("{:?}", issue.category)).or_insert(0) += 1;
            }
        }

        let avg_complexity = analyses.iter()
            .map(|a| a.complexity_score)
            .sum::<f32>() / total_tasks as f32;

        let avg_quality = analyses.iter()
            .map(|a| a.quality_score)
            .sum::<f32>() / total_tasks as f32;

        AnalysisSummary {
            total_tasks,
            high_complexity_count,
            total_issues,
            avg_complexity_score: avg_complexity,
            avg_quality_score: avg_quality,
            issues_by_category,
        }
    }
}

#[derive(Debug)]
pub struct AnalysisSummary {
    pub total_tasks: usize,
    pub high_complexity_count: usize,
    pub total_issues: usize,
    pub avg_complexity_score: f32,
    pub avg_quality_score: f32,
    pub issues_by_category: HashMap<String, usize>,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Info => write!(f, "INFO"),
            Severity::Warning => write!(f, "WARN"),
            Severity::Error => write!(f, "ERROR"),
        }
    }
}