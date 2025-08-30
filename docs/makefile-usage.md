# Makefile Usage Guide

This guide explains how to use the Monorepo Template's Makefile-based build system.

## Overview

The project uses a hierarchical Makefile system:

- **Top-level Makefile**: Orchestrates building all components
- **Component Makefiles**: Handle building individual components
- **Component Template**: Generates new component Makefiles

## Top-Level Commands

### Building Components

```bash
# Build all components
make build all

# Build all components
make build

# Build specific language components
make build-rust-components
make build-python-components
make build-typescript-components

# Build all (alternative syntax)
make all
```

### Component Management

```bash
# Add a new component
make add-component NAME=my-api

# This will:
# - Create the component directory
# - Generate a component-specific Makefile
# - Create a README.md
# - Set up basic structure
```

### Development and Testing

```bash
# Run development mode for all components
make dev

# Run tests for all components
make test

# Clean all build artifacts
make clean
```

### Getting Help

```bash
# Show all available targets
make help

# Show component-specific help
make -C <component-name> help
```

## Component-Specific Commands

Each component has its own Makefile with specialized targets. Navigate to a component directory to see its specific commands:

```bash
cd <component-name>
make help
```

### Component Management

```bash
# Build release version
make build

# Build development version
make build-dev

# Run in development mode
make dev

# Run tests
make test

# Clean build artifacts
make clean

# Code quality
make fmt          # Format code
make clippy       # Run linter
make check        # Check without building
```

## Advanced Usage

### Building Specific Components

The `make build` command accepts comma-separated component names:

```bash
# Build specific language components
make build-rust-components
make build-python-components
make build-typescript-components
```

### Component Dependencies

Components are built independently, but you can modify the top-level Makefile to add dependency relationships if needed.

### Custom Build Configurations

You can pass environment variables to customize builds:

```bash
# Build with debug information
DEBUG=1 make build

# Build with specific features
FEATURES=test,dev make build
```

## Troubleshooting

### Common Issues

1. **Component not found**: Run `make setup` to create the initial project structure
2. **Build failures**: Check that dependencies are installed (e.g., Rust for Rust components)
3. **Permission errors**: Ensure you have write permissions in the project directory

### Debugging

```bash
# Verbose output
make -d build

# Show what would be executed
make -n build
```

## Best Practices

1. **Always use `make help`** to see available commands
2. **Build specific components** rather than all when developing
3. **Use `make clean`** when switching between build configurations
4. **Check component-specific help** for specialized commands
5. **Use `make add-component`** for consistent component structure

## Extending the System

### Adding New Targets

To add new top-level targets, edit the main `Makefile`:

```makefile
new-target: ## Description of new target
	@echo "Executing new target..."
	# Your commands here
```

### Customizing Component Templates

Edit `component-template.mk` to change how new components are generated:

```makefile
create-component-makefile:
	# Add new targets or modify existing ones
```

### Adding Build Dependencies

Modify the top-level Makefile to add component dependencies:

```makefile
# Dependencies between components can be defined in component Makefiles
	@echo "Building component..."
	$(MAKE) -C <component-name> build
```
