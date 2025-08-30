# Getting Started

This guide will help you get the Monorepo Template up and running on your local machine.

## Prerequisites

Before you begin, ensure you have the following installed:

- **Git** - For version control
- **Make** - For the build system
- **Rust** - For high-performance components (see [Rust Installation](#rust-installation))
- **Python/Java/TypeScript** - For other components (technology decisions pending)

### Rust Installation

Rust components require the Rust toolchain. Install it using:

```bash
# macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
# Download from https://rustup.rs/

# Verify installation
rustc --version
cargo --version
```

## Quick Start

### 1. Clone the Repository

```bash
git clone <your-repo-url>
cd monorepo-template
```

### 2. Set Up Project Structure

```bash
# Create initial component structure
make setup
```

This will create:

- All component directories
- Component-specific Makefiles
- Basic README files
- Rust project for high-performance components

### 3. Verify Installation

```bash
# Check available commands
make help

# Check component-specific commands
make help
```

### 4. Build Your First Component

```bash
# Build all components
make build

# Or build specific language components
make build-rust-components
make build-python-components
make build-typescript-components
```

## Project Structure

After running `make setup`, your project will look like this:

```
monorepo-template/
├── docs/                    # Documentation
├── components/              # Your project components
│   ├── src/                # Source code
│   ├── Cargo.toml          # Rust dependencies
│   ├── Makefile            # Component build system
│   ├── docs/               # Component documentation
│   └── README.md           # Component overview



├── shared/                 # Shared utilities (placeholder)
├── Makefile                # Top-level build system
├── component-template.mk   # Component generator
└── README.md               # Project overview
```

## Development Workflow

### 1. Daily Development

```bash
# Start development mode for all components
make dev

# In another terminal, build changes
make build
```

### 2. Testing

```bash
# Run tests for all components
make test

# Run tests for specific component
make test
```

### 3. Code Quality

```bash
# Format all code
make fmt

# Run linter for all components
make lint
```

### 4. Adding New Components

```bash
# Add a new API component
make add-component NAME=api-gateway

# The component will be created with:
# - Directory structure
# - Makefile with standard targets
# - README.md
# - Basic setup
```

## Common Commands Reference

| Command                          | Description                  |
| -------------------------------- | ---------------------------- |
| `make help`                      | Show all available commands  |
| `make setup`                     | Initialize project structure |
| `make build all`                 | Build all components         |
| `make build <component>`         | Build specific component     |
| `make dev`                       | Run development mode         |
| `make test`                      | Run all tests                |
| `make clean`                     | Clean build artifacts        |
| `make add-component NAME=<name>` | Add new component            |

## Next Steps

1. **Explore the components**: Use `make list-components` to see available components
2. **Set up your IDE**: Configure your editor for your chosen languages
4. **Join the development**: Check the [Development Guidelines](./development-guidelines.md)

## Troubleshooting

### Common Issues

**"Component not found" error**

```bash
# Run setup to create component structure
make setup
```

**Rust build failures**

```bash
# Check Rust installation
rustc --version

# Update Rust
rustup update
```

**Permission errors**

```bash
# Check directory permissions
ls -la

# Fix permissions if needed
chmod 755 .
```

### Getting Help

- **Makefile help**: `make help`
- **Component help**: `make -C <component> help`
- **Documentation**: Check the `docs/` directory
- **Issues**: Create an issue in your repository

## Contributing

See [Development Guidelines](./development-guidelines.md) for information on:

- Code standards
- Testing requirements
- Pull request process
- Component development guidelines
