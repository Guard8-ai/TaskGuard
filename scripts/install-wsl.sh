#!/bin/bash

# TaskGuard WSL/WSL2 Installation Script
# This script installs TaskGuard globally on Windows Subsystem for Linux

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
INSTALL_DIR="$HOME/.cargo/bin"
REPO_URL="https://github.com/Guard8-ai/TaskGuard.git"
TEMP_DIR="/tmp/taskguard-install"
BINARY_NAME="taskguard"

# Functions
print_step() {
    echo -e "${BLUE}==>${NC} $1"
}

print_success() {
    echo -e "${GREEN}✅${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠️${NC} $1"
}

print_error() {
    echo -e "${RED}❌${NC} $1"
}

check_wsl() {
    print_step "Checking WSL environment..."

    # Check if we're in WSL
    if [[ ! -f /proc/version ]] || ! grep -qi "microsoft\|wsl" /proc/version 2>/dev/null; then
        print_error "This script is designed for WSL/WSL2 environments"
        echo "For native Linux, use: install-linux.sh"
        echo "For macOS, use: install-macos.sh"
        exit 1
    fi

    # Detect WSL version
    if grep -qi "wsl2" /proc/version 2>/dev/null; then
        WSL_VERSION="WSL2"
    else
        WSL_VERSION="WSL1"
    fi

    print_success "Detected $WSL_VERSION environment"

    # Check distribution
    if [ -f /etc/os-release ]; then
        DISTRO=$(grep "^NAME=" /etc/os-release | cut -d'"' -f2)
        print_success "Distribution: $DISTRO"
    fi
}

update_packages() {
    print_step "Updating package lists..."

    # Detect package manager and update
    if command -v apt &> /dev/null; then
        sudo apt update -qq
        print_success "Package lists updated (apt)"
    elif command -v yum &> /dev/null; then
        sudo yum check-update -q || true
        print_success "Package lists updated (yum)"
    elif command -v pacman &> /dev/null; then
        sudo pacman -Sy --noconfirm
        print_success "Package lists updated (pacman)"
    else
        print_warning "Unknown package manager - skipping package update"
    fi
}

install_dependencies() {
    print_step "Installing dependencies..."

    # Install Git if not present
    if ! command -v git &> /dev/null; then
        print_step "Installing Git..."
        if command -v apt &> /dev/null; then
            sudo apt install -y git
        elif command -v yum &> /dev/null; then
            sudo yum install -y git
        elif command -v pacman &> /dev/null; then
            sudo pacman -S --noconfirm git
        else
            print_error "Cannot install Git - unknown package manager"
            exit 1
        fi
        print_success "Git installed"
    fi

    # Install build essentials if not present
    if command -v apt &> /dev/null; then
        if ! dpkg -l | grep -q build-essential; then
            print_step "Installing build essentials..."
            sudo apt install -y build-essential
            print_success "Build essentials installed"
        fi
    elif command -v yum &> /dev/null; then
        if ! rpm -qa | grep -q "gcc\|make"; then
            print_step "Installing development tools..."
            sudo yum groupinstall -y "Development Tools"
            print_success "Development tools installed"
        fi
    elif command -v pacman &> /dev/null; then
        if ! pacman -Q base-devel &> /dev/null; then
            print_step "Installing base-devel..."
            sudo pacman -S --noconfirm base-devel
            print_success "Base development tools installed"
        fi
    fi
}

check_rust() {
    print_step "Checking Rust installation..."

    if ! command -v cargo &> /dev/null; then
        print_step "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
        print_success "Rust installed"
    fi

    # Check Rust version
    RUST_VERSION=$(rustc --version | cut -d' ' -f2)
    REQUIRED_VERSION="1.70.0"
    if [ "$(printf '%s\n' "$REQUIRED_VERSION" "$RUST_VERSION" | sort -V | head -n1)" != "$REQUIRED_VERSION" ]; then
        print_warning "Updating Rust to meet version requirements..."
        rustup update
        RUST_VERSION=$(rustc --version | cut -d' ' -f2)
    fi

    print_success "Rust version: $RUST_VERSION"
}

create_install_dir() {
    print_step "Creating installation directory..."
    mkdir -p "$INSTALL_DIR"
    print_success "Installation directory ready: $INSTALL_DIR"
}

clone_and_install() {
    print_step "Cloning TaskGuard repository..."

    # Clean up any existing temp directory
    rm -rf "$TEMP_DIR"

    # Clone the repository
    git clone "$REPO_URL" "$TEMP_DIR"
    cd "$TEMP_DIR"

    print_success "Repository cloned"

    print_step "Building and installing TaskGuard (this may take a few minutes)..."

    # Ensure we have the latest Rust environment
    source "$HOME/.cargo/env" 2>/dev/null || true

    # Use cargo install for consistent installation to ~/.cargo/bin
    cargo install --path . --locked

    print_success "TaskGuard installed to $INSTALL_DIR/$BINARY_NAME"
}

