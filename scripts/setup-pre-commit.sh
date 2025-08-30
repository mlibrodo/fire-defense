#!/bin/bash

# Setup script for pre-commit hooks in Monorepo Template
set -e

echo "🔧 Setting up pre-commit hooks for Monorepo Template..."

# Check if we're in the right directory
if [ ! -f "Makefile" ]; then
    echo "❌ Error: This script must be run from the root of the monorepo"
echo "   Expected: monorepo root directory"
    echo "   Current: $(pwd)"
    exit 1
fi

# Check if pre-commit is installed
if ! command -v pre-commit &> /dev/null; then
    echo "📦 Installing pre-commit..."
    if command -v pip3 &> /dev/null; then
        pip3 install pre-commit
    elif command -v pip &> /dev/null; then
        pip install pre-commit
    else
        echo "❌ Error: pip not found. Please install Python and pip first."
        exit 1
    fi
else
    echo "✅ pre-commit already installed"
fi

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "❌ Error: Rust not found. Please install Rust first: https://rustup.rs/"
    exit 1
fi

# Install Rust components
echo "🔧 Installing Rust components..."
rustup component add rustfmt clippy

# Use the top-level Makefile to install pre-commit hooks
echo "🔧 Installing pre-commit hooks using Makefile..."
make pre-commit-install

# Run pre-commit on all files
echo "🔍 Running pre-commit on all files..."
make pre-commit-run

echo "✅ Setup complete! Pre-commit hooks are now active."
echo ""
echo "📝 Monorepo Usage:"
echo "  - Pre-commit hooks will run automatically on commit"
echo "  - Run manually: make pre-commit-run"
echo "  - Run specific component: make -C <component-name> fmt"
echo "  - Skip hooks: git commit --no-verify"
echo ""
echo "🔧 Available Makefile targets:"
echo "  - make help                    # Show all available commands"
echo "  - make setup                   # Complete project setup"
echo "  - make build                   # Build all components"
echo "  - make test                    # Run tests for all components"
echo "  - make fmt                     # Format code for all components"
echo "  - make lint                    # Run linters for all components"
echo "  - make pre-commit-run          # Run pre-commit hooks manually"
echo ""
echo "🔧 Component-specific commands:"
echo "  - make -C <component-name> help     # Show component help"
echo "  - make list-components               # List all components"
echo ""
echo "🔧 Pre-commit hooks:"
echo "  - rustfmt: Rust code formatting"
echo "  - clippy: Rust linting"
echo "  - end-of-file-fixer: Fix line endings"
echo "  - trailing-whitespace: Remove trailing whitespace"
echo "  - cargo check: Compilation check"
echo "  - file validation: YAML, JSON, TOML checks"
