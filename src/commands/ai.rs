use crate::analysis::TaskAnalyzer;
use crate::config::load_all_tasks;
use crate::git::GitAnalyzer;
use crate::task::{Priority, Task, TaskStatus};
use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::path::Path;

#[derive(Debug)]
pub struct ValidationResult {
    pub available_tasks: Vec<Task>,
    pub blocked_tasks: Vec<(Task, Vec<String>)>,
}

pub struct AIAgent {
    #[allow(dead_code)] // Reserved for future Git-based analysis features
    git_analyzer: Option<GitAnalyzer>,
    task_analyzer: TaskAnalyzer,
}

impl AIAgent {
    pub fn new() -> Result<Self> {
        let git_analyzer = match GitAnalyzer::new(Path::new(".")) {
            Ok(analyzer) => Some(analyzer),
            Err(_) => None, // Git analysis is optional
        };

        let task_analyzer = TaskAnalyzer::new();

        Ok(Self {
            git_analyzer,
            task_analyzer,
        })
    }

    fn validate_tasks(&self, tasks: &[Task]) -> Result<ValidationResult> {
        let mut task_map: HashMap<String, &Task> = HashMap::new();
        for task in tasks {
            task_map.insert(task.id.clone(), task);
        }

        let completed_task_ids: HashSet<String> = tasks
            .iter()
            .filter(|task| matches!(task.status, TaskStatus::Done))
            .map(|task| task.id.clone())
            .collect();

        let mut available_tasks = Vec::new();
        let mut blocked_tasks = Vec::new();

        for task in tasks {
            if matches!(task.status, TaskStatus::Done) {
                continue; // Skip completed tasks
            }

            let mut blocking_deps = Vec::new();
            for dep_id in &task.dependencies {
                // A dependency blocks if it's not completed
                // This includes both existing incomplete tasks AND missing tasks
                if !completed_task_ids.contains(dep_id) {
                    blocking_deps.push(dep_id.clone());
                }
            }

            if blocking_deps.is_empty() {
                available_tasks.push(task.clone());
            } else {
                blocked_tasks.push((task.clone(), blocking_deps));
            }
        }

        Ok(ValidationResult {
            available_tasks,
            blocked_tasks,
        })
    }

    pub fn process_natural_language(&self, input: &str) -> Result<String> {
        let input_lower = input.to_lowercase();

        // Pattern matching for common natural language patterns
        // Order matters! More specific patterns should be checked first
        if self.is_completion_announcement(&input_lower) {
            self.handle_task_completion(input)
        } else if self.is_dependency_query(&input_lower) {
            // Check dependencies BEFORE status (both may match "what tasks")
            self.handle_dependency_analysis()
        } else if self.is_complexity_query(&input_lower) {
            self.handle_complexity_analysis()
        } else if self.is_status_inquiry(&input_lower) {
            self.handle_status_inquiry()
        } else if self.is_next_task_request(&input_lower) {
            self.handle_next_task_recommendation()
        } else if self.is_task_creation_request(&input_lower) {
            self.handle_task_creation(input)
        } else {
            self.handle_general_guidance(input)
        }
    }

    fn is_task_creation_request(&self, input: &str) -> bool {
        let creation_patterns = [
            "create a task",
            "add a task",
            "new task",
            "this is an",
            "this is a",
            "i need to",
            "we should",
            "create an",
            "create a",
            "add an",
            "add a",
            "add auth",
            "build a",
            "write a",
            "write test",
            "set up",
        ];

        creation_patterns
            .iter()
            .any(|pattern| input.contains(pattern))
    }

    fn is_status_inquiry(&self, input: &str) -> bool {
        // Avoid matching "show me X" where X is more specific (like "show me complexity")
        let status_patterns = [
            "what's the status",
            "what's the current status",
            "current status",
            "show me the status",
            "show me status",
            "show me the tasks",
            "show me all",
            "what tasks are available",
            "which tasks are available",
            "what tasks",
            "list tasks",
            "list what",
            "list all",
            "current state",
            "project status",
            "overview",
            "give me an overview",
        ];

        status_patterns
            .iter()
            .any(|pattern| input.contains(pattern))
    }

    fn is_next_task_request(&self, input: &str) -> bool {
        let next_patterns = [
            "what should i work on",
            "what's next",
            "what can i do",
            "what's available",  // "What's available?" - singular
            "what is available", // without "tasks"
            "ready to work",
            "recommend",
        ];

        // More specific check: if it contains "what tasks", it's probably status inquiry
        if input.contains("what tasks") || input.contains("which tasks") {
            return false;
        }

        next_patterns.iter().any(|pattern| input.contains(pattern))
    }

