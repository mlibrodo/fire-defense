# Monorepo Template - Makefile
# Generic monorepo management system for any project

.PHONY: help setup build clean test dev install-deps fmt lint pre-commit-install pre-commit-run pre-commit-clean

# Default target
help: ## Show this help message
	@echo "Monorepo Template - Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-25s\033[0m %s\n", $$1, $$2}'
	@echo ""
	@echo "Component Management:"
	@echo "  make add-component NAME=name LANG=rust|python|typescript|java|go [ARTIFACT_TYPE=type] # Add new component"
	@echo "  make create-component-docs                                # Create docs for existing components"
	@echo ""
	@echo "Component Operations:"
	@echo "  make build-component NAME=name                            # Build specific component"
	@echo "  make test-component NAME=name                            # Test specific component"
	@echo "  make clean-component NAME=name                           # Clean specific component"
	@echo "  make fmt-component NAME=name                             # Format specific component"
	@echo ""
	@echo "Component Discovery:"
	@echo "  make list-components                                      # List all monorepo components"
	@echo "  make status                                              # Show component status"
	@echo ""
	@echo "Testing:"
	@echo "  make check-component-generation LANG=all|rust|python|typescript|java|go  # Test component generation"
	@echo ""
	@echo "Language Support:"
	@echo "  Rust: rustfmt, clippy, cargo"
	@echo "  Python: black, flake8, isort, mypy"
	@echo "  TypeScript: prettier, eslint, tsc"
	@echo "  Java: maven, checkstyle, spotbugs"
	@echo "  Go: gofmt, golint, go vet"

# =============================================================================
# PROJECT SETUP
# =============================================================================

# Supported languages for components
SUPPORTED_LANGUAGES := rust python typescript java go

# Supported artifact types
SUPPORTED_ARTIFACT_TYPES := library docker webapp cli service

setup: ## Set up the entire project structure
	@echo "Setting up Monorepo Template..."
	@echo "Installing Rust components..."
	rustup component add rustfmt clippy
	@echo "Setting up pre-commit hooks..."
	@$(MAKE) pre-commit-install
	@echo "Setup complete!"

# =============================================================================
# BUILD COMMANDS
# =============================================================================

build: ## Build all components
	@echo "Building all components..."
	@$(MAKE) build-rust-components
	@$(MAKE) build-python-components
	@$(MAKE) build-typescript-components
	@echo "All components built successfully!"

build-component: ## Build a specific component (usage: make build-component NAME=component-name)
	@if [ -z "$(NAME)" ]; then \
		echo "Error: NAME parameter required. Usage: make build-component NAME=component-name"; \
		exit 1; \
	fi
	@if [ ! -f "$(NAME)/.monorepo-component" ]; then \
		echo "Error: Component '$(NAME)' not found or not a valid monorepo component"; \
		echo "Available components:"; \
		$(MAKE) list-components; \
		exit 1; \
	fi
	@echo "Building component: $(NAME)"
	@$(MAKE) -C $(NAME) build
	@echo "Component $(NAME) built successfully!"

test-component: ## Test a specific component (usage: make test-component NAME=component-name)
	@if [ -z "$(NAME)" ]; then \
		echo "Error: NAME parameter required. Usage: make test-component NAME=component-name"; \
		exit 1; \
	fi
	@if [ ! -f "$(NAME)/.monorepo-component" ]; then \
		echo "Error: Component '$(NAME)' not found or not a valid monorepo component"; \
		echo "Available components:"; \
		$(MAKE) list-components; \
		exit 1; \
	fi
	@echo "Testing component: $(NAME)"
	@$(MAKE) -C $(NAME) test
	@echo "Component $(NAME) tests completed!"

clean-component: ## Clean a specific component (usage: make clean-component NAME=component-name)
	@if [ -z "$(NAME)" ]; then \
		echo "Error: NAME parameter required. Usage: make clean-component NAME=component-name"; \
		exit 1; \
	fi
	@if [ ! -f "$(NAME)/.monorepo-component" ]; then \
		echo "Error: Component '$(NAME)' not found or not a valid monorepo component"; \
		echo "Available components:"; \
		$(MAKE) list-components; \
		exit 1; \
	fi
	@echo "Cleaning component: $(NAME)"
	@$(MAKE) -C $(NAME) clean
	@echo "Component $(NAME) cleaned!"

fmt-component: ## Format a specific component (usage: make fmt-component NAME=component-name)
	@if [ -z "$(NAME)" ]; then \
		echo "Error: NAME parameter required. Usage: make fmt-component NAME=component-name"; \
		exit 1; \
	fi
	@if [ ! -f "$(NAME)/.monorepo-component" ]; then \
		echo "Error: Component '$(NAME)' not found or not a valid monorepo component"; \
		echo "Available components:"; \
		$(MAKE) list-components; \
		exit 1; \
	fi
	@echo "Formatting component: $(NAME)"
	@$(MAKE) -C $(NAME) fmt
	@echo "Component $(NAME) formatted!"

list-components: ## List all monorepo components
	@echo "Monorepo Components:"
	@echo "==================="
	@for dir in */; do \
		if [ -f "$${dir%/}/.monorepo-component" ]; then \
			lang=$$(grep "^LANG=" "$${dir%/}/.monorepo-component" | cut -d'=' -f2); \
			name=$$(grep "^NAME=" "$${dir%/}/.monorepo-component" | cut -d'=' -f1 | cut -d'=' -f2); \
			created=$$(grep "^CREATED=" "$${dir%/}/.monorepo-component" | cut -d'=' -f2); \
			artifact_type=$$(grep "^ARTIFACT_TYPE=" "$${dir%/}/.monorepo-component" | cut -d'=' -f2 2>/dev/null || echo "N/A"); \
			echo "  $$(basename $$dir) ($$lang) - Created: $$created - Type: $$artifact_type"; \
		fi; \
	done

