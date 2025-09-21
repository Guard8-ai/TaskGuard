use anyhow::Result;
use clap::{Parser, Subcommand};

pub mod task;
pub mod config;
pub mod commands;
pub mod git;
pub mod analysis;

use commands::{init, list, create, validate, sync, lint, ai};

#[derive(Parser)]
#[command(name = "taskguard")]
#[command(about = "AI-ready local task management with Git integration")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize TaskGuard in the current project
    Init,
    /// List all tasks
    List {
        /// Filter by status
        #[arg(short, long)]
        status: Option<String>,
        /// Filter by area
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
    /// Show project status
    Status,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init::run(),
        Commands::List { status, area } => list::run(status, area),
        Commands::Create { title, area, priority } => create::run(title, area, priority),
        Commands::Show { task_id } => {
            println!("Show task: {}", task_id);
            Ok(())
        }
        Commands::Validate => validate::run(),
        Commands::Sync { limit, verbose } => sync::run(limit, verbose),
        Commands::Lint { verbose, area } => lint::run(verbose, area),
        Commands::Ai { input } => ai::run(input),
        Commands::Status => {
            println!("Project status overview");
            Ok(())
        }
    }
}