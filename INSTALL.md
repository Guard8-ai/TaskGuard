# TaskGuard Installation Guide

TaskGuard can be installed globally on your system to work with all your projects. Since this is a private repository, you'll need Git access to Guard8.ai organization.

## Prerequisites

Before installation, ensure you have:
- **Git** with access to Guard8.ai private repositories
- **Rust** 1.70.0+ (https://rustup.rs/)
- Appropriate permissions to clone private repositories

## Quick Installation

### 1. Clone Repository
```bash
git clone https://github.com/Guard8-ai/TaskGuard.git
cd TaskGuard
```

### 2. Run Platform-Specific Script

**Linux:**
```bash
./scripts/install-linux.sh
```

**macOS:**
```bash
./scripts/install-macos.sh
```

**Windows (PowerShell):**
```powershell
.\scripts\install-windows.ps1
```

**WSL/WSL2:**
```bash
./scripts/install-wsl.sh
```

## Manual Installation

### Step-by-Step Process

1. **Prerequisites Installation:**

   **Linux (Ubuntu/Debian):**
   ```bash
   sudo apt update
   sudo apt install git build-essential
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

   **macOS:**
   ```bash
   xcode-select --install  # Installs Git and build tools
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

   **Windows:**
   - Install Git: https://git-scm.com/download/win
   - Install Rust: https://rustup.rs/

   **WSL/WSL2:**
   ```bash
   sudo apt update && sudo apt install -y git build-essential
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Clone and Build:**
   ```bash
   git clone https://github.com/Guard8-ai/TaskGuard.git
   cd TaskGuard
   cargo build --release
   ```

3. **Global Installation:**

   **Linux/WSL:**
   ```bash
   # Use cargo install for consistent installation to ~/.cargo/bin
   cargo install --path . --locked
   # Cargo automatically adds ~/.cargo/bin to PATH
   ```

   **macOS:**
   ```bash
   # Use cargo install for consistent installation to ~/.cargo/bin
   cargo install --path . --locked
   # Cargo automatically adds ~/.cargo/bin to PATH
   ```

   **Windows:**
   ```powershell
   # Use cargo install for consistent installation
   cargo install --path . --locked
   # Cargo automatically adds %USERPROFILE%\.cargo\bin to PATH
   ```

## Installation Script Details

### Linux Installation (`install-linux.sh`)
- **Install Location**: `~/.cargo/bin/`
- **Features**:
  - Dependency verification (Git, Rust 1.70+)
  - Uses `cargo install` for consistent installation
  - Shell detection (bash/zsh/fish)
  - Automatic PATH configuration
  - Clean build and installation process

### macOS Installation (`install-macos.sh`)
- **Install Location**: `~/.cargo/bin/`
- **Features**:
  - macOS version compatibility check
  - Xcode Command Line Tools verification
  - Uses `cargo install` for consistent installation
  - Universal binary support (Intel/Apple Silicon)

### Windows Installation (`install-windows.ps1`)
- **Install Location**: `%USERPROFILE%\.cargo\bin\`
- **Features**:
  - Environment detection (Windows/WSL)
  - Uses `cargo install` for consistent installation
  - Automatic binary type selection
  - User PATH environment updates
  - PowerShell execution policy handling

### WSL Installation (`install-wsl.sh`)
- **Install Location**: `~/.cargo/bin/`
- **Features**:
  - WSL version detection (WSL1/WSL2)
  - Distribution-specific package managers
  - Uses `cargo install` for consistent installation
  - Automatic dependency installation
  - Cross-filesystem optimization

## Verification

Test your installation:

```bash
# Check version
taskguard --version

# Test functionality
cd /path/to/project
taskguard init
taskguard create --title "Test task" --area setup
taskguard list
taskguard validate
```

## Access Requirements

### Git Authentication

Ensure you have access to the Guard8.ai organization:

**SSH (Recommended):**
```bash
# Test SSH access
ssh -T git@github.com

# Clone with SSH
git clone git@github.com:Guard8-ai/TaskGuard.git
```

**HTTPS with Token:**
```bash
# Clone with personal access token
git clone https://YOUR_TOKEN@github.com/Guard8-ai/TaskGuard.git
```

### GitHub CLI Authentication
```bash
# Login with GitHub CLI
gh auth login

# Clone repository
gh repo clone Guard8-ai/TaskGuard
```

## Troubleshooting

### Access Issues

**Permission Denied:**
- Verify Guard8.ai organization membership
- Check SSH key authentication: `ssh -T git@github.com`
- Ensure personal access token has appropriate permissions

**Repository Not Found:**
- Confirm repository URL and organization name
- Verify private repository access permissions
- Check if using correct authentication method

### Build Issues

**Rust Version:**
```bash
rustc --version  # Should be 1.70.0+
rustup update    # Update if needed
```

**Missing Dependencies:**
- Linux: `sudo apt install build-essential`
- macOS: `xcode-select --install`
- Windows: Install Visual Studio Build Tools

### Installation Issues

**Command Not Found:**
1. Restart terminal/shell
2. Check PATH: `echo $PATH`
3. Manually add to PATH if needed

**Permission Errors:**
- Linux/macOS: Use `sudo` for system-wide installation
- Windows: Run PowerShell as Administrator

## Platform-Specific Notes

### Linux
- User installation (no sudo required)
- Works on all major distributions
- Supports multiple shell environments

### macOS
- System-wide installation (/usr/local/bin)
- Requires admin password for installation
- Compatible with Intel and Apple Silicon

### Windows
- User directory installation
- Works with PowerShell, CMD, Windows Terminal
- Compatible with Git for Windows

### WSL/WSL2
- Linux environment within Windows
- Full compatibility with Linux features
- Optimized for cross-filesystem operations

## Uninstalling

Remove TaskGuard:

```bash
# Remove binary using cargo
cargo uninstall taskguard

# Or manually remove
rm ~/.cargo/bin/taskguard                        # Linux/macOS/WSL
del %USERPROFILE%\.cargo\bin\taskguard.exe      # Windows

# Remove project data (optional)
# Delete .taskguard/ and tasks/ directories from projects
```

## Security

TaskGuard has been security-audited with comprehensive testing. See [security-report.md](security-report.md) for details.

## Support

- **Documentation**: [README.md](README.md), [CLAUDE.md](CLAUDE.md)
- **Issues**: Contact Guard8.ai team or repository maintainers
- **Security**: Follow responsible disclosure practices

## Getting Started

After installation, see [README.md Quick Start](README.md#quick-start) to begin using TaskGuard with your projects.