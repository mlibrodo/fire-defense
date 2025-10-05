# 🔧 Code Quality & Linting

This monorepo uses pre-commit hooks to ensure code quality, consistent formatting, and prevent common issues across all components.

## 🚀 Quick Setup

### **Option 1: Using the Setup Script**

Run the setup script from the monorepo root:

```bash
./scripts/setup-pre-commit.sh
```

### **Option 2: Using the Makefile**

Use the top-level Makefile commands:

```bash
# Complete setup (installs Rust components + pre-commit hooks)
make setup

# Just install pre-commit hooks
make pre-commit-install

# Run pre-commit hooks manually
make pre-commit-run
```

## 🏗️ **Monorepo Structure**

This project is organized as a monorepo with the following structure:

```
monorepo-template/
├── Makefile                    # Top-level build system with multi-language support
├── .pre-commit-config.yaml    # Multi-language pre-commit hooks configuration
├── components/                 # Your project components (Rust, Python, TypeScript, Java, Go)
│   ├── api-client/            # API client integration (Rust)
│   └── data-processor/        # Data processing (Rust)


```

### **Multi-Language Support**

- **Rust**: All Rust components (default)
- **Python**: logic-layer, business logic components
- **TypeScript**: web-ui, mobile-ui, frontend components
- **Future**: Java, Go, C++ (easily extensible)

## 📋 What Gets Checked

### 🔍 **Pre-commit Hooks**

#### **Universal Checks (All Languages)**

- **end-of-file-fixer** - Ensures files end with newline
- **trailing-whitespace** - Removes trailing whitespace
- **file validation** - YAML, JSON, TOML format checks
- **merge conflict detection** - Prevents incomplete merges
- **large file detection** - Prevents oversized files

#### **Rust Components**

- **rustfmt** - Code formatting for all `.rs` files
- **clippy** - Advanced linting with strict rules
- **cargo check** - Compilation verification

#### **Python Components**

- **black** - Code formatting (PEP 8 compliant)
- **flake8** - Style and error checking
- **isort** - Import sorting and organization
- **mypy** - Static type checking (optional)

#### **TypeScript/JavaScript Components**

- **prettier** - Code formatting for all web files
- **eslint** - Linting and error detection
- **tsc** - TypeScript compilation check

### 📝 **File Format Checks**

- YAML validation
- JSON validation
- TOML validation
- Large file detection
- Merge conflict detection
- Case conflict detection

## 🛠️ Manual Usage

### **Run on All Files**

```bash
pre-commit run --all-files
```

### **Run on Specific Files**

```bash
pre-commit run --files src/main.rs src/lib.rs
```

### **Component-Specific Commands**

```bash
# Format code for specific component
make -C <component-name> fmt

# Run linters for specific component
make -C <component-name> lint

# Run tests for specific component
make -C <component-name> test
```

### **Language-Specific Operations**

```bash
# Python components
make fmt-python-components      # Format all Python code
make lint-python-components     # Lint all Python code
make check-python-components    # Type check Python

# TypeScript components
make fmt-typescript-components  # Format all TypeScript code
make lint-typescript-components # Lint all TypeScript code
make check-typescript-components # Compile TypeScript

# Rust components
make -C <component-name> fmt
make -C <component-name> lint
```

### **Multi-Language Component Management**

```bash
# Create new components with specific languages
make add-component NAME=api-gateway LANG=rust
make add-component NAME=business-logic LANG=python
make add-component NAME=frontend LANG=typescript

# Create new data sources with languages


# Unified operations across all languages
make fmt    # Format all components (all languages)
make lint   # Lint all components (all languages)
make check  # Check all components (all languages)
make build  # Build all components (all languages)
```

### **Run Specific Hook**

```bash
pre-commit run rustfmt --all-files
pre-commit run clippy --all-files
```

### **Skip Hooks (Emergency)**

```bash
git commit --no-verify
```

## ⚙️ Configuration Files

### **`.pre-commit-config.yaml`**

- Defines all pre-commit hooks for multiple languages
- Configures Rust, Python, and TypeScript tools
- Sets up universal file format checks
- Language-specific hook configuration

### **`rustfmt.toml`**

- Rust code formatting rules
- Line width: 100 characters
- Tab spaces: 4
- Unix line endings
- Import organization

### **`rust-toolchain.toml`**

- Ensures consistent Rust versions
- Includes rustfmt and clippy components
- Cross-platform target support

### **Language-Specific Configurations**

#### **Python Components**

- `pyproject.toml` - Project configuration and dependencies
- `black` - Code formatting (PEP 8 compliant)
- `flake8` - Style and error checking
- `isort` - Import sorting and organization
- `mypy` - Static type checking

#### **TypeScript Components**

- `package.json` - Project configuration and dependencies
- `tsconfig.json` - TypeScript compiler settings
- `prettier` - Code formatting
- `eslint` - Linting and error detection

## 🔍 **Common Issues & Fixes**

### **Formatting Issues**

```bash
# Auto-format all Rust files
cargo fmt

# Format specific file
cargo fmt -- src/main.rs
```

### **Linting Issues**

```bash
# Run clippy
cargo clippy

# Run clippy with specific target
cargo clippy --target x86_64-apple-darwin
```

### **Pre-commit Failures**

1. **Check the error message** - Usually shows what needs to be fixed
2. **Run the failing hook manually** - `pre-commit run <hook-name>`
3. **Fix the issues** - Format code, fix lints, etc.
4. **Re-run pre-commit** - `pre-commit run --all-files`

## 📚 **Best Practices**

### **Before Committing**

1. **Rust Components**: Run `cargo fmt` and `cargo clippy`
2. **Python Components**: Run `black` and `flake8`
3. **TypeScript Components**: Run `prettier` and `eslint`
4. **All Components**: Run `make fmt` and `make lint`
5. Let pre-commit hooks run automatically

### **Code Style by Language**

#### **Rust**

- Follow Rust conventions and idioms
- Use meaningful variable names
- Add documentation for public APIs
- Keep functions focused and small
- Use appropriate error types

#### **Python**

- Follow PEP 8 style guidelines
- Use type hints where possible
- Organize imports with `isort`
- Write docstrings for functions and classes
- Use virtual environments for dependencies

#### **TypeScript**

- Follow ESLint rules and Prettier formatting
- Use strict TypeScript settings
- Write comprehensive type definitions
- Use modern ES6+ features
- Maintain consistent import/export patterns

### **Performance**

- Avoid unnecessary allocations
- Use references when possible
- Consider async/await for I/O operations
- Profile performance-critical code

## 🆘 **Troubleshooting**

### **Pre-commit Not Working**

```bash
# Reinstall hooks
pre-commit uninstall
pre-commit install

# Update pre-commit
pip install --upgrade pre-commit
```

### **Rust Tools Missing**

```bash
# Install rustfmt and clippy
rustup component add rustfmt clippy

# Update Rust
rustup update
```

### **Configuration Issues**

- Check file permissions
- Verify YAML syntax
- Ensure Rust version compatibility
- Check for conflicting configurations

## 🔗 **Useful Links**

- [Pre-commit Documentation](https://pre-commit.com/)
- [Rustfmt Documentation](https://rust-lang.github.io/rustfmt/)
- [Clippy Documentation](https://rust-lang.github.io/rust-clippy/)
- [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html)