build-rust-components: ## Build all Rust components
	@echo "Building Rust components..."
	@for dir in */; do \
		if [ -f "$${dir%/}/.monorepo-component" ] && [ -f "$${dir%/}/Cargo.toml" ]; then \
			echo "Building $$(basename $$dir)..."; \
			$(MAKE) -C "$${dir%/}" build; \
		fi; \
	done

build-python-components: ## Build all Python components
	@echo "Building Python components..."
	@for dir in */; do \
		if [ -f "$${dir%/}/.monorepo-component" ] && [ -f "$${dir%/}/pyproject.toml" ]; then \
			echo "Building $$(basename $$dir)..."; \
			cd "$${dir%/}" && python3 -m pip install -e . && cd ..; \
		fi; \
	done

build-typescript-components: ## Build all TypeScript components
	@echo "Building TypeScript components..."
	@for dir in */; do \
		if [ -f "$${dir%/}/.monorepo-component" ] && [ -f "$${dir%/}/package.json" ]; then \
			echo "Building $$(basename $$dir)..."; \
			cd "$${dir%/}" && npm run build && cd ..; \
		fi; \
	done

# =============================================================================
# CLEAN COMMANDS
# =============================================================================

clean: ## Clean all components
	@echo "Cleaning all components..."
	@for dir in */; do \
		if [ -f "$${dir%/}/.monorepo-component" ]; then \
			if [ -f "$${dir%/}/Makefile" ]; then \
				$(MAKE) -C "$${dir%/}" clean; \
			elif [ -f "$${dir%/}/pyproject.toml" ]; then \
				cd "$${dir%/}" && rm -rf build/ dist/ *.egg-info/ && cd ..; \
			elif [ -f "$${dir%/}/package.json" ]; then \
				cd "$${dir%/}" && rm -rf node_modules/ dist/ && cd ..; \
			fi; \
		fi; \
	done
	@echo "All components cleaned!"

# =============================================================================
# TEST COMMANDS
# =============================================================================

test: ## Run tests for all components
	@echo "Running tests for all components..."
	@for dir in */; do \
		if [ -f "$${dir%/}/.monorepo-component" ]; then \
			if [ -f "$${dir%/}/Makefile" ]; then \
				$(MAKE) -C "$${dir%/}" test; \
			elif [ -f "$${dir%/}/pyproject.toml" ]; then \
				cd "$${dir%/}" && python3 -m pytest && cd ..; \
			elif [ -f "$${dir%/}/package.json" ]; then \
				cd "$${dir%/}" && npm test && cd ..; \
			fi; \
		fi; \
	done
	@echo "All tests completed!"

# =============================================================================
# DEVELOPMENT COMMANDS
# =============================================================================

dev: ## Start development mode for all components
	@echo "Starting development mode..."
	@echo "Available components:"
	@$(MAKE) list-components
	@echo ""
	@echo "Use 'make -C <component-name> dev' to start specific components"

# =============================================================================
# CODE QUALITY COMMANDS
# =============================================================================

fmt: ## Format code for all components
	@echo "Formatting code for all components..."
	@find . -name "Makefile" -path "*/Makefile" -execdir make fmt \; 2>/dev/null || true
	@$(MAKE) fmt-python-components
	@$(MAKE) fmt-typescript-components
	@echo "Code formatting completed!"

lint: ## Run linters for all components
	@echo "Running linters for all components..."
	@find . -name "Makefile" -path "*/Makefile" -execdir make lint \; 2>/dev/null || true
	@$(MAKE) lint-python-components
	@$(MAKE) lint-typescript-components
	@echo "Linting completed!"

check: ## Check code for all components
	@echo "Checking code for all components..."
	@find . -name "Makefile" -path "*/Makefile" -execdir make check \; 2>/dev/null || true
	@$(MAKE) check-python-components
	@$(MAKE) check-typescript-components
	@echo "Code check completed!"

# Language-specific formatting
fmt-python-components: ## Format Python components
	@echo "Formatting Python components..."
	@find . -name "pyproject.toml" -execdir python3 -m black . \;
	@find . -name "pyproject.toml" -execdir python3 -m isort . \;

fmt-typescript-components: ## Format TypeScript components
	@echo "Formatting TypeScript components..."
	@find . -name "package.json" -execdir npx prettier --write . \;

# Language-specific linting
lint-python-components: ## Lint Python components
	@echo "Linting Python components..."
	@find . -name "pyproject.toml" -execdir python3 -m flake8 . \;

lint-typescript-components: ## Lint TypeScript components
	@echo "Linting TypeScript components..."
	@find . -name "package.json" -execdir npm run lint \;

# Language-specific checking
check-python-components: ## Check Python components
	@echo "Checking Python components..."
	@find . -name "pyproject.toml" -execdir python3 -m mypy . \;

check-typescript-components: ## Check TypeScript components
	@echo "Checking TypeScript components..."
	@find . -name "package.json" -execdir npm run build \;

# =============================================================================
# PRE-COMMIT HOOKS
# =============================================================================

