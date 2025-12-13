use anyhow::{Context, Result};
use chrono::Utc;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::{Config, get_config_path, get_tasks_dir};
use crate::task::{Priority, Task, TaskStatus};

#[derive(Debug, Clone)]
pub struct ImportOptions {
    pub area: Option<String>,
    pub prefix: Option<String>,
    pub dry_run: bool,
    pub start_number: Option<u32>,
    pub tags: Vec<String>,
    pub priority_override: Option<Priority>,
}

#[derive(Debug, Clone)]
struct MarkdownSection {
    section_type: SectionType,
    number: u32,
    title: String,
    content: String,
    priority: Option<Priority>,
    effort: Option<String>,
    location: Option<String>,
    dependencies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)] // Future support for Requirement and Breaking scenarios
enum SectionType {
    Fix,
    Issue,
    Requirement,
    Breaking,
}

impl SectionType {
    fn to_prefix(&self) -> &str {
        match self {
            SectionType::Fix => "fix",
            SectionType::Issue => "issue",
            SectionType::Requirement => "req",
            SectionType::Breaking => "breaking",
        }
    }
}

pub fn run(file_path: PathBuf, options: ImportOptions) -> Result<()> {
    // Read the markdown file
    let content = fs::read_to_string(&file_path)
        .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

    // Parse sections from markdown
    let sections = parse_markdown_sections(&content)?;

    if sections.is_empty() {
        println!(
            "⚠️  No importable sections found in {}",
            file_path.display()
        );
        println!("   Looking for: ## Fix #N, ### Issue #N, ## Breaking Scenarios, etc.");
        return Ok(());
    }

    // Get configuration
    let config_path = get_config_path()?;
    let config = Config::load_or_default(&config_path)?;

    // Determine area
    let area = determine_area(&options, &config, &file_path)?;

    // Determine prefix
    let prefix = determine_prefix(&options, &sections);

    // Generate tasks from sections
    let tasks = sections_to_tasks(sections, &area, &prefix, &options)?;

    if options.dry_run {
        println!("🔍 DRY RUN MODE - No files will be created");
        println!();
        println!("Would create {} tasks:", tasks.len());
        for task in &tasks {
            println!("  {} - {}", task.id, task.title);
            if !task.dependencies.is_empty() {
                println!("    Dependencies: {:?}", task.dependencies);
            }
        }
        return Ok(());
    }

    // Create task files
    let tasks_dir = get_tasks_dir()?;
    let area_dir = tasks_dir.join(&area);
    fs::create_dir_all(&area_dir)
        .with_context(|| format!("Failed to create area directory: {}", area))?;

    let mut created_count = 0;
    for task in tasks {
        let file_path = area_dir.join(task.file_name());

        if file_path.exists() {
            println!("⚠️  Skipping {} (file already exists)", task.id);
            continue;
        }

        let content = task.to_file_content()?;
        fs::write(&file_path, content)
            .with_context(|| format!("Failed to write task file: {}", file_path.display()))?;

        created_count += 1;
        println!("✅ Created: {}", task.id);
    }

    println!();
    println!("📊 Import complete:");
    println!("   Created: {} tasks", created_count);
    println!("   Area: {}", area);
    println!("   Directory: {}", area_dir.display());

    Ok(())
}

fn parse_markdown_sections(content: &str) -> Result<Vec<MarkdownSection>> {
    let mut sections = Vec::new();

    // Regex patterns for different section types
    let fix_pattern = Regex::new(r"^###?\s+Fix\s+#(\d+):\s*(.+)$").unwrap();
    let issue_pattern = Regex::new(r"^###?\s+(?:❌\s+)?Issue\s+#(\d+):\s*(.+)$").unwrap();

    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        // Check for Fix section
        if let Some(caps) = fix_pattern.captures(line) {
            let number: u32 = caps[1].parse().unwrap();
            let title = caps[2].trim().to_string();

            // Extract section content until next heading
            let (section_content, next_index) = extract_section_content(&lines, i + 1);
            let section = parse_section_details(SectionType::Fix, number, title, section_content);

            sections.push(section);
            i = next_index;
            continue;
        }

        // Check for Issue section
        if let Some(caps) = issue_pattern.captures(line) {
            let number: u32 = caps[1].parse().unwrap();
            let title = caps[2].trim().to_string();

            let (section_content, next_index) = extract_section_content(&lines, i + 1);
            let section = parse_section_details(SectionType::Issue, number, title, section_content);

            sections.push(section);
            i = next_index;
            continue;
        }

        i += 1;
    }

    Ok(sections)
}

