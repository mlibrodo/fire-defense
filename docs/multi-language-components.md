# 🌍 Multi-Language Component System

This document describes the multi-language component system that allows you to mix Rust, Python, TypeScript, and other languages in a single monorepo while maintaining consistent code quality and build processes.

## 🎯 **Overview**

The Monorepo Template supports multiple programming languages, allowing you to choose the best tool for each component's specific needs:

- **Rust**: High-performance data processing, systems programming
- **Python**: Business logic, data analysis, machine learning
- **TypeScript**: Web interfaces, APIs, frontend applications
- **Future**: Java, Go, C++, and other languages

## 🚀 **Quick Start**

### **Creating Components with Specific Languages**

### **Artifact Types**

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
make add-component NAME=tool LANG=go ARTIFACT_TYPE=cli
make add-component NAME=app LANG=java ARTIFACT_TYPE=service
```

```bash
# Create a Rust component
make add-component NAME=high-performance-processor LANG=rust

# Create a Python component
make add-component NAME=business-logic-engine LANG=python

# Create a TypeScript component
make add-component NAME=web-dashboard LANG=typescript

# Create components with specific artifact types
make add-component NAME=api-client LANG=rust ARTIFACT_TYPE=library
make add-component NAME=web-server LANG=python ARTIFACT_TYPE=service
make add-component NAME=dashboard LANG=typescript ARTIFACT_TYPE=webapp


```

### **Language-Specific Operations**

```bash
# Format code for all components of a specific language
make fmt-python-components
make fmt-typescript-components

# Lint code for all components of a specific language
make lint-python-components
make lint-typescript-components

# Check code for all components of a specific language
make check-python-components
make check-typescript-components
```

## 🏗️ **Component Structure**

### **Rust Components**

Rust components are designed for high-performance, systems-level operations:

```
component-name/
├── Cargo.toml          # Rust package configuration
├── src/
│   ├── main.rs         # Binary entry point
│   └── lib.rs          # Library interface
├── tests/              # Integration tests
├── docs/               # Component-specific documentation
│   └── README.md       # Detailed component documentation
├── Makefile            # Build targets
└── README.md           # Component overview
```

**Pre-commit Hooks:**
- `rustfmt` - Code formatting
- `clippy` - Advanced linting
- `cargo check` - Compilation verification

**Build Commands:**
```bash
make -C component-name build    # Build component
make -C component-name test     # Run tests
make -C component-name fmt      # Format code
make -C component-name clippy   # Run linter
```

### **Python Components**

Python components are ideal for business logic, data processing, and ML workflows:

```
component-name/
├── pyproject.toml      # Modern Python packaging
├── src/
│   └── component_name/
│       ├── __init__.py # Package initialization
│       └── main.py     # Main functionality
├── tests/              # Test suite
├── main.py             # CLI entry point
├── docs/               # Component-specific documentation
│   └── README.md       # Detailed component documentation
├── Makefile            # Build targets
└── README.md           # Component overview
```

**Pre-commit Hooks:**
- `black` - PEP 8 compliant formatting
- `flake8` - Style and error checking
- `isort` - Import sorting and organization
- `mypy` - Static type checking

**Build Commands:**
```bash
make -C component-name build    # Install in development mode
make -C component-name test     # Run pytest
make -C component-name fmt      # Format with black/isort
make -C component-name lint     # Run flake8
```

### **TypeScript Components**

TypeScript components are perfect for web interfaces, APIs, and frontend applications:

### **Java Components**

Java components are ideal for enterprise applications, microservices, and backend systems:

```
component-name/
├── pom.xml                 # Maven project configuration
├── src/
│   ├── main/java/          # Main source code
│   │   └── com/example/component_name/
│   │       └── Main.java   # Main class
│   └── test/java/          # Test source code
├── docs/                   # Component-specific documentation
│   └── README.md           # Detailed component documentation
├── Makefile                # Build targets
└── README.md               # Component overview
```

**Pre-commit Hooks:**
- `google-java-format` - Code formatting (Google style)
- `checkstyle` - Code quality and style checking
- `mvn validate` - Maven validation

**Build Commands:**
```bash
make -C component-name build    # Build component
make -C component-name test     # Run tests
make -C component-name fmt      # Format code
make -C component-name lint     # Run linter
```

### **Go Components**

Go components are perfect for high-performance services, CLI tools, and microservices:

```
component-name/
├── go.mod              # Go module configuration
├── cmd/                # Command-line entry points
│   └── main.go         # Main function
├── internal/            # Internal packages
├── pkg/                 # Public packages
├── docs/                # Component-specific documentation
│   └── README.md        # Detailed component documentation
├── Makefile             # Build targets
└── README.md            # Component overview
```

**Pre-commit Hooks:**
- `go fmt` - Code formatting
- `goimports` - Import organization
- `go vet` - Code analysis
- `golangci-lint` - Comprehensive linting

**Build Commands:**
```bash
make -C component-name build    # Build component
make -C component-name test     # Run tests
make -C component-name fmt      # Format code
make -C component-name lint     # Run linter
make -C component-name run      # Run component
```

**Pre-commit Hooks:**
- `prettier` - Code formatting
- `eslint` - Linting and error detection
- `tsc` - TypeScript compilation check

**Build Commands:**
```bash
make -C component-name build    # Compile TypeScript
make -C component-name test     # Run Jest tests
make -C component-name fmt      # Format with prettier
make -C component-name lint     # Run eslint
```

## 📚 **Component Documentation**

### **Documentation Structure**

Each component automatically includes a comprehensive documentation structure:

```
component-name/
├── docs/               # Component-specific documentation
│   └── README.md       # Detailed component documentation
└── README.md           # Component overview (top-level)
```

### **Automatic Documentation Generation**

When you create a new component, documentation is automatically generated:

```bash
# Create component with automatic documentation
make add-component NAME=my-api LANG=rust
make add-component NAME=business-logic LANG=python
make add-component NAME=frontend LANG=typescript