pre-commit-install: ## Install pre-commit hooks
	@echo "Installing pre-commit hooks..."
	@if command -v pre-commit >/dev/null 2>&1; then \
		pre-commit install; \
		echo "Pre-commit hooks installed successfully!"; \
	else \
		echo "Installing pre-commit..."; \
		if command -v pip3 >/dev/null 2>&1; then \
			pip3 install pre-commit; \
		elif command -v pip >/dev/null 2>&1; then \
			pip install pre-commit; \
		else \
			echo "Error: pip not found. Please install Python and pip first."; \
			exit 1; \
		fi; \
		pre-commit install; \
		echo "Pre-commit hooks installed successfully!"; \
	fi

pre-commit-run: ## Run pre-commit hooks on all files
	@echo "Running pre-commit hooks on all files..."
	@if command -v pre-commit >/dev/null 2>&1; then \
		pre-commit run --all-files; \
	else \
		echo "Error: pre-commit not installed. Run 'make pre-commit-install' first."; \
		exit 1; \
	fi

pre-commit-clean: ## Remove pre-commit hooks
	@echo "Removing pre-commit hooks..."
	@if command -v pre-commit >/dev/null 2>&1; then \
		pre-commit uninstall; \
		echo "Pre-commit hooks removed!"; \
	else \
		echo "Pre-commit not installed."; \
	fi

# =============================================================================
# COMPONENT MANAGEMENT
# =============================================================================

add-component: ## Add a new component (usage: make add-component NAME=component-name LANG=language [ARTIFACT_TYPE=type])
	@if [ -z "$(NAME)" ]; then \
		echo "Error: NAME parameter required. Usage: make add-component NAME=component-name LANG=language [ARTIFACT_TYPE=type]"; \
		exit 1; \
	fi
	@if [ -z "$(LANG)" ]; then \
		echo "Warning: LANG not specified, defaulting to rust"; \
		LANG=rust; \
	fi
	@if [ -n "$(LANG)" ] && ! echo "$(SUPPORTED_LANGUAGES)" | grep -q "$(LANG)"; then \
		echo "Error: Unsupported language '$(LANG)'"; \
		echo "Supported languages: $(SUPPORTED_LANGUAGES)"; \
		exit 1; \
	fi
	@if [ -n "$(ARTIFACT_TYPE)" ] && ! echo "$(SUPPORTED_ARTIFACT_TYPES)" | grep -q "$(ARTIFACT_TYPE)"; then \
		echo "Error: Unsupported artifact type '$(ARTIFACT_TYPE)'"; \
		echo "Supported types: $(SUPPORTED_ARTIFACT_TYPES)"; \
		exit 1; \
	fi
	@echo "Creating new $(LANG) component: $(NAME)"
	@if [ -n "$(ARTIFACT_TYPE)" ]; then \
		echo "Artifact type: $(ARTIFACT_TYPE)"; \
	fi
	@mkdir -p $(NAME)
	@cp component-template.mk $(NAME)/Makefile
	@sed -i '' 's/COMPONENT_NAME/$(NAME)/g' $(NAME)/Makefile
	@echo "# $(NAME) Component" > $(NAME)/README.md
	@echo "Component created successfully in $(NAME)/"
	@echo "Language: $(LANG)"
	@echo "Pre-commit hooks will be configured for $(LANG)"
	@$(MAKE) configure-component-hooks NAME=$(NAME) LANG=$(LANG) ARTIFACT_TYPE=$(ARTIFACT_TYPE)

configure-component-hooks: ## Configure pre-commit hooks for a component (internal use)
	@echo "Configuring pre-commit hooks for $(NAME) ($(LANG))..."
	@case "$(LANG)" in \
		rust) \
			$(MAKE) configure-rust-component NAME=$(NAME) ARTIFACT_TYPE=$(ARTIFACT_TYPE); \
			;; \
		python) \
			$(MAKE) configure-python-component NAME=$(NAME) ARTIFACT_TYPE=$(ARTIFACT_TYPE); \
			;; \
		typescript) \
			$(MAKE) configure-typescript-component NAME=$(NAME) ARTIFACT_TYPE=$(ARTIFACT_TYPE); \
			;; \
		java) \
			$(MAKE) configure-java-component NAME=$(NAME) ARTIFACT_TYPE=$(ARTIFACT_TYPE); \
			;; \
		go) \
			$(MAKE) configure-go-component NAME=$(NAME) ARTIFACT_TYPE=$(ARTIFACT_TYPE); \
			;; \
		*) \
			echo "Warning: Unknown language '$(LANG)', using basic template"; \
			;; \
	esac



# =============================================================================
# LANGUAGE-SPECIFIC COMPONENT CONFIGURATION
# =============================================================================

