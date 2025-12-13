use anyhow::{Context, Result};
use std::path::Path;
use walkdir::WalkDir;

use crate::analysis::{Severity, TaskAnalyzer};
use crate::config::find_taskguard_root;
use crate::task::Task;

pub fn run(verbose: bool, area: Option<String>) -> Result<()> {
    let taskguard_root =
        find_taskguard_root().context("Not in a TaskGuard project. Run 'taskguard init' first.")?;

    let tasks_dir = taskguard_root.join("tasks");

    if !tasks_dir.exists() {
        println!("📋 No tasks directory found. Run 'taskguard init' to set up the project.");
        return Ok(());
    }

    // Load all tasks
    let mut tasks = Vec::new();
    let mut parse_errors = 0;

    for entry in WalkDir::new(&tasks_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file()
            && entry.path().extension().and_then(|s| s.to_str()) == Some("md")
        {
            // Filter by area if specified
            if let Some(ref filter_area) = area {
                if let Some(area_dir) = entry
                    .path()
                    .parent()
                    .and_then(|p| p.file_name())
                    .and_then(|n| n.to_str())
                {
                    if area_dir != filter_area {
                        continue;
                    }
                }
            }

            match Task::from_file(entry.path()) {
                Ok(task) => tasks.push(task),
                Err(_) => {
                    parse_errors += 1;
                    if verbose {
                        println!("⚠️  Skipping {}: Failed to parse", entry.path().display());
                    }
                }
            }
        }
    }

    if tasks.is_empty() && parse_errors == 0 {
        println!("📋 No task files found in {}", tasks_dir.display());
        return Ok(());
    }

    // Analyze all tasks
    let analyzer = TaskAnalyzer::new();
    let analyses = analyzer.analyze_all_tasks(&tasks);
    let summary = analyzer.generate_summary(&analyses);

    // Print header
    println!("🔍 TASK ANALYSIS REPORT");
    println!("   ═══════════════════════");

    // Print summary
    println!();
    println!("📊 SUMMARY");
    println!("   Total tasks analyzed: {}", summary.total_tasks);
    println!(
        "   Average complexity: {:.1}/10",
        summary.avg_complexity_score
    );
    println!("   Average quality: {:.1}/10", summary.avg_quality_score);
    println!(
        "   High complexity tasks: {}",
        summary.high_complexity_count
    );
    println!("   Total issues found: {}", summary.total_issues);

    if parse_errors > 0 {
        println!("   Parse errors: {}", parse_errors);
    }

    // Print issues by category
    if !summary.issues_by_category.is_empty() {
        println!();
        println!("📋 ISSUES BY CATEGORY");
        for (category, count) in &summary.issues_by_category {
            println!("   {}: {}", category, count);
        }
    }

    // Print detailed analysis
    let mut error_count = 0;
    let mut warning_count = 0;
    let mut info_count = 0;

    for analysis in &analyses {
        if analysis.issues.is_empty() && !verbose {
            continue;
        }

        println!();
        println!(
            "📁 {} (Complexity: {:.1}, Quality: {:.1})",
            analysis.task_id, analysis.complexity_score, analysis.quality_score
        );

        if analysis.issues.is_empty() {
            if verbose {
                println!("   ✅ No issues found");
            }
            continue;
        }

        // Group issues by severity
        for issue in &analysis.issues {
            let icon = match issue.severity {
                Severity::Error => {
                    error_count += 1;
                    "❌"
                }
                Severity::Warning => {
                    warning_count += 1;
                    "⚠️ "
                }
                Severity::Info => {
                    info_count += 1;
                    "ℹ️ "
                }
            };

            println!("   {} [{}] {}", icon, issue.severity, issue.message);

            if let Some(ref suggestion) = issue.suggestion {
                println!("      💡 {}", suggestion);
            }
        }

        // Print suggestions if any
        if !analysis.suggestions.is_empty() && verbose {
            println!("   🎯 Suggestions:");
            for suggestion in &analysis.suggestions {
                println!("      • {}", suggestion);
            }
        }
    }

    // Print final summary
    println!();
    println!("🏁 LINT SUMMARY");
    if error_count + warning_count + info_count == 0 {
        println!("   ✅ All tasks look good!");
    } else {
        if error_count > 0 {
            println!("   ❌ Errors: {}", error_count);
        }
        if warning_count > 0 {
            println!("   ⚠️  Warnings: {}", warning_count);
        }
        if info_count > 0 {
            println!("   ℹ️  Info: {}", info_count);
        }
    }

    // Recommendations
    if summary.high_complexity_count > 0 {
        println!();
        println!("💡 RECOMMENDATIONS");
        println!(
            "   Consider breaking down {} high-complexity tasks",
            summary.high_complexity_count
        );
        if summary.avg_complexity_score > 6.0 {
            println!("   Overall task complexity is high - focus on smaller, more focused tasks");
        }
    }

    Ok(())
}

pub fn run_single_task<P: AsRef<Path>>(task_path: P, _verbose: bool) -> Result<()> {
    let task = Task::from_file(&task_path)
        .with_context(|| format!("Failed to load task from {}", task_path.as_ref().display()))?;

    let analyzer = TaskAnalyzer::new();
    let analysis = analyzer.analyze_task(&task);

    println!("🔍 TASK ANALYSIS: {}", task.id);
    println!("   ═══════════════════════");
    println!();
    println!("📊 SCORES");
    println!("   Complexity: {:.1}/10", analysis.complexity_score);
    println!("   Quality: {:.1}/10", analysis.quality_score);

    if analysis.issues.is_empty() {
        println!();
        println!("✅ No issues found");
        return Ok(());
    }

    println!();
    println!("📋 ISSUES");

    for issue in &analysis.issues {
        let icon = match issue.severity {
            Severity::Error => "❌",
            Severity::Warning => "⚠️ ",
            Severity::Info => "ℹ️ ",
        };

        println!(
            "   {} [{}] [{}] {}",
            icon,
            issue.severity,
            format!("{:?}", issue.category).to_uppercase(),
            issue.message
        );

        if let Some(ref suggestion) = issue.suggestion {
            println!("      💡 {}", suggestion);
        }
    }

    if !analysis.suggestions.is_empty() {
        println!();
        println!("🎯 SUGGESTIONS");
        for suggestion in &analysis.suggestions {
            println!("   • {}", suggestion);
        }
    }

    Ok(())
}