fn extract_section_content(lines: &[&str], start_index: usize) -> (String, usize) {
    let mut content_lines = Vec::new();
    let mut i = start_index;

    while i < lines.len() {
        let line = lines[i];

        // Stop at next heading of same or higher level
        if line.starts_with("## ") || line.starts_with("### ") {
            // Check if it's another Fix/Issue section
            if line.contains("Fix #") || line.contains("Issue #") {
                break;
            }
        }

        content_lines.push(line);
        i += 1;
    }

    (content_lines.join("\n"), i)
}

fn parse_section_details(
    section_type: SectionType,
    number: u32,
    title: String,
    content: String,
) -> MarkdownSection {
    let mut priority = None;
    let mut effort = None;
    let mut location = None;

    // Extract metadata from content
    let priority_pattern = Regex::new(r"\*\*Priority:\*\*\s+(CRITICAL|HIGH|MEDIUM|LOW)").unwrap();
    let effort_pattern = Regex::new(r"\*\*Effort:\*\*\s+(\d+)\s+hours?").unwrap();
    let location_pattern = Regex::new(r"\*\*Location:\*\*\s+([^\n]+)").unwrap();

    if let Some(caps) = priority_pattern.captures(&content) {
        priority = match caps[1].to_uppercase().as_str() {
            "CRITICAL" => Some(Priority::Critical),
            "HIGH" => Some(Priority::High),
            "MEDIUM" => Some(Priority::Medium),
            "LOW" => Some(Priority::Low),
            _ => None,
        };
    }

    if let Some(caps) = effort_pattern.captures(&content) {
        effort = Some(format!("{}h", &caps[1]));
    }

    if let Some(caps) = location_pattern.captures(&content) {
        location = Some(caps[1].trim().to_string());
    }

    // Extract dependencies
    let dependencies = extract_dependencies(&content);

    MarkdownSection {
        section_type,
        number,
        title,
        content,
        priority,
        effort,
        location,
        dependencies,
    }
}

fn extract_dependencies(content: &str) -> Vec<String> {
    let mut deps = Vec::new();

    // Pre-compile regex patterns outside the loop
    let dep_pattern = Regex::new(
        r"(?i)^\s*(?:\*\*)?(?:depends? on|requires?|blocked by|prerequisites?):?\s*(?:\*\*)?\s+(?:Fix|Issue)\s+#(\d+)"
    ).unwrap();
    let list_pattern =
        Regex::new(r"^\s*(?:\*\*)?Dependencies:?\s*(?:\*\*)?\s*\[([^\]]+)\]").unwrap();

    // Only look in specific sections, not code blocks
    let mut in_code_block = false;

    for line in content.lines() {
        // Track code block boundaries
        if line.trim().starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }

        // Skip lines inside code blocks
        if in_code_block {
            continue;
        }

        // Pattern: "Depends on Fix #N" or "Requires Issue #N" (not in code)
        if let Some(caps) = dep_pattern.captures(line) {
            let dep_num = &caps[1];
            deps.push(format!("#{}", dep_num));
        }

        // Also check for explicit dependencies format: Dependencies: [fix-1, fix-2]
        if let Some(caps) = list_pattern.captures(line) {
            let dep_list = &caps[1];
            for dep in dep_list.split(',') {
                let trimmed = dep.trim();
                if !trimmed.is_empty() {
                    deps.push(trimmed.to_string());
                }
            }
        }
    }

    deps
}

fn determine_area(options: &ImportOptions, config: &Config, file_path: &Path) -> Result<String> {
    if let Some(ref area) = options.area {
        return Ok(area.clone());
    }

    // Try to infer from filename
    if let Some(file_name) = file_path.file_stem().and_then(|s| s.to_str()) {
        let lower = file_name.to_lowercase();

        for area in &config.project.areas {
            if lower.contains(&area.to_lowercase()) {
                return Ok(area.clone());
            }
        }
    }

    // Default to first available area or "import"
    Ok(config
        .project
        .areas
        .first()
        .cloned()
        .unwrap_or_else(|| "import".to_string()))
}

fn determine_prefix(options: &ImportOptions, sections: &[MarkdownSection]) -> String {
    if let Some(ref prefix) = options.prefix {
        return prefix.clone();
    }

    // Check if we have mixed section types (Issues + Fixes)
    if !sections.is_empty() {
        let has_fix = sections.iter().any(|s| s.section_type == SectionType::Fix);
        let has_issue = sections
            .iter()
            .any(|s| s.section_type == SectionType::Issue);

        // If mixed, we'll use type-specific prefixes in task IDs
        // Return empty string to signal this
        if has_fix && has_issue {
            return String::new();
        }

        // Use section type if all sections are the same type
        let first_type = &sections[0].section_type;
        if sections.iter().all(|s| &s.section_type == first_type) {
            return first_type.to_prefix().to_string();
        }
    }

    // Default prefix
    "import".to_string()
}