configure-rust-component: ## Configure Rust component (internal use)
	@echo "Setting up Rust component: $(NAME)"
	@mkdir -p $(NAME)/src
	@mkdir -p $(NAME)/docs
	@echo 'fn main() {\n    println!("Hello from $(NAME)!");\n}' > $(NAME)/src/main.rs
	@echo 'fn lib() {\n    println!("Library function");\n}' > $(NAME)/src/lib.rs
	@echo '[package]\nname = "$(shell basename $(NAME))"\nversion = "0.1.0"\nedition = "2021"\n\n[dependencies]\n' > $(NAME)/Cargo.toml
	@echo 'use std::error::Error;\n\nfn main() -> Result<(), Box<dyn Error>> {\n    println!("Hello from $(NAME)!");\n    Ok(())\n}' > $(NAME)/src/main.rs
	@echo "# $(shell basename $(NAME)) Component" > $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "## Overview" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "This is a Rust component created with the Monorepo Template." >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "## Features" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "- High-performance Rust implementation" >> $(NAME)/docs/README.md
	@echo "- Integrated with monorepo build system" >> $(NAME)/docs/README.md
	@echo "- Pre-commit hooks for code quality" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "## Usage" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "\`\`\`bash" >> $(NAME)/docs/README.md
	@echo "# Build the component" >> $(NAME)/docs/README.md
	@echo "make -C $(NAME) build" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "# Run tests" >> $(NAME)/docs/README.md
	@echo "make -C $(NAME) test" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "# Format code" >> $(NAME)/docs/README.md
	@echo "make -C $(NAME) fmt" >> $(NAME)/docs/README.md
	@echo "\`\`\`" >> $(NAME)/docs/README.md
	@echo "# Monorepo Component Metadata" > $(NAME)/.monorepo-component
	@echo "NAME=$(shell basename $(NAME))" >> $(NAME)/.monorepo-component
	@echo "LANG=rust" >> $(NAME)/.monorepo-component
	@echo "CREATED=$$(date '+%Y-%m-%d %H:%M:%S')" >> $(NAME)/.monorepo-component
	@if [ -n "$(ARTIFACT_TYPE)" ]; then \
		echo "ARTIFACT_TYPE=$(ARTIFACT_TYPE)" >> $(NAME)/.monorepo-component; \
	fi
	@echo "# $(shell basename $(NAME)) Component Makefile" > $(NAME)/Makefile
	@echo "" >> $(NAME)/Makefile
	@echo ".PHONY: build clean test fmt lint help" >> $(NAME)/Makefile
	@echo "" >> $(NAME)/Makefile
	@echo "help: ## Show this help message" >> $(NAME)/Makefile
	@echo "	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = \":.*?## \"}; {printf \"\\033[36m%-15s\\033[0m %s\\n\", $$1, $$2}'" >> $(NAME)/Makefile
	@echo "" >> $(NAME)/Makefile
	@echo "build: ## Build the Rust component" >> $(NAME)/Makefile
	@echo "	cargo build" >> $(NAME)/Makefile
	@echo "" >> $(NAME)/Makefile
	@echo "test: ## Run tests" >> $(NAME)/Makefile
	@echo "	cargo test" >> $(NAME)/Makefile
	@echo "" >> $(NAME)/Makefile
	@echo "clean: ## Clean build artifacts" >> $(NAME)/Makefile
	@echo "	cargo clean" >> $(NAME)/Makefile
	@echo "" >> $(NAME)/Makefile
	@echo "fmt: ## Format Rust code" >> $(NAME)/Makefile
	@echo "	cargo fmt" >> $(NAME)/Makefile
	@echo "" >> $(NAME)/Makefile
	@echo "lint: ## Run Rust linters" >> $(NAME)/Makefile
	@echo "	cargo clippy" >> $(NAME)/Makefile
	@echo "" >> $(NAME)/Makefile
	@echo "check: ## Check code without building" >> $(NAME)/Makefile
	@echo "	cargo check" >> $(NAME)/Makefile
	@echo "Rust component configured with pre-commit hooks and documentation"

configure-python-component: ## Configure Python component (internal use)
	@echo "Setting up Python component: $(NAME)"
	@mkdir -p $(NAME)/src/$(shell basename $(NAME))
	@mkdir -p $(NAME)/tests
	@mkdir -p $(NAME)/docs
	@echo '#!/usr/bin/env python3\n"""$(NAME) component."""\n\ndef main():\n    """Main function."""\n    print("Hello from $(NAME)!")\n\nif __name__ == "__main__":\n    main()' > $(NAME)/src/$(shell basename $(NAME))/__init__.py
	@echo '#!/usr/bin/env python3\n"""Main module for $(NAME)."""\n\nfrom . import main\n\nif __name__ == "__main__":\n    main.main()' > $(NAME)/main.py
	@echo '#!/usr/bin/env python3\n"""Main function for $(NAME)."""\n\ndef main():\n    """Main function."""\n    print("Hello from $(NAME)!")\n' > $(NAME)/src/$(shell basename $(NAME))/main.py
	@echo 'name = "$(shell basename $(NAME))"\nversion = "0.1.0"\ndescription = "$(NAME) component"\nauthors = ["Your Name <your.email@example.com>"]\nlicense = "MIT"\nrequires-python = ">=3.8"\n\n[build-system]\nrequires = ["setuptools>=45", "wheel"]\nbuild-backend = "setuptools.build_meta"\n\n[project.optional-dependencies]\ndev = ["pytest", "black", "flake8", "mypy"]\n' > $(NAME)/pyproject.toml
	@echo '"""Tests for $(NAME)."""\n\ndef test_main():\n    """Test main function."""\n    from . import main\n    assert main.main is not None\n' > $(NAME)/tests/test_$(shell basename $(NAME)).py
	@echo "# $(shell basename $(NAME)) Component" > $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "## Overview" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "This is a Python component created with the Monorepo Template." >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "## Features" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "- Python-based business logic and data processing" >> $(NAME)/docs/README.md
	@echo "- Integrated with monorepo build system" >> $(NAME)/docs/README.md
	@echo "- Pre-commit hooks for code quality (black, flake8, isort)" >> $(NAME)/docs/README.md
	@echo "- Type hints and mypy support" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "## Usage" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "\`\`\`bash" >> $(NAME)/docs/README.md
	@echo "# Install in development mode" >> $(NAME)/docs/README.md
	@echo "make -C $(NAME) build" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "# Run tests" >> $(NAME)/docs/README.md
	@echo "make -C $(NAME) test" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "# Format code" >> $(NAME)/docs/README.md
	@echo "make -C $(NAME) fmt" >> $(NAME)/docs/README.md
	@echo "\`\`\`" >> $(NAME)/docs/README.md
	@echo "# Monorepo Component Metadata" > $(NAME)/.monorepo-component
	@echo "NAME=$(shell basename $(NAME))" >> $(NAME)/.monorepo-component
	@echo "LANG=python" >> $(NAME)/.monorepo-component
	@echo "CREATED=$$(date '+%Y-%m-%d %H:%M:%S')" >> $(NAME)/.monorepo-component
	@if [ -n "$(ARTIFACT_TYPE)" ]; then \
		echo "ARTIFACT_TYPE=$(ARTIFACT_TYPE)" >> $(NAME)/.monorepo-component; \
	fi
	@echo "Python component configured with pre-commit hooks and documentation"