    fn is_completion_announcement(&self, input: &str) -> bool {
        let completion_patterns = [
            "finished",
            "completed",
            "done with",
            "just finished",
            "implemented",
            "built",
        ];

        completion_patterns
            .iter()
            .any(|pattern| input.contains(pattern))
    }

    fn is_dependency_query(&self, input: &str) -> bool {
        let dependency_patterns = [
            "dependencies",
            "depends on",
            "tasks are blocked",
            "tasks blocked",
            "what tasks are blocked",
            "which tasks are blocked",
            "what's blocked",
            "what is blocked",
            "show me dependencies",
            "waiting for",
            "waiting for other",
            "depend on others",
            "prerequisite",
        ];

        dependency_patterns
            .iter()
            .any(|pattern| input.contains(pattern))
    }

    fn is_complexity_query(&self, input: &str) -> bool {
        let complexity_patterns = [
            "complexity",
            "how complex",
            "task complexity",
            "how hard",
            "difficult",
            "difficulty",
            "estimate",
            "effort",
            "analyze",
        ];

        complexity_patterns
            .iter()
            .any(|pattern| input.contains(pattern))
    }

    fn handle_task_creation(&self, input: &str) -> Result<String> {
        // Extract task information from natural language
        let title = self.extract_task_title(input);
        let area = self.infer_task_area(input);
        let priority = self.infer_priority(input);

        let response = format!(
            "I'll help you create a task. Based on your request:\n\n\
            📝 **Suggested Task:**\n\
            • Title: {}\n\
            • Area: {}\n\
            • Priority: {}\n\n\
            💡 **To create this task, run:**\n\
            `taskguard create --title \"{}\" --area {} --priority {}`\n\n\
            🤔 **Need adjustments?** Let me know if you'd like to modify any details!",
            title, area, priority, title, area, priority
        );

        Ok(response)
    }

    fn handle_status_inquiry(&self) -> Result<String> {
        let tasks = load_all_tasks()?;
        let validation_result = self.validate_tasks(&tasks)?;

        let total_tasks = tasks.len();
        let available_tasks = validation_result.available_tasks.len();
        let blocked_tasks = validation_result.blocked_tasks.len();

        let mut response = format!(
            "📊 **Project Status Overview**\n\n\
            • Total tasks: {}\n\
            • Available to work on: {}\n\
            • Blocked by dependencies: {}\n\n",
            total_tasks, available_tasks, blocked_tasks
        );

        if !validation_result.available_tasks.is_empty() {
            response.push_str("✅ **Ready to work on:**\n");
            for task in validation_result.available_tasks.iter().take(3) {
                response.push_str(&format!(
                    "   • {} - {} ({:?})\n",
                    task.id, task.title, task.priority
                ));
            }
            if validation_result.available_tasks.len() > 3 {
                response.push_str(&format!(
                    "   • ... and {} more\n",
                    validation_result.available_tasks.len() - 3
                ));
            }
        }

        Ok(response)
    }

    fn handle_next_task_recommendation(&self) -> Result<String> {
        let tasks = load_all_tasks()?;
        let validation_result = self.validate_tasks(&tasks)?;

        if validation_result.available_tasks.is_empty() {
            return Ok("🤔 **No tasks are currently available to work on.**\n\nAll tasks may be blocked by dependencies. Run `taskguard validate` to see what's blocking progress.".to_string());
        }

        // Prioritize tasks by priority and complexity
        let mut available = validation_result.available_tasks;
        available.sort_by(|a, b| {
            // Sort by priority first (high > medium > low), then by complexity (lower first)
            let priority_order = |p: &Priority| match p {
                Priority::Critical => 4,
                Priority::High => 3,
                Priority::Medium => 2,
                Priority::Low => 1,
            };

            let a_priority = priority_order(&a.priority);
            let b_priority = priority_order(&b.priority);

            b_priority
                .cmp(&a_priority)
                .then_with(|| a.complexity.unwrap_or(5).cmp(&b.complexity.unwrap_or(5)))
        });

        let recommended = &available[0];

        let mut response = format!(
            "🎯 **Recommended Next Task** ({} available):\n\n\
            **{}** - {}\n\
            • Priority: {:?}\n\
            • Complexity: {}/10\n\
            • Area: {}\n",
            available.len(),
            recommended.id,
            recommended.title,
            recommended.priority,
            recommended.complexity.unwrap_or(5),
            recommended.area
        );

        if let Some(estimate) = &recommended.estimate {
            response.push_str(&format!("• Estimated time: {}\n", estimate));
        }

        response.push_str(&format!(
            "\n💡 **To get started:**\n\
            `taskguard show {}`\n\n",
            recommended.id
        ));

        if available.len() > 1 {
            response.push_str("🔄 **Other available options:**\n");
            for task in available.iter().skip(1).take(2) {
                response.push_str(&format!(
                    "   • {} - {} ({:?})\n",
                    task.id, task.title, task.priority
                ));
            }
        }

        Ok(response)
    }