setup_path() {
    print_step "Setting up PATH..."

    # Check if install directory is in PATH
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        print_warning "$INSTALL_DIR is not in your PATH"

        # Detect shell and add to appropriate config file
        SHELL_CONFIG=""
        if [ -n "$BASH_VERSION" ]; then
            SHELL_CONFIG="$HOME/.bashrc"
        elif [ -n "$ZSH_VERSION" ]; then
            SHELL_CONFIG="$HOME/.zshrc"
        else
            # Try to detect from $SHELL
            case "$SHELL" in
                */bash)
                    SHELL_CONFIG="$HOME/.bashrc"
                    ;;
                */zsh)
                    SHELL_CONFIG="$HOME/.zshrc"
                    ;;
                */fish)
                    SHELL_CONFIG="$HOME/.config/fish/config.fish"
                    print_warning "Fish shell detected. Please manually add: set -gx PATH $INSTALL_DIR \$PATH"
                    return
                    ;;
                *)
                    print_warning "Unknown shell. Please manually add $INSTALL_DIR to your PATH"
                    return
                    ;;
            esac
        fi

        if [ -n "$SHELL_CONFIG" ]; then
            # Also add cargo environment
            echo "" >> "$SHELL_CONFIG"
            echo "# TaskGuard installation" >> "$SHELL_CONFIG"
            echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$SHELL_CONFIG"
            echo "source \"\$HOME/.cargo/env\" 2>/dev/null || true" >> "$SHELL_CONFIG"
            print_success "Added $INSTALL_DIR to PATH in $SHELL_CONFIG"
            print_warning "Please restart your shell or run: source $SHELL_CONFIG"
        fi
    else
        print_success "PATH is already configured"
    fi
}

cleanup() {
    print_step "Cleaning up..."
    rm -rf "$TEMP_DIR"
    print_success "Cleanup completed"
}

verify_installation() {
    print_step "Verifying installation..."

    # Ensure cargo environment is available
    source "$HOME/.cargo/env" 2>/dev/null || true

    # Test if binary is accessible
    if command -v "$BINARY_NAME" &> /dev/null; then
        VERSION=$($BINARY_NAME --version 2>/dev/null || echo "version check failed")
        print_success "TaskGuard installed successfully!"
        echo "Version: $VERSION"
    else
        # Try direct path
        if [ -x "$INSTALL_DIR/$BINARY_NAME" ]; then
            VERSION=$("$INSTALL_DIR/$BINARY_NAME" --version 2>/dev/null || echo "version check failed")
            print_success "TaskGuard installed successfully!"
            echo "Version: $VERSION"
            print_warning "Binary is at $INSTALL_DIR/$BINARY_NAME (may need to restart shell for PATH)"
        else
            print_error "Installation verification failed"
            exit 1
        fi
    fi
}

print_usage_info() {
    echo ""
    echo -e "${GREEN}🎉 TaskGuard Installation Complete!${NC}"
    echo ""
    echo "WSL-Specific Notes:"
    echo "  - TaskGuard is now available in your WSL environment"
    echo "  - Works with Git repositories in WSL filesystem (/home/...)"
    echo "  - Also works with Windows filesystem (/mnt/c/...)"
    echo "  - Best performance with repositories in WSL filesystem"
    echo ""
    echo "Quick Start:"
    echo "  1. Restart your shell or run: source ~/.bashrc (or ~/.zshrc)"
    echo "  2. Navigate to any project directory"
    echo "  3. Run: taskguard init"
    echo "  4. Create your first task: taskguard create --title \"Setup project\" --area setup"
    echo "  5. List tasks: taskguard list"
    echo "  6. Check dependencies: taskguard validate"
    echo ""
    echo "WSL Integration Tips:"
    echo "  - Access Windows files: cd /mnt/c/Users/YourName/Projects"
    echo "  - Keep repositories in WSL for best performance: ~/projects/"
    echo "  - TaskGuard works in both environments seamlessly"
    echo ""
    echo "For more information, visit: https://github.com/Guard8-ai/TaskGuard"
    echo ""
}

# Main installation process
main() {
    echo -e "${BLUE}TaskGuard WSL/WSL2 Installation Script${NC}"
    echo "This will install TaskGuard globally in your WSL environment"
    echo ""

    check_wsl
    update_packages
    install_dependencies
    check_rust
    create_install_dir
    clone_and_install
    setup_path
    cleanup
    verify_installation
    print_usage_info
}

# Run main function
main "$@"