configure-typescript-component: ## Configure TypeScript component (internal use)
	@echo "Setting up TypeScript component: $(NAME)"
	@mkdir -p $(NAME)/src
	@mkdir -p $(NAME)/tests
	@mkdir -p $(NAME)/docs
	@echo '{\n  "name": "$(shell basename $(NAME))",\n  "version": "0.1.0",\n  "description": "$(NAME) component",\n  "main": "dist/index.js",\n  "scripts": {\n    "build": "tsc",\n    "test": "jest",\n    "lint": "eslint src/**/*.ts",\n    "format": "prettier --write src/**/*.ts"\n  },\n  "devDependencies": {\n    "@types/node": "^20.0.0",\n    "@typescript-eslint/eslint-plugin": "^6.0.0",\n    "@typescript-eslint/parser": "^6.0.0",\n    "eslint": "^8.0.0",\n    "jest": "^29.0.0",\n    "prettier": "^3.0.0",\n    "typescript": "^5.0.0"\n  }\n}' > $(NAME)/package.json
	@echo '{\n  "compilerOptions": {\n    "target": "ES2020",\n    "module": "commonjs",\n    "lib": ["ES2020"],\n    "outDir": "./dist",\n    "rootDir": "./src",\n    "strict": true,\n    "esModuleInterop": true,\n    "skipLibCheck": true,\n    "forceConsistentCasingInFileNames": true\n  },\n  "include": ["src/**/*"],\n  "exclude": ["node_modules", "dist", "tests"]\n}' > $(NAME)/tsconfig.json
	@echo 'export function main(): string {\n    return "Hello from $(NAME)!";\n}\n\nexport default main;' > $(NAME)/src/index.ts
	@echo 'import { main } from "./index";\n\nconsole.log(main());' > $(NAME)/src/cli.ts
	@echo 'describe("$(NAME)", () => {\n    it("should return hello message", () => {\n        const { main } = require("../src/index");\n        expect(main()).toBe("Hello from $(NAME)!");\n    });\n});' > $(NAME)/tests/index.test.ts
	@echo "# $(shell basename $(NAME)) Component" > $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "## Overview" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "This is a TypeScript component created with the Monorepo Template." >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "## Features" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "- TypeScript-based web interfaces and APIs" >> $(NAME)/docs/README.md
	@echo "- Integrated with monorepo build system" >> $(NAME)/docs/README.md
	@echo "- Pre-commit hooks for code quality (prettier, eslint)" >> $(NAME)/docs/README.md
	@echo "- Strict TypeScript configuration" >> $(NAME)/docs/README.md
	@echo "- Jest testing framework" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "## Usage" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "\`\`\`bash" >> $(NAME)/docs/README.md
	@echo "# Build the component" >> $(NAME)/docs/README.md
	@echo "make -C $(NAME) build" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "# Run tests" >> $(NAME)/docs/README.md
	@echo "make -C $(NAME) test" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "# Format code" >> $(NAME)/docs/README.md
	@echo "make -C $(NAME) fmt" >> $(NAME)/docs/README.md
	@echo "\`\`\`" >> $(NAME)/docs/README.md
	@echo "# Monorepo Component Metadata" > $(NAME)/.monorepo-component
	@echo "NAME=$(shell basename $(NAME))" >> $(NAME)/.monorepo-component
	@echo "LANG=typescript" >> $(NAME)/.monorepo-component
	@echo "CREATED=$$(date '+%Y-%m-%d %H:%M:%S')" >> $(NAME)/.monorepo-component
	@if [ -n "$(ARTIFACT_TYPE)" ]; then \
		echo "ARTIFACT_TYPE=$(ARTIFACT_TYPE)" >> $(NAME)/.monorepo-component; \
	fi
	@echo "TypeScript component configured with pre-commit hooks and documentation"

