# Monorepo Template Documentation

Welcome to the Monorepo Template documentation. This directory contains detailed information about the template system and how to use the build system for any multi-language project.

## Table of Contents

- [Getting Started](./getting-started.md) - Quick start guide and setup
- [Makefile Usage](./makefile-usage.md) - Comprehensive guide to using the Makefile system

- [Multi-Language Components](./multi-language-components.md) - Multi-language component system guide
- [Linting & Code Quality](./LINTING.md) - Pre-commit hooks and quality tools
- [Development Guidelines](./development-guidelines.md) - Coding standards and best practices
- [API Documentation](./api/README.md) - API specifications and examples

## Quick Navigation

- **Top-level commands**: See [Makefile Usage](./makefile-usage.md)
- **Component development**: See individual component directories

- **Getting help**: Run `make help` from the project root

## Project Structure

```
monorepo-template/
├── docs/                    # This documentation directory
│   ├── multi-language-components.md  # Multi-language system guide
│   ├── LINTING.md          # Code quality and pre-commit hooks
│   ├── api/                # API documentation
│   └── ...
├── components/              # Your project components (any language)
│   ├── api-service/         # Example Rust component
│   │   ├── .monorepo-component  # Component metadata
│   │   ├── docs/            # Component-specific documentation
│   │   │   └── README.md    # Detailed component docs
│   │   ├── src/             # Source code
│   │   ├── Cargo.toml       # Rust configuration
│   │   ├── Makefile         # Component build system
│   │   └── README.md        # Component overview
│   ├── business-logic/      # Example Python component
│   │   ├── .monorepo-component  # Component metadata
│   │   ├── docs/            # Component-specific documentation
│   │   ├── src/             # Source code
│   │   ├── pyproject.toml   # Python configuration
│   │   └── README.md        # Component overview
│   └── web-interface/       # Example TypeScript component
│       ├── .monorepo-component  # Component metadata
│       ├── docs/            # Component-specific documentation
│       ├── src/             # Source code
│       ├── package.json     # Node.js configuration
│       └── README.md        # Component overview
├── Makefile                 # Top-level build system with multi-language support
├── .pre-commit-config.yaml  # Multi-language pre-commit hooks
└── .gitignore               # Git ignore patterns
```

### **Component Documentation Structure**

Each component now includes:

- **`docs/README.md`**: Detailed component documentation with features, usage, and language-specific information
- **`README.md`**: Component overview and quick start information
- **`.monorepo-component`**: Metadata file with component name, language, creation date, and artifact type
- **Automatic Generation**: Documentation is created automatically when components are added
- **Consistent Format**: Standardized documentation structure across all components

## Getting Help

- **Makefile help**: `make help` - Shows all available targets
- **Component help**: `make -C <component> help` - Shows component-specific targets
- **Component discovery**: `make list-components` - Lists all components with metadata
- **Component status**: `make status` - Shows component status and health
- **Build all components**: `make build` - Builds all components (all languages)
- **Language-specific builds**: `make build-python-components`, `make build-typescript-components`
- **Add new component**: `make add-component NAME=<name> LANG=<rust|python|typescript>`

- **Code quality**: `make fmt`, `make lint`, `make check` - Format, lint, and check all components
- **Documentation**: `make create-component-docs` - Create docs for existing components
