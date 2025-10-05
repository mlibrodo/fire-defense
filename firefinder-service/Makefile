# Component Template Makefile
# This file is used by the top-level Makefile to generate component Makefiles

.PHONY: create-component-makefile

# Create a component-specific Makefile
create-component-makefile:
	@echo "# $(firefinder-service) - Monorepo Component" > $(firefinder-service)/Makefile
	@echo "" >> $(firefinder-service)/Makefile
	@echo ".PHONY: build clean test dev help" >> $(firefinder-service)/Makefile
	@echo "" >> $(firefinder-service)/Makefile
	@echo "# Default target" >> $(firefinder-service)/Makefile
	@echo "build: ## Build this component" >> $(firefinder-service)/Makefile
	@echo "\techo \"Building $(firefinder-service)...\"" >> $(firefinder-service)/Makefile
	@echo "\techo \"Build target not implemented yet for $(firefinder-service)\"" >> $(firefinder-service)/Makefile
	@echo "" >> $(firefinder-service)/Makefile
	@echo "clean: ## Clean build artifacts" >> $(firefinder-service)/Makefile
	@echo "\techo \"Cleaning $(firefinder-service)...\"" >> $(firefinder-service)/Makefile
	@echo "\techo \"Clean target not implemented yet for $(firefinder-service)\"" >> $(firefinder-service)/Makefile
	@echo "" >> $(firefinder-service)/Makefile
	@echo "test: ## Run tests" >> $(firefinder-service)/Makefile
	@echo "\techo \"Testing $(firefinder-service)...\"" >> $(firefinder-service)/Makefile
	@echo "\techo \"Test target not implemented yet for $(firefinder-service)\"" >> $(firefinder-service)/Makefile
	@echo "" >> $(firefinder-service)/Makefile
	@echo "dev: ## Run in development mode" >> $(firefinder-service)/Makefile
	@echo "\techo \"Starting dev mode for $(firefinder-service)...\"" >> $(firefinder-service)/Makefile
	@echo "\techo \"Dev target not implemented yet for $(firefinder-service)\"" >> $(firefinder-service)/Makefile
	@echo "" >> $(firefinder-service)/Makefile
	@echo "help: ## Show this help message" >> $(firefinder-service)/Makefile
	@echo "\t@echo \"$(firefinder-service) - Available targets:\"" >> $(firefinder-service)/Makefile
	@echo "\t@grep -E '^[a-zA-Z_-]+:.*?## .*$$' \$$(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = \":.*?## \"}; {printf \"\\033[36m%-20s\\033[0m %s\\n\", \$$1, \$$2}'" >> $(firefinder-service)/Makefile