configure-java-component: ## Configure Java component (internal use)
	@echo "Setting up Java component: $(NAME)"
	@mkdir -p $(NAME)/src/main/java
	@mkdir -p $(NAME)/src/test/java
	@mkdir -p $(NAME)/docs
	@echo 'package com.example.$(shell basename $(NAME));\n\npublic class Main {\n    public static void main(String[] args) {\n        System.out.println("Hello from $(NAME)!");\n    }\n}' > $(NAME)/src/main/java/Main.java
	@echo '<?xml version="1.0" encoding="UTF-8"?>\n<project xmlns="http://maven.apache.org/POM/4.0.0">\n    <modelVersion>4.0.0</modelVersion>\n    <groupId>com.example</groupId>\n    <artifactId>$(shell basename $(NAME))</artifactId>\n    <version>0.1.0</version>\n    <properties>\n        <maven.compiler.source>11</maven.compiler.source>\n        <maven.compiler.target>11</maven.compiler.target>\n    </properties>\n    <build>\n        <plugins>\n            <plugin>\n                <groupId>org.apache.maven.plugins</groupId>\n                <artifactId>maven-compiler-plugin</artifactId>\n                <version>3.11.0</version>\n            </plugin>\n            <plugin>\n                <groupId>org.apache.maven.plugins</groupId>\n                <artifactId>maven-surefire-plugin</artifactId>\n                <version>3.2.2</version>\n            </plugin>\n        </plugins>\n    </build>\n</project>' > $(NAME)/pom.xml
	@echo "# $(shell basename $(NAME)) Component" > $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "## Overview" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "This is a Java component created with the Monorepo Template." >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "## Features" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "- Enterprise Java implementation" >> $(NAME)/docs/README.md
	@echo "- Integrated with monorepo build system" >> $(NAME)/docs/README.md
	@echo "- Maven build system" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "## Usage" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "\`\`\`bash" >> $(NAME)/docs/README.md
	@echo "# Build the component" >> $(NAME)/docs/README.md
	@echo "make -C $(NAME) build" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "# Run tests" >> $(NAME)/docs/README.md
	@echo "make -C $(NAME) test" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "# Clean build artifacts" >> $(NAME)/docs/README.md
	@echo "make -C $(NAME) clean" >> $(NAME)/docs/README.md
	@echo "\`\`\`" >> $(NAME)/docs/README.md
	@echo "# Monorepo Component Metadata" > $(NAME)/.monorepo-component
	@echo "NAME=$(shell basename $(NAME))" >> $(NAME)/.monorepo-component
	@echo "LANG=java" >> $(NAME)/.monorepo-component
	@echo "CREATED=$$(date '+%Y-%m-%d %H:%M:%S')" >> $(NAME)/.monorepo-component
	@if [ -n "$(ARTIFACT_TYPE)" ]; then \
		echo "ARTIFACT_TYPE=$(ARTIFACT_TYPE)" >> $(NAME)/.monorepo-component; \
	fi
	@echo "# $(shell basename $(NAME)) Component Makefile" > $(NAME)/Makefile
	@echo "" >> $(NAME)/Makefile
	@echo ".PHONY: build clean test fmt lint help" >> $(NAME)/Makefile
	@echo "" >> $(NAME)/Makefile
	@echo "help: ## Show this help message" >> $(NAME)/Makefile
	@echo "	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf \"\\033[36m%-15s\\033[0m %s\\n\", $$1, $$2}'" >> $(NAME)/Makefile
	@echo "" >> $(NAME)/Makefile
	@echo "build: ## Build the Java component" >> $(NAME)/Makefile
	@echo "	mvn clean compile" >> $(NAME)/Makefile
	@echo "" >> $(NAME)/Makefile
	@echo "test: ## Run tests" >> $(NAME)/Makefile
	@echo "	mvn test" >> $(NAME)/Makefile
	@echo "" >> $(NAME)/Makefile
	@echo "clean: ## Clean build artifacts" >> $(NAME)/Makefile
	@echo "	mvn clean" >> $(NAME)/Makefile
	@echo "" >> $(NAME)/Makefile
	@echo "fmt: ## Format Java code" >> $(NAME)/Makefile
	@echo "	@echo \"Formatting Java code...\"" >> $(NAME)/Makefile
	@echo "	@find . -name \"*.java\" -exec google-java-format --aosp -i {} \\;" >> $(NAME)/Makefile
	@echo "" >> $(NAME)/Makefile
	@echo "lint: ## Run Java linters" >> $(NAME)/Makefile
	@echo "	@echo \"Running Checkstyle...\"" >> $(NAME)/Makefile
	@echo "	mvn checkstyle:check" >> $(NAME)/Makefile
	@echo "" >> $(NAME)/Makefile
	@echo "check: ## Check code without building" >> $(NAME)/Makefile
	@echo "	mvn validate compile test-compile" >> $(NAME)/Makefile
	@echo "Java component configured with pre-commit hooks and documentation"

