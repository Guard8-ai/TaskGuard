# Installation

Install TaskGuard globally to use with all your projects.

---

## Quick Install (Recommended)

Use platform-specific scripts for automated installation:

=== "macOS"

    ```bash
    # Clone repository
    git clone git@github.com:Guard8-ai/TaskGuard.git
    cd TaskGuard

    # Run install script
    ./scripts/install-macos.sh
    ```

=== "Linux"

    ```bash
    # Clone repository
    git clone git@github.com:Guard8-ai/TaskGuard.git
    cd TaskGuard

    # Run install script
    ./scripts/install-linux.sh
    ```

=== "Windows"

    ```powershell
    # Clone repository
    git clone git@github.com:Guard8-ai/TaskGuard.git
    cd TaskGuard

    # Run install script
    .\scripts\install-windows.ps1
    ```

=== "WSL"

    ```bash
    # Clone repository
    git clone git@github.com:Guard8-ai/TaskGuard.git
    cd TaskGuard

    # Run install script
    ./scripts/install-wsl.sh
    ```

**Installation Location:** `~/.cargo/bin/taskguard`

**What the script does:**
1. Checks prerequisites (Git, Rust 1.70+)
2. Builds release binary (`cargo build --release`)
3. Installs globally (`cargo install --path . --locked`)
4. Verifies installation

**Time:** ~2-3 minutes (depending on build speed)

---

## Manual Installation

### Step-by-Step

1. **Clone Repository**

    ```bash
    git clone git@github.com:Guard8-ai/TaskGuard.git
    cd TaskGuard
    ```

2. **Build Release Binary**

    ```bash
    cargo build --release
    ```

    **Output:**
    ```
    Compiling taskguard v0.2.2
    Finished release [optimized] target(s) in 1m 23s
    ```

3. **Install Globally**

    ```bash
    cargo install --path . --locked
    ```

    **Installation path:** `~/.cargo/bin/taskguard`

4. **Verify Installation**

    ```bash
    taskguard --version
    ```

    **Expected:**
    ```
    taskguard 0.2.2
    ```

---

## Verification

### Test Commands

```bash
# Check version
taskguard --version

# Show help
taskguard --help

# Test in a project
cd ~/test-project
taskguard init
```

**Expected output from `taskguard init`:**
```
🚀 Initializing TaskGuard...
📝 Created example task: tasks/setup/001-project-setup.md
✅ TaskGuard initialized successfully!

📁 Created directories:
   .taskguard/         # Configuration and state
   tasks/              # Task files organized by area
   tasks/setup/
   tasks/backend/
   tasks/frontend/
   tasks/api/
   tasks/auth/
   tasks/testing/
```

---

## Troubleshooting

### Command Not Found

**Issue:**
```bash
taskguard: command not found
```

**Solution:**

1. **Check installation:**
    ```bash
    ls -la ~/.cargo/bin/taskguard
    ```

2. **Verify PATH:**
    ```bash
    echo $PATH | grep -q ".cargo/bin" && echo "✅ In PATH" || echo "❌ Not in PATH"
    ```

3. **Add to PATH:**
    ```bash
    # Bash
    echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
    source ~/.bashrc

    # Zsh
    echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
    source ~/.zshrc
    ```

4. **Restart terminal**

### Build Errors

**Issue:**
```
error: failed to compile taskguard
```

**Solutions:**

1. **Update Rust:**
    ```bash
    rustup update
    rustc --version  # Should be 1.70+
    ```

2. **Clean build:**
    ```bash
    cargo clean
    cargo build --release
    ```

3. **Check dependencies:**
    ```bash
    # Linux: Install build essentials
    sudo apt install build-essential

    # macOS: Install Xcode tools
    xcode-select --install
    ```

### Permission Errors

**Issue:**
```
Permission denied
```

**Solution:**

```bash
# Make script executable
chmod +x scripts/install-*.sh

# Or use cargo directly (no sudo needed)
cargo install --path . --locked
```

### Repository Access Issues

**Issue:**
```
fatal: repository not found
```

**Solutions:**

1. **Check SSH access:**
    ```bash
    ssh -T git@github.com
    ```

2. **Verify organization membership:**
    - Ensure you're part of Guard8-ai organization
    - Check repository permissions

3. **Use HTTPS with token:**
    ```bash
    git clone https://YOUR_TOKEN@github.com/Guard8-ai/TaskGuard.git
    ```

---

## Platform-Specific Notes

### macOS

- **Requires:** Xcode Command Line Tools
- **Install location:** `~/.cargo/bin/`
- **Universal binary:** Works on Intel & Apple Silicon
- **Permissions:** User-level (no admin required)

### Linux

- **Requires:** `build-essential` (Ubuntu/Debian) or equivalent
- **Install location:** `~/.cargo/bin/`
- **Supported distros:** Ubuntu, Debian, Fedora, Arch, etc.
- **Permissions:** User-level (no sudo required)

### Windows

- **Requires:** Git for Windows, Rust
- **Install location:** `%USERPROFILE%\.cargo\bin\`
- **PowerShell:** May need execution policy adjustment
- **Compatibility:** Works with PowerShell, CMD, Windows Terminal

### WSL/WSL2

- **Fully compatible** with Linux installation
- **Performance:** WSL2 recommended for faster builds
- **Cross-filesystem:** Works across Windows/Linux filesystems
- **Install location:** `~/.cargo/bin/`

---

## Updating TaskGuard

To update to the latest version:

```bash
cd TaskGuard
git pull origin main
cargo install --path . --locked --force
```

**Verify update:**
```bash
taskguard --version
```

---

## Uninstalling

To remove TaskGuard:

```bash
# Remove binary
cargo uninstall taskguard

# Or manually
rm ~/.cargo/bin/taskguard                        # Linux/macOS
del %USERPROFILE%\.cargo\bin\taskguard.exe      # Windows

# Optional: Remove project data
# Delete .taskguard/ and tasks/ from your projects
```

---

## Next Steps

Installation complete! Now:

→ [Create Your First Task](first-task.md)
