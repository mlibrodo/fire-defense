# Component Template Makefile
# This file is used by the top-level Makefile to generate component Makefiles

.PHONY: create-component-makefile

# Create a component-specific Makefile
create-component-makefile:
	@echo "# $(wind-data-playground) - Monorepo Component" > $(wind-data-playground)/Makefile
	@echo "" >> $(wind-data-playground)/Makefile
	@echo ".PHONY: build clean test dev help" >> $(wind-data-playground)/Makefile
	@echo "" >> $(wind-data-playground)/Makefile
	@echo "# Default target" >> $(wind-data-playground)/Makefile
	@echo "build: ## Build this component" >> $(wind-data-playground)/Makefile
	@echo "\techo \"Building $(wind-data-playground)...\"" >> $(wind-data-playground)/Makefile
	@echo "\techo \"Build target not implemented yet for $(wind-data-playground)\"" >> $(wind-data-playground)/Makefile
	@echo "" >> $(wind-data-playground)/Makefile
	@echo "clean: ## Clean build artifacts" >> $(wind-data-playground)/Makefile
	@echo "\techo \"Cleaning $(wind-data-playground)...\"" >> $(wind-data-playground)/Makefile
	@echo "\techo \"Clean target not implemented yet for $(wind-data-playground)\"" >> $(wind-data-playground)/Makefile
	@echo "" >> $(wind-data-playground)/Makefile
	@echo "test: ## Run tests" >> $(wind-data-playground)/Makefile
	@echo "\techo \"Testing $(wind-data-playground)...\"" >> $(wind-data-playground)/Makefile
	@echo "\techo \"Test target not implemented yet for $(wind-data-playground)\"" >> $(wind-data-playground)/Makefile
	@echo "" >> $(wind-data-playground)/Makefile
	@echo "dev: ## Run in development mode" >> $(wind-data-playground)/Makefile
	@echo "\techo \"Starting dev mode for $(wind-data-playground)...\"" >> $(wind-data-playground)/Makefile
	@echo "\techo \"Dev target not implemented yet for $(wind-data-playground)\"" >> $(wind-data-playground)/Makefile
	@echo "" >> $(wind-data-playground)/Makefile
	@echo "help: ## Show this help message" >> $(wind-data-playground)/Makefile
	@echo "\t@echo \"$(wind-data-playground) - Available targets:\"" >> $(wind-data-playground)/Makefile
	@echo "\t@grep -E '^[a-zA-Z_-]+:.*?## .*$$' \$$(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = \":.*?## \"}; {printf \"\\033[36m%-20s\\033[0m %s\\n\", \$$1, \$$2}'" >> $(wind-data-playground)/Makefile
