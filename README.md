# Monorepo Template

A generic monorepo template that can be used for any multi-language project. Supports Rust, Python, and TypeScript components with automated setup and quality tools.

## Project Structure

This monorepo template is designed to be flexible and can contain any combination of components:

- **Rust Components**: High-performance systems and services
- **Python Components**: Business logic, data processing, and ML workflows
- **TypeScript Components**: Web interfaces, APIs, and frontend applications
- **Shared Components**: Common utilities and libraries
- **Custom Components**: Any other language or framework you need

### **Multi-Language Support**

- **Rust**: High-performance data processing and systems
- **Python**: Business logic, data analysis, and ML components
- **TypeScript**: Web interfaces, APIs, and frontend components
- **Future**: Java, Go, C++ for specialized use cases

## Quick Start

```bash
# Set up the project structure
make setup

# Build all components (all languages)
make build

# Build all components
make build

# Build language-specific components
make build-python-components
make build-typescript-components

# Add a new component with specific language
make add-component NAME=my-api LANG=rust
make add-component NAME=business-logic LANG=python
make add-component NAME=frontend LANG=typescript

# Add a component with specific artifact type
make add-component NAME=api-service LANG=rust ARTIFACT_TYPE=service
make add-component NAME=web-app LANG=typescript ARTIFACT_TYPE=webapp
make add-component NAME=utility-lib LANG=python ARTIFACT_TYPE=library



# Show all available commands
make help
```

## Documentation

📚 **Comprehensive documentation is available in the [`docs/`](./docs/) directory:**

- **[Getting Started](./docs/getting-started.md)** - Quick start guide and setup
- **[Makefile Usage](./docs/makefile-usage.md)** - Complete guide to the build system

- **[Multi-Language Components](./docs/multi-language-components.md)** - Component system guide

## Key Features

- **Multi-Language Architecture**: Mix Rust, Python, TypeScript in one monorepo
- **Modular Components**: Independent components with clear interfaces
- **Intelligent Build System**: Language-aware Makefile with unified commands
- **Component Generation**: Add new components with `make add-component NAME=name LANG=language`
- **Pre-commit Integration**: Language-specific quality tools for all components
- **Technology Flexibility**: Choose the right tool for each component's needs
- **Comprehensive Documentation**: Detailed guides for development and usage

## Requirements

- **Rust** (for Rust components)
- **Python 3.8+** (for Python components)
- **Node.js 18+** (for TypeScript components)
- **Make** (for build system)
- **Docker** (optional, for containerization)

## Development

### First Time Setup

```bash
# Clone the repository
git clone <your-repo-url>
cd monorepo-template

# Set up the project structure
make setup

# Verify installation
make help
```

### Daily Development

```bash
# Build all components (all languages)
make build

# Build language-specific components
make build-python-components
make build-typescript-components

# Run in development mode
make dev

# Run tests for all components
make test

# Format code (all languages)
make fmt

# Lint code (all languages)
make lint

# Clean builds
make clean
```

### Adding New Components

```bash
# Add a new component with specific language
make add-component NAME=my-api LANG=rust
make add-component NAME=business-logic LANG=python
make add-component NAME=frontend LANG=typescript

# Add a component with specific artifact type
make add-component NAME=api-service LANG=rust ARTIFACT_TYPE=service
make add-component NAME=web-app LANG=typescript ARTIFACT_TYPE=webapp
make add-component NAME=utility-lib LANG=python ARTIFACT_TYPE=library

# The component will be created with:
# - Language-appropriate directory structure
# - Language-specific configuration files
# - Makefile with standard targets
# - README.md (component overview)
# - docs/README.md (detailed documentation)
# - .monorepo-component metadata file with artifact type
# - Pre-commit hooks configured for the language
# - Build and test setup ready to use
```

### Artifact Types

Components can be created with specific artifact types to indicate their purpose:

- **`library`**: Reusable code libraries and packages
- **`service`**: Backend services and APIs
- **`webapp`**: Web applications and frontends
- **`cli`**: Command-line interface tools
- **`docker`**: Containerized applications

```bash
# Create components with specific artifact types
make add-component NAME=api-client LANG=rust ARTIFACT_TYPE=library
make add-component NAME=web-server LANG=python ARTIFACT_TYPE=service
make add-component NAME=dashboard LANG=typescript ARTIFACT_TYPE=webapp
```

### Component Documentation

Each component automatically includes comprehensive documentation:

- **`README.md`**: Component overview and quick start
- **`docs/README.md`**: Detailed documentation with features, usage, and examples
- **`.monorepo-component`**: Metadata file with component information
- **Automatic Generation**: Documentation is created when components are added
- **Consistent Structure**: Standardized format across all components

```bash
# Create documentation for existing components
make create-component-docs

# List all components with metadata
make list-components
```

## Getting Help

- **Top-level help**: `make help` - Shows all available commands
- **Component help**: `make -C <component> help` - Shows component-specific commands
- **Documentation**: Check the [`docs/`](./docs/) directory
- **Examples**: See the [Makefile Usage Guide](./docs/makefile-usage.md)

## Contributing

1. **Read the documentation**: Start with [Getting Started](./docs/getting-started.md)
2. **Follow development guidelines**: Check [Development Guidelines](./docs/development-guidelines.md)
3. **Use the build system**: Learn [Makefile Usage](./docs/makefile-usage.md)

## Status

- **No components yet**: This is a fresh monorepo template
- **Ready for components**: Use `make add-component` to create your first component
- **Pre-commit hooks**: 🟢 Installed and ready
- **Build system**: 🟢 Ready for multi-language development

### **Language Support Status**

- **Rust**: 🟢 Fully implemented with rustfmt, clippy, cargo
- **Python**: 🟢 Fully implemented with black, flake8, isort, mypy
- **TypeScript**: 🟢 Fully implemented with prettier, eslint, tsc
- **Java**: 🟢 Fully implemented with Google Java Format, Checkstyle, Maven
- **Go**: 🟢 Fully implemented with gofmt, golint, go vet, golangci-lint
- **Pre-commit Hooks**: 🟢 Multi-language support active

## License

[Add your license information here]