# Create documentation for existing components
make create-component-docs
```

### **Documentation Content**

Each component's `docs/README.md` includes:

- **Overview**: Component purpose and functionality
- **Features**: Key capabilities and integrations
- **Usage**: Build, test, and development commands
- **Language-specific**: Information relevant to the component's language

### **Documentation Management**

- **Automatic**: Created when components are added
- **Consistent**: Standardized format across all components
- **Maintainable**: Easy to update and extend
- **Integrated**: Part of the component creation workflow

## 🔧 **Unified Build System**

### **Cross-Language Operations**

```bash
# Build all components (all languages)
make build

# Format all components (all languages)
make fmt

# Lint all components (all languages)
make lint

# Check all components (all languages)
make check

# Test all components (all languages)
make test
```

### **Language-Specific Build Targets**

```bash
# Rust components
make build-rust-components      # Build Rust components
make build-rust-components      # Build Rust components

# Python components
make build-python-components     # Build all Python components

# TypeScript components
make build-typescript-components # Build all TypeScript components
```

## 📋 **Pre-commit Integration**

### **Universal Hooks**

These hooks run on all files regardless of language:

- **end-of-file-fixer** - Ensures proper file endings
- **trailing-whitespace** - Removes trailing spaces
- **check-yaml** - Validates YAML files
- **check-json** - Validates JSON files
- **check-toml** - Validates TOML files
- **check-merge-conflict** - Prevents incomplete merges
- **check-added-large-files** - Prevents oversized files

### **Language-Specific Hooks**

Hooks are automatically applied based on file extensions:

- **`.rs` files** → Rust hooks (rustfmt, clippy)
- **`.py` files** → Python hooks (black, flake8, isort)
- **`.ts/.js/.tsx/.jsx` files** → TypeScript hooks (prettier, eslint)

## 🎨 **Code Quality Standards**

### **Rust Quality Standards**

- **Formatting**: `rustfmt` with 100-character line width
- **Linting**: `clippy` with strict warnings enabled
- **Documentation**: Comprehensive doc comments
- **Testing**: Unit and integration tests
- **Error Handling**: Proper `Result<T, E>` usage

### **Python Quality Standards**

- **Formatting**: `black` with default settings
- **Style**: `flake8` with PEP 8 compliance
- **Imports**: `isort` for organized imports
- **Types**: `mypy` for static type checking
- **Documentation**: Docstrings for all functions

### **TypeScript Quality Standards**

- **Formatting**: `prettier` with consistent rules
- **Linting**: `eslint` with TypeScript rules
- **Types**: Strict TypeScript configuration
- **Testing**: Jest test framework
- **Documentation**: JSDoc comments

## 🚀 **Advanced Usage**

### **Custom Language Support**

To add support for a new language:

1. **Update the Makefile**:
   ```makefile
   configure-new-language-component: ## Configure New Language component
       @echo "Setting up New Language component: $(NAME)"
       # Add language-specific setup commands
   ```

2. **Update pre-commit configuration**:
   ```yaml
   # NEW LANGUAGE COMPONENTS
   - repo: https://github.com/example/tool
     rev: v1.0.0
     hooks:
       - id: tool-name
         files: ^.*\.ext$
   ```

3. **Add language-specific targets**:
   ```makefile
   fmt-new-language-components: ## Format New Language components
       @echo "Formatting New Language components..."
       # Add formatting commands
   ```

### **Component Dependencies**

Components can depend on each other across languages:

```bash
# Python component depending on Rust library
make add-component NAME=ml-processor LANG=python
# Add Rust dependency to pyproject.toml