fn sections_to_tasks(
    sections: Vec<MarkdownSection>,
    area: &str,
    prefix: &str,
    options: &ImportOptions,
) -> Result<Vec<Task>> {
    let mut tasks = Vec::new();
    let mut task_id_map: HashMap<String, String> = HashMap::new();

    // Determine if we need type-specific prefixes
    let use_type_prefixes = prefix.is_empty();

    // First pass: create task IDs
    for section in &sections {
        let section_key = format!("{}-{}", section.section_type.to_prefix(), section.number);
        let task_id = if use_type_prefixes {
            // Use type-specific prefix: fix-001, issue-001, etc.
            format!("{}-{:03}", section.section_type.to_prefix(), section.number)
        } else {
            // Use provided prefix: github-fix-001, etc.
            format!("{}-{:03}", prefix, section.number)
        };
        task_id_map.insert(section_key, task_id);
    }

    // Second pass: create tasks
    for section in sections {
        let section_key = format!("{}-{}", section.section_type.to_prefix(), section.number);
        let task_id = task_id_map.get(&section_key).unwrap().clone();

        // Resolve dependencies from #N format to task-N format
        let dependencies: Vec<String> = section
            .dependencies
            .iter()
            .filter_map(|dep| {
                if let Some(stripped) = dep.strip_prefix('#') {
                    // Extract number from #N
                    if let Ok(num) = stripped.parse::<u32>() {
                        // Try to find matching task - could be fix-N or issue-N
                        // Default to same type as current section
                        let dep_key = format!("{}-{}", section.section_type.to_prefix(), num);
                        task_id_map.get(&dep_key).cloned()
                    } else {
                        None
                    }
                } else {
                    // Already in task-id format
                    Some(dep.clone())
                }
            })
            .collect();

        // Determine priority
        let priority = options
            .priority_override
            .clone()
            .or(section.priority.clone())
            .unwrap_or(Priority::Medium);

        // Build content
        let mut content = section.content.clone();

        // Add location to technical notes if present
        if let Some(ref location) = section.location {
            if !content.contains("## Technical Notes") {
                content.push_str("\n\n## Technical Notes\n");
            }
            content.push_str(&format!("Location: {}\n", location));
        }

        // Convert list items to checkboxes
        content = convert_to_checkboxes(&content);

        // Add tags
        let mut tags = options.tags.clone();
        tags.push(area.to_string());
        tags.push(section.section_type.to_prefix().to_string());

        let task = Task {
            id: task_id,
            title: section.title.clone(),
            status: TaskStatus::Todo,
            priority,
            tags,
            dependencies,
            assignee: Some("developer".to_string()),
            created: Utc::now(),
            estimate: section.effort,
            complexity: estimate_complexity(&section.content),
            area: area.to_string(),
            content,
            file_path: std::path::PathBuf::new(), // Will be set when saved
        };

        tasks.push(task);
    }

    Ok(tasks)
}

fn convert_to_checkboxes(content: &str) -> String {
    let mut result = String::new();

    for line in content.lines() {
        let trimmed = line.trim_start();

        // Convert bullet points to checkboxes if they look like tasks
        if trimmed.starts_with("- ")
            && !trimmed.starts_with("- [ ]")
            && !trimmed.starts_with("- [x]")
        {
            // Check if it looks like an actionable item
            let item_text = &trimmed[2..];
            if is_actionable(item_text) {
                let indent = &line[..line.len() - trimmed.len()];
                result.push_str(&format!("{}[ ] {}\n", indent, item_text));
                continue;
            }
        }

        result.push_str(line);
        result.push('\n');
    }

    result.trim_end().to_string()
}

fn is_actionable(text: &str) -> bool {
    // Simple heuristic: starts with a verb or contains certain keywords
    let action_words = [
        "add",
        "create",
        "update",
        "fix",
        "implement",
        "write",
        "test",
        "check",
        "ensure",
        "verify",
        "remove",
        "delete",
        "modify",
        "install",
        "configure",
        "setup",
        "build",
        "deploy",
    ];

    let lower = text.to_lowercase();
    action_words.iter().any(|word| lower.starts_with(word))
}

fn estimate_complexity(content: &str) -> Option<u8> {
    // Simple heuristic based on content length and structure
    let lines = content.lines().count();
    let code_blocks = content.matches("```").count() / 2;

    let complexity = match (lines, code_blocks) {
        (0..=20, 0..=1) => 2,
        (21..=50, _) => 4,
        (51..=100, _) => 6,
        _ => 8,
    };

    Some(complexity)
}