    fn handle_task_completion(&self, input: &str) -> Result<String> {
        // Try to extract task ID or area from the completion message
        let potential_task = self.extract_completed_task_reference(input);

        let mut response = String::from("🎉 **Great job on completing that work!**\n\n");

        if let Some(task_ref) = potential_task {
            response.push_str(&format!(
                "It sounds like you finished work on: **{}**\n\n",
                task_ref
            ));
        }

        response.push_str(
            "📋 **Next steps:**\n\
            1. Update the task status to `done` in the task file\n\
            2. Run `taskguard validate` to see what's now available\n\
            3. Consider running `taskguard sync` to analyze your Git commits\n\n\
            🚀 **Ready for more?** Ask me \"what should I work on next?\" for recommendations!",
        );

        Ok(response)
    }

    fn handle_dependency_analysis(&self) -> Result<String> {
        let tasks = load_all_tasks()?;
        let validation_result = self.validate_tasks(&tasks)?;

        let mut response = String::from("🔗 **Dependency Analysis**\n\n");

        if !validation_result.blocked_tasks.is_empty() {
            response.push_str("🚫 **Blocked tasks:**\n");
            for (task, blocking_deps) in &validation_result.blocked_tasks {
                response.push_str(&format!(
                    "   • **{}** - {}\n     Waiting for: {}\n\n",
                    task.id,
                    task.title,
                    blocking_deps.join(", ")
                ));
            }
        }

        if !validation_result.available_tasks.is_empty() {
            response.push_str("✅ **Ready to work on (no blocking dependencies):**\n");
            for task in &validation_result.available_tasks {
                response.push_str(&format!("   • **{}** - {}\n", task.id, task.title));
            }
        }

        Ok(response)
    }

    fn handle_complexity_analysis(&self) -> Result<String> {
        let tasks = load_all_tasks()?;
        let _complexity_results = self.task_analyzer.analyze_all_tasks(&tasks);

        let mut response = String::from("🧠 **Complexity Analysis**\n\n");

        // Group tasks by complexity
        let mut by_complexity: std::collections::HashMap<u8, Vec<_>> =
            std::collections::HashMap::new();
        for task in &tasks {
            by_complexity
                .entry(task.complexity.unwrap_or(5))
                .or_default()
                .push(task);
        }

        for complexity in [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] {
            if let Some(task_list) = by_complexity.get(&complexity) {
                if !task_list.is_empty() {
                    let emoji = match complexity {
                        1..=3 => "🟢",
                        4..=6 => "🟡",
                        7..=8 => "🟠",
                        9..=10 => "🔴",
                        _ => "⚪",
                    };
                    response.push_str(&format!("{} **Complexity {}:**\n", emoji, complexity));
                    for task in task_list {
                        response.push_str(&format!("   • {} - {}\n", task.id, task.title));
                    }
                    response.push('\n');
                }
            }
        }

        response.push_str("💡 **Tip:** Start with lower complexity tasks to build momentum!");

        Ok(response)
    }

    fn handle_general_guidance(&self, input: &str) -> Result<String> {
        let guidance = format!(
            "🤖 **TaskGuard AI Assistant**\n\n\
            I can help you with:\n\n\
            📝 **Task Management:**\n\
            • \"Create a task for user authentication\" → Task creation guidance\n\
            • \"What should I work on next?\" → Smart recommendations\n\
            • \"Show me the current status\" → Project overview\n\n\
            🔗 **Dependencies:**\n\
            • \"What's blocked?\" → Dependency analysis\n\
            • \"What can I work on?\" → Available tasks\n\n\
            🧠 **Analysis:**\n\
            • \"How complex are my tasks?\" → Complexity breakdown\n\
            • \"I finished the auth work\" → Next steps guidance\n\n\
            📊 **Quick Commands:**\n\
            • `taskguard validate` - Check dependencies\n\
            • `taskguard sync` - Analyze Git history\n\
            • `taskguard lint` - Task quality analysis\n\n\
            💬 **Your input:** \"{}\"\n\
            Try rephrasing with one of the patterns above, or ask for specific help!",
            input.trim()
        );

        Ok(guidance)
    }

    // Helper methods for natural language processing