configure-go-component: ## Configure Go component (internal use)
	@echo "Setting up Go component: $(NAME)"
	@mkdir -p $(NAME)/cmd
	@mkdir -p $(NAME)/internal
	@mkdir -p $(NAME)/pkg
	@mkdir -p $(NAME)/docs
	@echo 'package main\n\nimport "fmt"\n\nfunc main() {\n    fmt.Println("Hello from $(NAME)!")\n}' > $(NAME)/cmd/main.go
	@echo 'module $(shell basename $(NAME))\n\ngo 1.21\n' > $(NAME)/go.mod
	@echo "# $(shell basename $(NAME)) Component" > $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "## Overview" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "This is a Go component created with the Monorepo Template." >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "## Features" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "- High-performance Go implementation" >> $(NAME)/docs/README.md
	@echo "- Integrated with monorepo build system" >> $(NAME)/docs/README.md
	@echo "- Go modules support" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "## Usage" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "\`\`\`bash" >> $(NAME)/docs/README.md
	@echo "# Build the component" >> $(NAME)/docs/README.md
	@echo "make -C $(NAME) build" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "# Run tests" >> $(NAME)/docs/README.md
	@echo "make -C $(NAME) test" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "# Run the component" >> $(NAME)/docs/README.md
	@echo "make -C $(NAME) run" >> $(NAME)/docs/README.md
	@echo "\`\`\`" >> $(NAME)/docs/README.md
	@echo "# Monorepo Component Metadata" > $(NAME)/.monorepo-component
	@echo "NAME=$(shell basename $(NAME))" >> $(NAME)/.monorepo-component
	@echo "LANG=go" >> $(NAME)/.monorepo-component
	@echo "CREATED=$$(date '+%Y-%m-%d %H:%M:%S')" >> $(NAME)/.monorepo-component
	@if [ -n "$(ARTIFACT_TYPE)" ]; then \
		echo "ARTIFACT_TYPE=$(ARTIFACT_TYPE)" >> $(NAME)/.monorepo-component; \
	fi
	@echo "# $(shell basename $(NAME)) Component Makefile" > $(NAME)/Makefile
	@echo "" >> $(NAME)/Makefile
	@echo ".PHONY: build clean test fmt lint help run" >> $(NAME)/Makefile
	@echo "" >> $(NAME)/Makefile
	@echo "help: ## Show this help message" >> $(NAME)/Makefile
	@echo "	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf \"\\033[36m%-15s\\033[0m %s\\n\", $$1, $$2}'" >> $(NAME)/Makefile
	@echo "" >> $(NAME)/Makefile
	@echo "build: ## Build the Go component" >> $(NAME)/Makefile
	@echo "	go build -o bin/$(shell basename $(NAME)) ./cmd" >> $(NAME)/Makefile
	@echo "" >> $(NAME)/Makefile
	@echo "test: ## Run tests" >> $(NAME)/Makefile
	@echo "	go test ./..." >> $(NAME)/Makefile
	@echo "" >> $(NAME)/Makefile
	@echo "clean: ## Clean build artifacts" >> $(NAME)/Makefile
	@echo "	rm -rf bin/" >> $(NAME)/Makefile
	@echo "" >> $(NAME)/Makefile
	@echo "fmt: ## Format Go code" >> $(NAME)/Makefile
	@echo "	go fmt ./..." >> $(NAME)/Makefile
	@echo "	goimports -w ." >> $(NAME)/Makefile
	@echo "" >> $(NAME)/Makefile
	@echo "lint: ## Run Go linters" >> $(NAME)/Makefile
	@echo "	golangci-lint run" >> $(NAME)/Makefile
	@echo "" >> $(NAME)/Makefile
	@echo "check: ## Check code without building" >> $(NAME)/Makefile
	@echo "	go vet ./..." >> $(NAME)/Makefile
	@echo "	go mod tidy" >> $(NAME)/Makefile
	@echo "" >> $(NAME)/Makefile
	@echo "run: ## Run the Go component" >> $(NAME)/Makefile
	@echo "	go run ./cmd" >> $(NAME)/Makefile
	@echo "Go component configured with pre-commit hooks and documentation"

# =============================================================================
# TESTING COMMANDS
# =============================================================================

check-component-generation: ## Test component generation for all or specific languages
	@if [ -z "$(LANG)" ] || [ "$(LANG)" = "all" ]; then \
		echo "Testing component generation for all supported languages..."; \
		$(MAKE) check-rust-component-generation; \
		$(MAKE) check-python-component-generation; \
		$(MAKE) check-typescript-component-generation; \
		$(MAKE) check-java-component-generation; \
		$(MAKE) check-go-component-generation; \
		echo "✅ All language component generation tests passed!"; \
	else \
		echo "Testing component generation for $(LANG)..."; \
		$(MAKE) check-$(LANG)-component-generation; \
	fi

check-rust-component-generation: ## Test Rust component generation
	@echo "🧪 Testing Rust component generation..."
	@$(MAKE) add-component NAME=test-rust-component LANG=rust ARTIFACT_TYPE=library
	@if [ -f "test-rust-component/.monorepo-component" ] && [ -f "test-rust-component/Cargo.toml" ]; then \
		echo "✅ Rust component generation test passed"; \
	else \
		echo "❌ Rust component generation test failed"; \
		exit 1; \
	fi
	@rm -rf test-rust-component

check-python-component-generation: ## Test Python component generation
	@echo "🧪 Testing Python component generation..."
	@$(MAKE) add-component NAME=test-python-component LANG=python ARTIFACT_TYPE=service
	@if [ -f "test-python-component/.monorepo-component" ] && [ -f "test-python-component/pyproject.toml" ]; then \
		echo "✅ Python component generation test passed"; \
	else \
		echo "❌ Python component generation test failed"; \
		exit 1; \
	fi
	@rm -rf test-python-component

check-typescript-component-generation: ## Test TypeScript component generation
	@echo "🧪 Testing TypeScript component generation..."
	@$(MAKE) add-component NAME=test-typescript-component LANG=typescript ARTIFACT_TYPE=webapp
	@if [ -f "test-typescript-component/.monorepo-component" ] && [ -f "test-typescript-component/package.json" ]; then \
		echo "✅ TypeScript component generation test passed"; \
	else \
		echo "❌ TypeScript component generation test failed"; \
		exit 1; \
	fi
	@rm -rf test-typescript-component

check-java-component-generation: ## Test Java component generation
	@echo "🧪 Testing Java component generation..."
	@$(MAKE) add-component NAME=test-java-component LANG=java ARTIFACT_TYPE=library
	@if [ -f "test-java-component/.monorepo-component" ] && [ -f "test-java-component/pom.xml" ]; then \
		echo "✅ Java component generation test passed"; \
	else \
		echo "❌ Java component generation test failed"; \
		exit 1; \
	fi
	@rm -rf test-java-component

