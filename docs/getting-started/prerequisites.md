# Prerequisites

Before installing TaskGuard, ensure you have the required tools installed on your system.

---

## System Requirements

TaskGuard is a lightweight CLI tool with minimal dependencies:

- **Operating System:** Linux, macOS, Windows, or WSL
- **Disk Space:** ~10MB for binary
- **Memory:** Minimal (runs in <50MB RAM)

---

## Required Software

### 1. Git

**Why:** TaskGuard uses Git for version control and history analysis.

**Installation:**

=== "macOS"

    ```bash
    # Install via Xcode Command Line Tools
    xcode-select --install

    # Or via Homebrew
    brew install git
    ```

=== "Linux"

    ```bash
    # Ubuntu/Debian
    sudo apt update && sudo apt install git

    # Fedora
    sudo dnf install git

    # Arch
    sudo pacman -S git
    ```

=== "Windows"

    Download from [git-scm.com](https://git-scm.com/download/win)

**Verify:**
```bash
git --version
# Expected: git version 2.30+
```

### 2. Rust (1.70+)

**Why:** TaskGuard is built in Rust and requires the Rust toolchain for installation.

**Installation:**

Visit [rustup.rs](https://rustup.rs/) or run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**After installation:**
```bash
source ~/.cargo/env  # Linux/macOS
# Restart terminal on Windows
```

**Verify:**
```bash
rustc --version
# Expected: rustc 1.70.0+

cargo --version
# Expected: cargo 1.70.0+
```

---

## Optional Requirements

### GitHub Access (Private Repository)

TaskGuard is currently a private repository in the Guard8.ai organization.

**SSH Setup (Recommended):**

```bash
# Generate SSH key
ssh-keygen -t ed25519 -C "your_email@example.com"

# Add to SSH agent
eval "$(ssh-agent -s)"
ssh-add ~/.ssh/id_ed25519

# Copy public key
cat ~/.ssh/id_ed25519.pub
```

Add the public key to GitHub: Settings → SSH Keys

**Test SSH connection:**
```bash
ssh -T git@github.com
# Expected: Hi username! You've successfully authenticated...
```

**Alternative: Personal Access Token**

Create a token at GitHub Settings → Developer → Personal Access Tokens

---

## Environment Setup

### Shell Configuration

Ensure `~/.cargo/bin` is in your PATH:

```bash
# Check PATH
echo $PATH | grep -q ".cargo/bin" && echo "✅ Cargo bin in PATH" || echo "❌ Not in PATH"
```

**If not in PATH, add to shell config:**

=== "Bash"

    ```bash
    echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
    source ~/.bashrc
    ```

=== "Zsh"

    ```bash
    echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
    source ~/.zshrc
    ```

=== "Fish"

    ```bash
    set -U fish_user_paths $HOME/.cargo/bin $fish_user_paths
    ```

---

## Verification Checklist

Before proceeding to installation, verify:

- [ ] Git installed and working
- [ ] Rust 1.70+ installed
- [ ] Cargo in PATH
- [ ] GitHub SSH access configured (for private repo)
- [ ] Terminal/shell properly configured

**Quick verification:**
```bash
git --version && rustc --version && cargo --version
```

**Expected output:**
```
git version 2.39.0
rustc 1.75.0
cargo 1.75.0
```

---

## Next Steps

Once prerequisites are met:

→ [Install TaskGuard](installation.md)