    fn extract_task_title(&self, input: &str) -> String {
        // Look for patterns like "create a task for X" or "add X"
        let patterns = [
            r"(?i)create.*?(?:task|item).*?for\s+(.+?)(?:\s+(?:in|with|using)|\.|$)",
            r"(?i)add.*?(?:new\s+)?feature\s+for\s+(.+?)(?:\s+(?:in|with|using)|\.|$)",
            r"(?i)add.*?(?:task|item).*?(?:for|to)\s+(.+?)(?:\s+(?:in|with|using)|\.|$)",
            r"(?i)(?:i\s+need\s+to|we\s+should)\s+(?:build|create|implement|write)\s+(?:a|an)?\s*(.+?)(?:\?|\.|$)",
            r"(?i)(?:build|create|implement|write)\s+(?:a|an)\s+(.+?)(?:\s+(?:for|component|feature)|\?|\.|$)",
            r"(?i)create\s+(?:a|an)\s+(.+?)(?:\s+(?:feature|functionality|component|for)|\.|$)",
            r"(?i)(?:task|item).*?(?:for|to)\s+(.+?)(?:\.|$)",
            r"(?i)(?:fix|resolve)\s+(?:the\s+)?(.+?)(?:\s+bug|\?|\.|$)",
        ];

        for pattern in &patterns {
            if let Ok(re) = regex::Regex::new(pattern) {
                if let Some(captures) = re.captures(input) {
                    if let Some(title) = captures.get(1) {
                        let extracted = title.as_str().trim().to_string();
                        // Don't return empty or very short titles
                        if extracted.len() > 2 {
                            return extracted;
                        }
                    }
                }
            }
        }

        // Fallback: extract the most likely title from the input
        let words: Vec<&str> = input.split_whitespace().collect();
        if words.len() > 2 {
            words[2..].join(" ")
        } else {
            "New Task".to_string()
        }
    }

    fn infer_task_area(&self, input: &str) -> String {
        let input_lower = input.to_lowercase();

        // Check for more specific patterns first
        // Order matters: more specific keywords should come before generic ones
        let area_keywords = [
            (
                "testing",
                vec![
                    "write tests",
                    "write test",
                    "testing",
                    "test for",
                    "test the",
                    "spec",
                    "validation",
                ],
            ),
            (
                "frontend",
                vec![
                    "react",
                    "vue",
                    "angular",
                    "frontend",
                    "component for",
                    "component",
                    "ui",
                    "view",
                    "page",
                ],
            ),
            (
                "auth",
                vec![
                    "authentication",
                    "auth to",
                    "auth for",
                    "login",
                    "security",
                    "permission",
                ],
            ),
            (
                "setup",
                vec![
                    "set up",
                    "setup",
                    "configuration",
                    "config",
                    "install",
                    "deployment",
                    "database configuration",
                ],
            ),
            (
                "backend",
                vec!["api", "endpoint", "server", "database", "backend"],
            ),
        ];

        for (area, keywords) in &area_keywords {
            for keyword in keywords {
                if input_lower.contains(keyword) {
                    return area.to_string();
                }
            }
        }

        "setup".to_string() // default
    }

    fn infer_priority(&self, input: &str) -> String {
        let input_lower = input.to_lowercase();

        if input_lower.contains("critical")
            || input_lower.contains("urgent")
            || input_lower.contains("asap")
        {
            "critical".to_string()
        } else if input_lower.contains("high")
            || input_lower.contains("important")
            || input_lower.contains("priority")
        {
            "high".to_string()
        } else if input_lower.contains("low")
            || input_lower.contains("minor")
            || input_lower.contains("later")
        {
            "low".to_string()
        } else {
            "medium".to_string()
        }
    }

    fn extract_completed_task_reference(&self, input: &str) -> Option<String> {
        // Look for task IDs, areas, or general task references
        let patterns = [
            r"(?i)(?:finished|completed|done with)\s+([a-z]+-\d+)",
            r"(?i)(?:finished|completed|done with)\s+(?:the\s+)?([a-z]+)\s+(?:work|task|feature)",
            r"(?i)(?:finished|completed|done with)\s+(.+?)(?:\s+(?:task|work|feature)|\.|$)",
        ];

        for pattern in &patterns {
            if let Ok(re) = regex::Regex::new(pattern) {
                if let Some(captures) = re.captures(input) {
                    if let Some(reference) = captures.get(1) {
                        return Some(reference.as_str().trim().to_string());
                    }
                }
            }
        }

        None
    }
}

pub fn run(input: String) -> Result<()> {
    let ai_agent = AIAgent::new()?;
    let response = ai_agent.process_natural_language(&input)?;
    println!("{}", response);
    Ok(())
}