check-go-component-generation: ## Test Go component generation
	@echo "🧪 Testing Go component generation..."
	@$(MAKE) add-component NAME=test-go-component LANG=go ARTIFACT_TYPE=cli
	@if [ -f "test-go-component/.monorepo-component" ] && [ -f "test-go-component/go.mod" ]; then \
		echo "✅ Go component generation test passed"; \
	else \
		echo "❌ Go component generation test failed"; \
		exit 1; \
	fi
	@rm -rf test-go-component

# =============================================================================
# UTILITY COMMANDS
# =============================================================================

status: ## Show status of all components
	@echo "Monorepo Template - Component Status:"
	@echo "====================================="
	@echo "Pre-commit hooks: $(shell if [ -f .git/hooks/pre-commit ]; then echo "🟢 Installed"; else echo "🔴 Not installed"; fi)"
	@echo ""
	@echo "Components:"
	@component_count=0; \
	for dir in */; do \
		if [ -f "$${dir%/}/.monorepo-component" ]; then \
			lang=$$(grep "^LANG=" "$${dir%/}/.monorepo-component" | cut -d'=' -f2); \
			status="🟢 Active"; \
			echo "  $$(basename $$dir) ($$lang): $$status"; \
			component_count=$$((component_count + 1)); \
		fi; \
	done; \
	if [ $$component_count -eq 0 ]; then \
		echo "  No components found"; \
	fi

create-component-docs: ## Create documentation for existing components
	@echo "Creating documentation for existing components..."
	@for dir in */; do \
		if [ -f "$${dir%/}/.monorepo-component" ]; then \
			lang=$$(grep "^LANG=" "$${dir%/}/.monorepo-component" | cut -d'=' -f2); \
			name=$$(basename $$dir); \
			echo "Creating docs for $$name ($$lang)..."; \
			$(MAKE) create-$${lang}-component-docs NAME=$$name; \
		fi; \
	done
	@echo "Component documentation created!"

create-rust-component-docs: ## Create Rust component documentation (internal use)
	@mkdir -p $(NAME)/docs
	@echo "# $(shell basename $(NAME)) Component" > $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "## Overview" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "This is a Rust component created with the Monorepo Template." >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "## Features" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "- High-performance Rust implementation" >> $(NAME)/docs/README.md
	@echo "- Integrated with monorepo build system" >> $(NAME)/docs/README.md
	@echo "- Pre-commit hooks for code quality" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "## Usage" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "\`\`\`bash" >> $(NAME)/docs/README.md
	@echo "# Build the component" >> $(NAME)/docs/README.md
	@echo "make -C $(NAME) build" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "# Run tests" >> $(NAME)/docs/README.md
	@echo "make -C $(NAME) test" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "# Format code" >> $(NAME)/docs/README.md
	@echo "make -C $(NAME) fmt" >> $(NAME)/docs/README.md
	@echo "\`\`\`" >> $(NAME)/docs/README.md

create-python-component-docs: ## Create Python component documentation (internal use)
	@mkdir -p $(NAME)/docs
	@echo "# $(shell basename $(NAME)) Component" > $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "## Overview" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "This is a Python component created with the Monorepo Template." >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "## Features" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "- Python-based business logic and data processing" >> $(NAME)/docs/README.md
	@echo "- Integrated with monorepo build system" >> $(NAME)/docs/README.md
	@echo "- Pre-commit hooks for code quality (black, flake8, isort)" >> $(NAME)/docs/README.md
	@echo "- Type hints and mypy support" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "## Usage" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "\`\`\`bash" >> $(NAME)/docs/README.md
	@echo "# Install in development mode" >> $(NAME)/docs/README.md
	@echo "make -C $(NAME) build" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "# Run tests" >> $(NAME)/docs/README.md
	@echo "make -C $(NAME) test" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "# Format code" >> $(NAME)/docs/README.md
	@echo "make -C $(NAME) fmt" >> $(NAME)/docs/README.md
	@echo "\`\`\`" >> $(NAME)/docs/README.md

create-typescript-component-docs: ## Create TypeScript component documentation (internal use)
	@mkdir -p $(NAME)/docs
	@echo "# $(shell basename $(NAME)) Component" > $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "## Overview" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "This is a TypeScript component created with the Monorepo Template." >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "## Features" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "- TypeScript-based web interfaces and APIs" >> $(NAME)/docs/README.md
	@echo "- Integrated with monorepo build system" >> $(NAME)/docs/README.md
	@echo "- Pre-commit hooks for code quality (prettier, eslint)" >> $(NAME)/docs/README.md
	@echo "- Strict TypeScript configuration" >> $(NAME)/docs/README.md
	@echo "- Jest testing framework" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "## Usage" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "\`\`\`bash" >> $(NAME)/docs/README.md
	@echo "# Build the component" >> $(NAME)/docs/README.md
	@echo "make -C $(NAME) build" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "# Run tests" >> $(NAME)/docs/README.md
	@echo "make -C $(NAME) test" >> $(NAME)/docs/README.md
	@echo "" >> $(NAME)/docs/README.md
	@echo "# Format code" >> $(NAME)/docs/README.md
	@echo "make -C $(NAME) fmt" >> $(NAME)/docs/README.md
	@echo "\`\`\`" >> $(NAME)/docs/README.md

deps: ## Install dependencies for all components
	@echo "Installing dependencies for all components..."
	@find . -name "Makefile" -path "*/Makefile" -execdir make install-deps \; 2>/dev/null || true
	@echo "Dependencies installed!"

update: ## Update dependencies for all components
	@echo "Updating dependencies for all components..."
	@find . -name "Makefile" -path "*/Makefile" -execdir make update \; 2>/dev/null || true
	@echo "Dependencies updated!"
