# TaskGuard Windows/WSL Installation Script
# This script installs TaskGuard globally on Windows (WSL/WSL2) systems

param(
    [switch]$Force,
    [string]$InstallPath = "$env:USERPROFILE\.cargo\bin"
)

# Configuration
$RepoUrl = "https://github.com/Guard8-ai/TaskGuard.git"
$TempDir = "$env:TEMP\taskguard-install"
$BinaryName = "taskguard.exe"

# Colors for output (if supported)
$ColorSupport = $Host.UI.RawUI.ForegroundColor -ne $null

function Write-Step {
    param([string]$Message)
    if ($ColorSupport) {
        Write-Host "==> $Message" -ForegroundColor Blue
    } else {
        Write-Host "==> $Message"
    }
}

function Write-Success {
    param([string]$Message)
    if ($ColorSupport) {
        Write-Host "✅ $Message" -ForegroundColor Green
    } else {
        Write-Host "[SUCCESS] $Message"
    }
}

function Write-Warning {
    param([string]$Message)
    if ($ColorSupport) {
        Write-Host "⚠️ $Message" -ForegroundColor Yellow
    } else {
        Write-Host "[WARNING] $Message"
    }
}

function Write-Error {
    param([string]$Message)
    if ($ColorSupport) {
        Write-Host "❌ $Message" -ForegroundColor Red
    } else {
        Write-Host "[ERROR] $Message"
    }
}

function Test-IsWSL {
    return $env:WSL_DISTRO_NAME -ne $null -or (Test-Path "/proc/version" -ErrorAction SilentlyContinue)
}

function Test-Dependencies {
    Write-Step "Checking dependencies..."

    # Check for Git
    try {
        $gitVersion = git --version
        Write-Success "Git found: $gitVersion"
    }
    catch {
        Write-Error "Git is required but not installed."
        if (Test-IsWSL) {
            Write-Host "In WSL, install Git with: sudo apt update && sudo apt install git"
        } else {
            Write-Host "Please install Git from: https://git-scm.com/download/win"
        }
        exit 1
    }

    # Check for Rust/Cargo
    try {
        $cargoVersion = cargo --version
        Write-Success "Cargo found: $cargoVersion"
    }
    catch {
        Write-Error "Rust/Cargo is required but not installed."
        Write-Host "Please install Rust from: https://rustup.rs/"
        if (Test-IsWSL) {
            Write-Host "In WSL: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        } else {
            Write-Host "Download rustup-init.exe from the website above"
        }
        exit 1
    }

    # Check Rust version
    $rustVersion = (rustc --version).Split(' ')[1]
    $requiredVersion = [version]"1.70.0"
    $currentVersion = [version]$rustVersion

    if ($currentVersion -lt $requiredVersion) {
        Write-Error "Rust version 1.70.0 or higher is required. Found: $rustVersion"
        Write-Host "Please update Rust: rustup update"
        exit 1
    }

    Write-Success "All dependencies satisfied"
}

function New-InstallDirectory {
    Write-Step "Creating installation directory..."

    if (-not (Test-Path $InstallPath)) {
        New-Item -ItemType Directory -Path $InstallPath -Force | Out-Null
    }

    Write-Success "Installation directory ready: $InstallPath"
}

function Invoke-CloneAndBuild {
    Write-Step "Cloning TaskGuard repository..."

    # Clean up any existing temp directory
    if (Test-Path $TempDir) {
        Remove-Item -Recurse -Force $TempDir
    }

    # Clone the repository
    git clone $RepoUrl $TempDir
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Failed to clone repository"
        exit 1
    }

    Set-Location $TempDir
    Write-Success "Repository cloned"

    Write-Step "Building TaskGuard (this may take a few minutes)..."

    # Build for Windows
    if (Test-IsWSL) {
        # In WSL, build Linux binary
        cargo build --release
        $script:BinaryName = "taskguard"
    } else {
        # In Windows, build Windows binary
        cargo build --release
        $script:BinaryName = "taskguard.exe"
    }

    if ($LASTEXITCODE -ne 0) {
        Write-Error "Build failed"
        exit 1
    }

    Write-Success "Build completed"
}

function Install-Binary {
    Write-Step "Installing TaskGuard binary..."

    $sourcePath = Join-Path $TempDir "target\release\$BinaryName"
    $destPath = Join-Path $InstallPath $BinaryName

    if (-not (Test-Path $sourcePath)) {
        Write-Error "Built binary not found at: $sourcePath"
        exit 1
    }

    Copy-Item $sourcePath $destPath -Force
    Write-Success "Binary installed to $destPath"
}

