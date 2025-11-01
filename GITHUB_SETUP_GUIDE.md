# GitHub Projects v2 Setup for TaskGuard

## Quick Setup (2 minutes)

TaskGuard now features **zero-config GitHub integration** with automatic project creation!

### 1. Install GitHub CLI

Download from: https://cli.github.com/

Or install via package manager:
```bash
# macOS
brew install gh

# Ubuntu/Debian
curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | sudo dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg
echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null
sudo apt update
sudo apt install gh

# Windows
winget install --id GitHub.cli
```

### 2. Authenticate

```bash
gh auth login
```

Follow the prompts in your browser to authenticate.

### 3. Add Project Scope

```bash
gh auth refresh -s project --hostname github.com
```

Click "Authorize" in your browser when prompted. This grants TaskGuard permission to create and manage Projects v2.

### 4. Sync Your First Task (Automatic Setup!)

```bash
taskguard sync backend-001 --github --yes
```

That's it! The `--yes` flag enables **fully automatic setup**. TaskGuard will:
- ✅ Detect if GitHub project exists
- ✅ Auto-create a GitHub Projects v2 board if needed
- ✅ Link the project to your repository
- ✅ Configure default status columns (Todo, In Progress, Done)
- ✅ Save configuration to `.taskguard/github.toml`
- ✅ Create the GitHub issue for your task
- ✅ Add the issue to the project board
- ✅ Set the correct status column

**Example output:**
```
✨ Creating GitHub Projects board...
✓ Detected organization: Guard8-ai
✓ Created project: "TaskGuard Tasks" (#1)
✓ Linked to repository: Guard8-ai/TaskGuard
✓ Configured status columns: Todo, In Progress, Done
✓ Saved configuration to .taskguard/github.toml

🚀 Project ready! View at: https://github.com/orgs/Guard8-ai/projects/1

Now syncing task backend-001...
✅ Created issue #123: Implement backend feature
✅ Added to project board
✅ Status: In Progress
```

## View Your Tasks

Your project board will be at:
```
https://github.com/orgs/YourOrg/projects/1
```

Or for personal repos:
```
https://github.com/users/YourUsername/projects/1
```

## Troubleshooting

### "authentication token is missing required scopes"
```bash
gh auth refresh -s project --hostname github.com
```

### "gh: command not found"
Install GitHub CLI: https://cli.github.com/

## Using Without --yes Flag

If you prefer to see what will happen before TaskGuard creates anything, omit the `--yes` flag:

```bash
taskguard sync backend-001 --github
```

TaskGuard will show you a preview and instructions:
```
⚠️  No GitHub Projects board found for this repository.

   TaskGuard can create one with:
   - Name: "TaskGuard Tasks"
   - Default columns: Todo, In Progress, In Review, Done
   - Linked to: Guard8-ai/TaskGuard

   Run with --yes to create automatically:
   $ taskguard sync backend-001 --github --yes
```

## For AI Agents and Automation

The `--yes` flag was specifically designed for **AI coding assistants** and **CI/CD automation**:

```bash
# Claude Code, GitHub Copilot, etc.
taskguard sync backend-001 --github --yes

# CI/CD pipelines
taskguard sync ${TASK_ID} --github --yes
```

**Why `--yes` is important for AI agents:**
- ✅ No interactive prompts (won't block waiting for input)
- ✅ Non-blocking I/O (perfect for automation)
- ✅ Predictable behavior (same result every time)
- ✅ Works in headless environments

## Manual Configuration (Optional)

After your first sync, TaskGuard creates `.taskguard/github.toml`:

```toml
owner = "Guard8-ai"
repo = "TaskGuard"
project_number = 1
```

You can manually edit this file if needed, but the auto-setup handles everything for most users.

## Advanced: Manual Project Creation

If you prefer to create the GitHub project manually:

1. Go to https://github.com/orgs/YourOrg/projects (or /users/YourUsername/projects)
2. Click "New project"
3. Choose "Board" template
4. Name it "TaskGuard Tasks"
5. Link it to your repository
6. Note the project number from the URL
7. Create `.taskguard/github.toml` with your settings

Then sync without `--yes`:
```bash
taskguard sync backend-001 --github
```

## That's All!

TaskGuard handles everything else automatically with the `--yes` flag. No manual project creation, no config file editing, no setup hassles.

**Just three commands and you're syncing:**
```bash
gh auth login
gh auth refresh -s project
taskguard sync backend-001 --github --yes
```
