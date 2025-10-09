use anyhow::Result;
use clap::{Parser, Subcommand};

pub mod task;
pub mod config;
pub mod commands;
pub mod git;
pub mod analysis;

use commands::{init, list, create, validate, sync, lint, ai, update, clean, stats};

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
    },
    /// Show detailed task information
    Show {
        /// Task ID
        task_id: String,
    },
    /// Validate tasks and dependencies
    Validate,
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
    /// Clean old completed tasks and empty directories (mobile optimization)
    Clean {
        /// Dry run - show what would be deleted without actually deleting
        #[arg(long)]
        dry_run: bool,
        /// Number of days to retain completed tasks (default: 30)
        #[arg(short, long)]
        days: Option<u32>,
    },
    /// Show storage statistics and usage breakdown (mobile optimization)
    Stats,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init::run(),
        Commands::List { command, status, area } => match command {
            Some(ListCommands::Items { task_id }) => list::run_items(task_id),
            None => list::run(status, area),
        },
        Commands::Create { title, area, priority } => create::run(title, area, priority),
        Commands::Show { task_id } => {
            println!("Show task: {}", task_id);
            Ok(())
        }
        Commands::Validate => validate::run(),
        Commands::Sync { limit, verbose, remote, dry_run } => sync::run(limit, verbose, remote, dry_run),
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
        Commands::Clean { dry_run, days } => clean::run(dry_run, days),
        Commands::Stats => stats::run(),
    }
}