#!/bin/bash

# TaskGuard Linux Installation Script
# This script installs TaskGuard globally on Linux systems

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

check_dependencies() {
    print_step "Checking dependencies..."

    # Check for Git
    if ! command -v git &> /dev/null; then
        print_error "Git is required but not installed. Please install Git first."
        exit 1
    fi

    # Check for Rust/Cargo
    if ! command -v cargo &> /dev/null; then
        print_error "Rust/Cargo is required but not installed."
        echo "Please install Rust from: https://rustup.rs/"
        exit 1
    fi

    # Check Rust version
    RUST_VERSION=$(rustc --version | cut -d' ' -f2)
    REQUIRED_VERSION="1.70.0"
    if [ "$(printf '%s\n' "$REQUIRED_VERSION" "$RUST_VERSION" | sort -V | head -n1)" != "$REQUIRED_VERSION" ]; then
        print_error "Rust version $REQUIRED_VERSION or higher is required. Found: $RUST_VERSION"
        echo "Please update Rust: rustup update"
        exit 1
    fi

    print_success "All dependencies satisfied"
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
            echo "" >> "$SHELL_CONFIG"
            echo "# TaskGuard installation" >> "$SHELL_CONFIG"
            echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$SHELL_CONFIG"
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
    echo "Quick Start:"
    echo "  1. Navigate to any project directory"
    echo "  2. Run: taskguard init"
    echo "  3. Create your first task: taskguard create --title \"Setup project\" --area setup"
    echo "  4. List tasks: taskguard list"
    echo "  5. Check dependencies: taskguard validate"
    echo ""
    echo "For more information, visit: https://github.com/Guard8-ai/TaskGuard"
    echo ""
}

# Main installation process
main() {
    echo -e "${BLUE}TaskGuard Linux Installation Script${NC}"
    echo "This will install TaskGuard globally on your system"
    echo ""

    check_dependencies
    create_install_dir
    clone_and_install
    setup_path
    cleanup
    verify_installation
    print_usage_info
}

# Run main function
main "$@"