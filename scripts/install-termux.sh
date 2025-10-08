#!/data/data/com.termux/files/usr/bin/bash

# TaskGuard Termux (Android) Installation Script
# This script installs TaskGuard globally on Termux/Android systems

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
TEMP_DIR="$PREFIX/tmp/taskguard-install"
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

detect_termux() {
    print_step "Detecting Termux environment..."

    if [ -z "$PREFIX" ]; then
        print_error "Not running in Termux environment. PREFIX variable not set."
        exit 1
    fi

    if [ ! -d "$PREFIX" ]; then
        print_error "Termux PREFIX directory not found: $PREFIX"
        exit 1
    fi

    print_success "Termux environment detected: $PREFIX"
}

check_dependencies() {
    print_step "Checking dependencies..."

    # Check for Git
    if ! command -v git &> /dev/null; then
        print_warning "Git not found. Installing Git..."
        pkg install git -y
        print_success "Git installed"
    else
        print_success "Git is already installed"
    fi

    # Check for Rust/Cargo
    if ! command -v cargo &> /dev/null; then
        print_warning "Rust/Cargo not found. Installing Rust..."
        pkg install rust -y
        print_success "Rust installed"

        # Source cargo environment
        if [ -f "$HOME/.cargo/env" ]; then
            source "$HOME/.cargo/env"
        fi
    else
        print_success "Rust is already installed"
    fi

    # Verify Rust installation
    if ! command -v cargo &> /dev/null; then
        print_error "Rust installation failed. Please install manually: pkg install rust"
        exit 1
    fi

    # Check Rust version
    RUST_VERSION=$(rustc --version | cut -d' ' -f2)
    REQUIRED_VERSION="1.70.0"

    # Version comparison for Termux
    if [ "$(printf '%s\n' "$REQUIRED_VERSION" "$RUST_VERSION" | sort -V | head -n1)" != "$REQUIRED_VERSION" ]; then
        print_warning "Rust version $REQUIRED_VERSION+ recommended. Found: $RUST_VERSION"
        print_warning "Attempting to update Rust..."
        pkg upgrade rust -y || print_warning "Update failed, continuing with current version"
    fi

    print_success "All dependencies satisfied"
}

check_storage() {
    print_step "Checking available storage..."

    # Check available storage in $HOME
    AVAILABLE=$(df -h "$HOME" | tail -1 | awk '{print $4}')
    print_success "Available storage: $AVAILABLE"

    # Warn if low storage
    AVAILABLE_MB=$(df -m "$HOME" | tail -1 | awk '{print $4}')
    if [ "$AVAILABLE_MB" -lt 500 ]; then
        print_warning "Low storage space. TaskGuard build requires ~300MB temporarily"
        read -p "Continue? (y/n) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 1
        fi
    fi
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

    # Create temp directory
    mkdir -p "$PREFIX/tmp"

    # Clone the repository
    git clone "$REPO_URL" "$TEMP_DIR"
    cd "$TEMP_DIR"

    print_success "Repository cloned"

    print_step "Building and installing TaskGuard (this may take 5-10 minutes on mobile)..."
    print_warning "This is a one-time build. Please be patient..."

    # Use cargo install for consistent installation to ~/.cargo/bin
    # Set lower optimization for faster builds on mobile if needed
    CARGO_BUILD_JOBS=2 cargo install --path . --locked

    print_success "TaskGuard installed to $INSTALL_DIR/$BINARY_NAME"
}

setup_path() {
    print_step "Setting up PATH..."

    # Check if install directory is in PATH
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        print_warning "$INSTALL_DIR is not in your PATH"

        # Termux uses bash by default
        SHELL_CONFIG="$HOME/.bashrc"

        # Check if cargo env is already sourced
        if ! grep -q "\.cargo/env" "$SHELL_CONFIG" 2>/dev/null; then
            echo "" >> "$SHELL_CONFIG"
            echo "# Cargo environment (includes TaskGuard)" >> "$SHELL_CONFIG"
            echo ". \"\$HOME/.cargo/env\"" >> "$SHELL_CONFIG"
            print_success "Added Cargo environment to PATH in $SHELL_CONFIG"
        else
            print_success "Cargo environment already configured in $SHELL_CONFIG"
        fi
        print_warning "Please restart Termux or run: source $SHELL_CONFIG"
    else
        print_success "PATH is already configured (Cargo bin in PATH)"
    fi
}

cleanup() {
    print_step "Cleaning up temporary files..."
    rm -rf "$TEMP_DIR"
    print_success "Cleanup completed"
}

verify_installation() {
    print_step "Verifying installation..."

    # Source cargo env for verification
    if [ -f "$HOME/.cargo/env" ]; then
        source "$HOME/.cargo/env"
    fi

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
            print_warning "Binary is at $INSTALL_DIR/$BINARY_NAME"
            print_warning "Restart Termux to use 'taskguard' command globally"
        else
            print_error "Installation verification failed"
            exit 1
        fi
    fi
}

print_usage_info() {
    echo ""
    echo -e "${GREEN}🎉 TaskGuard Installation Complete on Termux!${NC}"
    echo ""
    echo "Quick Start:"
    echo "  1. Restart Termux or run: source ~/.bashrc"
    echo "  2. Navigate to any project directory: cd ~/projects/myapp"
    echo "  3. Initialize TaskGuard: taskguard init"
    echo "  4. Create your first task: taskguard create --title \"Setup project\" --area setup"
    echo "  5. List tasks: taskguard list"
    echo "  6. Check dependencies: taskguard validate"
    echo ""
    echo "Termux-Specific Tips:"
    echo "  • Use Termux:Widget for quick task access"
    echo "  • Storage location: $HOME (shared with Termux)"
    echo "  • Binary location: $INSTALL_DIR/$BINARY_NAME"
    echo ""
    echo "For more information, visit: https://github.com/Guard8-ai/TaskGuard"
    echo ""
}

print_termux_widget_info() {
    echo -e "${BLUE}📱 Optional: Termux Widget Integration${NC}"
    echo ""
    echo "Create a widget script for quick task status:"
    echo ""
    cat << 'EOF'
mkdir -p ~/.shortcuts
cat > ~/.shortcuts/taskguard-status.sh << 'WIDGET'
#!/data/data/com.termux/files/usr/bin/bash
cd ~/your-project-dir
taskguard list
WIDGET
chmod +x ~/.shortcuts/taskguard-status.sh
EOF
    echo ""
    echo "Then add Termux:Widget to your home screen!"
    echo ""
}

# Main installation process
main() {
    echo -e "${BLUE}TaskGuard Termux Installation Script${NC}"
    echo "This will install TaskGuard globally on Termux/Android"
    echo ""

    detect_termux
    check_storage
    check_dependencies
    create_install_dir
    clone_and_install
    setup_path
    cleanup
    verify_installation
    print_usage_info
    print_termux_widget_info
}

# Run main function
main "$@"
