use anyhow::Result;
use clap::{Parser, Subcommand};

pub mod task;
pub mod config;
pub mod commands;
pub mod git;
pub mod analysis;
pub mod github;
pub mod templates;

use commands::{init, list, create, validate, sync, lint, ai, update, import_md, clean, stats, archive, compact, restore};

#[derive(Parser)]
#[command(name = "taskguard")]
#[command(about = "AI-ready local task management with Git integration")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum ListCommands {
    /// List checklist items within a task
    Items {
        /// Task ID to list items from
        task_id: String,
    },
}

#[derive(Subcommand)]
enum TaskCommands {
    /// Update a specific checklist item within a task
    Update {
        /// Task ID
        task_id: String,
        /// Item index (1-based)
        item_index: usize,
        /// New status (done or todo)
        status: String,
    },
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize TaskGuard in the current project
    Init,
    /// List tasks or task items
    List {
        #[command(subcommand)]
        command: Option<ListCommands>,
        /// Filter by status (when listing tasks)
        #[arg(short, long)]
        status: Option<String>,
        /// Filter by area (when listing tasks)
        #[arg(short, long)]
        area: Option<String>,
        /// Include archived tasks in the list
        #[arg(long)]
        include_archive: bool,
    },
    /// Create a new task
    Create {
        /// Task title
        #[arg(short, long)]
        title: String,
        /// Task area
        #[arg(short, long)]
        area: Option<String>,
        /// Task priority
        #[arg(short, long)]
        priority: Option<String>,
        /// Task complexity (1-10)
        #[arg(long)]
        complexity: Option<u8>,
        /// Tags (comma-separated)
        #[arg(long)]
        tags: Option<String>,
        /// Dependencies (comma-separated task IDs)
        #[arg(short, long)]
        dependencies: Option<String>,
        /// Assignee
        #[arg(long)]
        assignee: Option<String>,
        /// Time estimate (e.g., "4h", "2d")
        #[arg(short, long)]
        estimate: Option<String>,
    },
    /// Show detailed task information
    Show {
        /// Task ID
        task_id: String,
    },
    /// Validate tasks and dependencies
    Validate {
        /// Sync config areas with task directories
        #[arg(long)]
        sync_areas: bool,
    },
    /// Analyze Git history and suggest task updates
    Sync {
        /// Number of commits to analyze
        #[arg(short, long, default_value = "50")]
        limit: usize,
        /// Show detailed analysis
        #[arg(short, long)]
        verbose: bool,
        /// Fetch and analyze remote repository changes
        #[arg(short, long)]
        remote: bool,
        /// Sync with GitHub Issues and Projects (requires GitHub integration)
        #[arg(long)]
        github: bool,
        /// Add all existing issues to Projects v2 board (GitHub sync only)
        #[arg(long)]
        backfill_project: bool,
        /// Dry run mode - show what would change without applying
        #[arg(long)]
        dry_run: bool,
    },
    /// Analyze task complexity and quality
    Lint {
        /// Show detailed analysis for all tasks
        #[arg(short, long)]
        verbose: bool,
        /// Filter by area
        #[arg(short, long)]
        area: Option<String>,
    },
    /// AI-powered natural language task management
    Ai {
        /// Natural language input for task management
        input: String,
    },
    /// Update task fields (status, priority, assignee, dependencies)
    Update {
        /// Field to update (status, priority, assignee, dependencies)
        field: String,
        /// Task ID to update
        task_id: String,
        /// New value for the field
        value: String,
    },
    /// Task-specific operations (update checklist items)
    Task {
        #[command(subcommand)]
        command: TaskCommands,
    },
    /// Show project status
    Status,
    /// Import tasks from structured markdown file
    ImportMd {
        /// Path to markdown file to import
        file: std::path::PathBuf,
        /// Default area for imported tasks
        #[arg(short, long)]
        area: Option<String>,
        /// Task ID prefix (default: inferred from file)
        #[arg(short, long)]
        prefix: Option<String>,
        /// Show what would be created without creating
        #[arg(long)]
        dry_run: bool,
        /// Starting task number (default: auto-detect)
        #[arg(long)]
        start_number: Option<u32>,
        /// Additional tags (comma-separated)
        #[arg(short, long)]
        tags: Option<String>,
        /// Override inferred priority
        #[arg(long)]
        priority: Option<String>,
    },
    /// Clean old completed tasks and empty directories (efficiency optimization)
    Clean {
        /// Dry run - show what would be deleted without actually deleting
        #[arg(long)]
        dry_run: bool,
        /// Number of days to retain completed tasks (default: 30)
        #[arg(short, long)]
        days: Option<u32>,
    },
    /// Show storage statistics and usage breakdown (efficiency optimization)
    Stats,
    /// Archive old completed tasks to preserve history without bloat (efficiency optimization)
    Archive {
        /// Dry run - show what would be archived without actually moving files
        #[arg(long)]
        dry_run: bool,
        /// Number of days to retain completed tasks (default: 30)
        #[arg(short, long)]
        days: Option<u32>,
    },
    /// Compact task files to reduce storage (efficiency optimization)
    Compact {
        /// Dry run - show what would be compacted without actually modifying files
        #[arg(long)]
        dry_run: bool,
    },
    /// Restore archived task back to active tasks
    Restore {
        /// Task ID to restore from archive
        task_id: String,
        /// Dry run - show what would be restored without actually moving files
        #[arg(long)]
        dry_run: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init::run(),
        Commands::List { command, status, area, include_archive } => match command {
            Some(ListCommands::Items { task_id }) => list::run_items(task_id),
            None => list::run(status, area, include_archive),
        },
        Commands::Create { title, area, priority, complexity, tags, dependencies, assignee, estimate } => create::run(title, area, priority, complexity, tags, dependencies, assignee, estimate),
        Commands::Show { task_id } => {
            println!("Show task: {}", task_id);
            Ok(())
        }
        Commands::Validate { sync_areas } => validate::run(sync_areas),
        Commands::Sync { limit, verbose, remote, github, backfill_project, dry_run } => sync::run(limit, verbose, remote, github, backfill_project, dry_run),
        Commands::Lint { verbose, area } => lint::run(verbose, area),
        Commands::Ai { input } => ai::run(input),
        Commands::Update { field, task_id, value } => update::run(field, task_id, value),
        Commands::Task { command } => match command {
            TaskCommands::Update { task_id, item_index, status } => update::run_task_item(task_id, item_index, status),
        },
        Commands::Status => {
            println!("Project status overview");
            Ok(())
        }
        Commands::ImportMd { file, area, prefix, dry_run, start_number, tags, priority } => {
            let priority_override = match priority.as_deref() {
                Some("low") => Some(crate::task::Priority::Low),
                Some("medium") => Some(crate::task::Priority::Medium),
                Some("high") => Some(crate::task::Priority::High),
                Some("critical") => Some(crate::task::Priority::Critical),
                Some(p) => {
                    println!("⚠️  Invalid priority '{}'. Will use inferred or default.", p);
                    None
                }
                None => None,
            };

            let tag_list = tags
                .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
                .unwrap_or_default();

            let options = import_md::ImportOptions {
                area,
                prefix,
                dry_run,
                start_number,
                tags: tag_list,
                priority_override,
            };

            import_md::run(file, options)
        }
        Commands::Clean { dry_run, days } => clean::run(dry_run, days),
        Commands::Stats => stats::run(),
        Commands::Archive { dry_run, days } => archive::run(dry_run, days),
        Commands::Compact { dry_run } => compact::run(dry_run),
        Commands::Restore { task_id, dry_run } => restore::run(&task_id, dry_run),
    }
}