function Set-EnvironmentPath {
    Write-Step "Setting up PATH..."

    # Check if install directory is in PATH
    $currentPath = $env:PATH
    if ($currentPath -notlike "*$InstallPath*") {
        Write-Warning "$InstallPath is not in your PATH"

        if (Test-IsWSL) {
            # In WSL, update shell profile
            $shellProfile = if ($env:SHELL -like "*zsh*") { "$env:HOME/.zshrc" } else { "$env:HOME/.bashrc" }

            Add-Content -Path $shellProfile -Value ""
            Add-Content -Path $shellProfile -Value "# TaskGuard installation"
            Add-Content -Path $shellProfile -Value "export PATH=`"$InstallPath:`$PATH`""

            Write-Success "Added $InstallPath to PATH in $shellProfile"
            Write-Warning "Please restart your shell or run: source $shellProfile"
        } else {
            # In Windows, update user PATH
            $userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
            if ($userPath -notlike "*$InstallPath*") {
                $newPath = if ($userPath) { "$userPath;$InstallPath" } else { $InstallPath }
                [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
                Write-Success "Added $InstallPath to user PATH"
                Write-Warning "Please restart your terminal/PowerShell for PATH changes to take effect"
            }
        }
    } else {
        Write-Success "PATH is already configured"
    }
}

function Remove-TempFiles {
    Write-Step "Cleaning up..."

    Set-Location $env:USERPROFILE
    if (Test-Path $TempDir) {
        Remove-Item -Recurse -Force $TempDir
    }

    Write-Success "Cleanup completed"
}

function Test-Installation {
    Write-Step "Verifying installation..."

    $binaryPath = Join-Path $InstallPath $BinaryName

    if (Test-Path $binaryPath) {
        try {
            if (Test-IsWSL) {
                $version = & $binaryPath --version 2>$null
            } else {
                $version = & $binaryPath --version 2>$null
            }
            Write-Success "TaskGuard installed successfully!"
            Write-Host "Version: $version"
        }
        catch {
            Write-Success "TaskGuard binary installed successfully!"
            Write-Warning "Version check failed, but binary exists at $binaryPath"
        }
    } else {
        Write-Error "Installation verification failed - binary not found"
        exit 1
    }
}

function Show-UsageInfo {
    Write-Host ""
    if ($ColorSupport) {
        Write-Host "🎉 TaskGuard Installation Complete!" -ForegroundColor Green
    } else {
        Write-Host "TaskGuard Installation Complete!"
    }
    Write-Host ""
    Write-Host "Quick Start:"
    Write-Host "  1. Open a new terminal window"
    Write-Host "  2. Navigate to any project directory"
    Write-Host "  3. Run: taskguard init"
    Write-Host "  4. Create your first task: taskguard create --title `"Setup project`" --area setup"
    Write-Host "  5. List tasks: taskguard list"
    Write-Host "  6. Check dependencies: taskguard validate"
    Write-Host ""

    if (Test-IsWSL) {
        Write-Host "WSL Notes:"
        Write-Host "  - TaskGuard is now available in your WSL environment"
        Write-Host "  - Works with all your Git repositories in WSL"
    } else {
        Write-Host "Windows Notes:"
        Write-Host "  - Use PowerShell or Command Prompt"
        Write-Host "  - Works with Git repositories on Windows"
    }

    Write-Host ""
    Write-Host "For more information, visit: https://github.com/Guard8-ai/TaskGuard"
    Write-Host ""
}

# Main installation process
function Main {
    if ($ColorSupport) {
        Write-Host "TaskGuard Windows/WSL Installation Script" -ForegroundColor Blue
    } else {
        Write-Host "TaskGuard Windows/WSL Installation Script"
    }

    if (Test-IsWSL) {
        Write-Host "Detected WSL environment - installing Linux version"
    } else {
        Write-Host "Detected Windows environment - installing Windows version"
    }
    Write-Host "This will install TaskGuard globally on your system"
    Write-Host ""

    Test-Dependencies
    New-InstallDirectory
    Invoke-CloneAndBuild
    Install-Binary
    Set-EnvironmentPath
    Remove-TempFiles
    Test-Installation
    Show-UsageInfo
}

# Run main function
try {
    Main
}
catch {
    Write-Error "Installation failed: $_"
    exit 1
}