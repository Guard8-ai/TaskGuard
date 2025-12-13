use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use git2::Repository;

use crate::config::{get_tasks_dir, find_taskguard_root};
use crate::task::Task;
use crate::github::{
    GitHubClient, GitHubMutations,
    TaskIssueMapper, is_github_sync_enabled,
};

/// Restore archived task back to active tasks
pub fn run(task_id: &str, dry_run: bool) -> Result<()> {
    let root = find_taskguard_root()
        .context("Not in a TaskGuard project")?;
    let archive_dir = root.join(".taskguard").join("archive");
    let tasks_dir = get_tasks_dir()?;

    if !archive_dir.exists() {
        anyhow::bail!("❌ No archive directory found. No tasks have been archived yet.");
    }

    println!("📦 RESTORE ARCHIVED TASK");
    if dry_run {
        println!("   Mode: DRY RUN (no files will be moved)");
    }
    println!();

    // Check for GitHub integration
    let github_enabled = is_github_sync_enabled()?;
    let mut mapper = if github_enabled {
        Some(TaskIssueMapper::new()?)
    } else {
        None
    };

    // Find archived task
    let (archived_path, task) = find_archived_task(&archive_dir, task_id)?;
    let area = &task.area;

    println!("   Task: {} - {}", task.id, task.title);
    println!("   Area: {}", area);
    println!("   From: {}", archived_path.display());

    let restore_dir = tasks_dir.join(area);
    let restore_path = restore_dir.join(archived_path.file_name().unwrap());
    println!("   To: {}", restore_path.display());
    println!();

    // Check if task has GitHub issue
    let github_issue_to_reopen = if let Some(ref mapper) = mapper {
        mapper.get_by_task_id(task_id).map(|mapping| {
            (
                mapping.issue_number,
                mapping.issue_id.clone(),
                mapping.is_archived,
            )
        })
    } else {
        None
    };

    // Display GitHub integration info
    if let Some((issue_num, _, is_archived)) = &github_issue_to_reopen {
        println!("🌐 GITHUB INTEGRATION");
        if *is_archived {
            println!("   Task is synced with GitHub Issue #{}", issue_num);
            println!("   Issue will be reopened on restore");
        } else {
            println!("   ⚠️  Task is synced with Issue #{} but not marked as archived", issue_num);
            println!("   Issue state will be synchronized");
        }
        println!();
    }

    if dry_run {
        println!("🔍 DRY RUN - No files moved");
        println!("   Run without --dry-run to actually restore");
        return Ok(());
    }

    // Create area directory if it doesn't exist
    fs::create_dir_all(&restore_dir)
        .context("Failed to create area directory")?;

    // Check if target already exists
    if restore_path.exists() {
        anyhow::bail!(
            "❌ Cannot restore: File already exists at {}\n   Please remove or rename the existing file first.",
            restore_path.display()
        );
    }

    // Move file back to tasks/
    fs::rename(&archived_path, &restore_path)
        .context("Failed to restore task file")?;

    println!("✅ RESTORE COMPLETE");
    println!("   Task restored: {} → {}/{}", task_id, area, archived_path.file_name().unwrap().to_string_lossy());
    println!("   Location: {}", restore_path.display());

    // Reopen GitHub issue if enabled
    if github_enabled && github_issue_to_reopen.is_some() {
        println!();
        match reopen_github_issue(task_id, &github_issue_to_reopen, &mut mapper) {
            Ok(()) => {
                println!("🌐 GITHUB SYNC COMPLETE");
                if let Some((issue_num, _, _)) = github_issue_to_reopen {
                    println!("   Reopened GitHub Issue #{}", issue_num);
                }
            }
            Err(e) => {
                eprintln!("\n⚠️  Warning: Failed to reopen GitHub issue: {}", e);
                eprintln!("   Task was restored successfully, but GitHub sync may be incomplete.");
                eprintln!("   You may need to manually reopen the issue or sync again.");
            }
        }
    }

    // Create Git commit for tracking
    if let Err(e) = create_restore_commit(&root, task_id) {
        eprintln!("\n⚠️  Warning: Failed to create Git commit: {}", e);
        eprintln!("   Task was restored successfully, but Git tracking may be incomplete.");
    }

    Ok(())
}

/// Find an archived task by ID
fn find_archived_task(archive_dir: &Path, task_id: &str) -> Result<(PathBuf, Task)> {
    for entry in WalkDir::new(archive_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
    {
        let path = entry.path();

        match Task::from_file(path) {
            Ok(task) => {
                if task.id == task_id {
                    return Ok((path.to_path_buf(), task));
                }
            }
            Err(_) => continue,
        }
    }

    Err(anyhow::anyhow!(
        "❌ Task not found: {}\n   No archived task found with this ID.\n   Use 'taskguard list --include-archive' to see all archived tasks.",
        task_id
    ))
}

/// Reopen GitHub issue for restored task and update mapping
fn reopen_github_issue(
    task_id: &str,
    issue_info: &Option<(i64, String, bool)>,
    mapper: &mut Option<TaskIssueMapper>,
) -> Result<()> {
    if let Some((issue_num, issue_id, _)) = issue_info {
        let client = GitHubClient::new()?;

        println!("   🌐 Reopening GitHub issue #{} for {}", issue_num, task_id);

        match GitHubMutations::update_issue_state(&client, issue_id, "OPEN") {
            Ok(_) => {
                println!("      ✅ Reopened issue #{}", issue_num);

                // Update mapping to mark as not archived
                if let Some(m) = mapper {
                    if let Some(mut mapping) = m.get_by_task_id(task_id).cloned() {
                        mapping.is_archived = false;
                        m.update_mapping(mapping)?;
                    }
                }
                Ok(())
            }
            Err(e) => {
                Err(anyhow::anyhow!("Failed to reopen issue #{}: {}", issue_num, e))
            }
        }
    } else {
        Ok(())
    }
}

/// Create a Git commit to track restored task
fn create_restore_commit(repo_path: &Path, task_id: &str) -> Result<()> {
    let repo = Repository::open(repo_path)
        .context("Failed to open Git repository")?;

    if repo.is_bare() {
        return Err(anyhow::anyhow!("Cannot commit in a bare repository"));
    }

    let mut index = repo.index()
        .context("Failed to get repository index")?;

    // Stage all changes (restored file in tasks/ and removed from archive/)
    index.add_all(["."].iter(), git2::IndexAddOption::DEFAULT, None)
        .context("Failed to stage changes")?;

    index.update_all(["."].iter(), None)
        .context("Failed to update index")?;

    index.write()
        .context("Failed to write index")?;

    let tree_id = index.write_tree()
        .context("Failed to write tree")?;
    let tree = repo.find_tree(tree_id)
        .context("Failed to find tree")?;

    // Get HEAD commit as parent
    let head = repo.head()
        .context("Failed to get HEAD")?;
    let parent_commit = head.peel_to_commit()
        .context("Failed to get parent commit")?;

    // Get signature for commit
    let signature = repo.signature()
        .context("Failed to get Git signature")?;

    // Create commit message
    let commit_message = format!("Restore archived task: {}", task_id);

    // Create the commit
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        &commit_message,
        &tree,
        &[&parent_commit],
    ).context("Failed to create commit")?;

    println!("\n📝 Git commit created:");
    println!("   Message: {}", commit_message);

    Ok(())
}