# TypeScript component depending on Python API
make add-component NAME=api-client LANG=typescript
# Add Python API calls to TypeScript code
```

### **Testing Strategies**

```bash
# Test specific language components
make test                       # All tests
make test-python-components      # Python tests
make test-typescript-components  # TypeScript tests

# Integration tests across languages
make test-integration            # Cross-language tests
```

## 🔍 **Troubleshooting**

### **Common Issues**

1. **Language Tools Not Found**:
   ```bash
   # Install Python tools
   pip install black flake8 isort mypy
   
   # Install Node.js tools
   npm install -g prettier eslint typescript
   ```

2. **Pre-commit Hook Failures**:
   ```bash
   # Run specific hook manually
   pre-commit run black --all-files
   
   # Check hook configuration
   cat .pre-commit-config.yaml
   ```

3. **Build Failures**:
   ```bash
   # Check component status
   make status
   
   # Clean and rebuild
   make clean && make build
   ```

### **Performance Optimization**

- **Parallel Builds**: Components build independently
- **Incremental Compilation**: Only changed components rebuild
- **Language-Specific Caching**: Each language uses appropriate caching
- **Dependency Management**: Smart dependency resolution

## 📚 **Best Practices**

### **Component Design**

1. **Single Responsibility**: Each component has one clear purpose
2. **Language Choice**: Choose language based on component needs
3. **Interface Design**: Clear APIs between components
4. **Testing Strategy**: Comprehensive test coverage
5. **Documentation**: Clear README and inline docs

### **Development Workflow**

1. **Create Component**: Use `make add-component` with appropriate language
2. **Implement Functionality**: Follow language-specific best practices
3. **Add Tests**: Ensure comprehensive test coverage
4. **Format Code**: Run language-specific formatters
5. **Commit Changes**: Pre-commit hooks ensure quality

### **Maintenance**

1. **Regular Updates**: Keep language tools updated
2. **Dependency Management**: Monitor and update dependencies
3. **Performance Monitoring**: Track build and test times
4. **Code Quality**: Regular linting and formatting
5. **Documentation**: Keep docs current with code changes

## 🔮 **Future Enhancements**

### **Planned Features**

- **Language Detection**: Automatic language detection for existing code
- **Template System**: Customizable component templates
- **CI/CD Integration**: Language-specific CI/CD pipelines
- **Performance Metrics**: Build time and quality metrics
- **Plugin System**: Extensible language support

### **Language Roadmap**

- **Java**: Enterprise integration components
- **Go**: High-performance networking components
- **C++**: Low-level system components
- **Kotlin**: Android mobile components
- **Swift**: iOS mobile components

## 📖 **Additional Resources**

- **[Linting Guide](./LINTING.md)** - Detailed pre-commit and quality tools
- **[Makefile Usage](./makefile-usage.md)** - Complete build system guide

- **[Getting Started](./getting-started.md)** - Quick start guide

---

*This multi-language component system provides the flexibility to use the right tool for each job while maintaining consistent quality standards across your entire monorepo.*
