# Component Template Makefile
# This file is used by the top-level Makefile to generate component Makefiles

.PHONY: create-component-makefile

# Create a component-specific Makefile
create-component-makefile:
	@echo "# $(COMPONENT_NAME) - Monorepo Component" > $(COMPONENT_NAME)/Makefile
	@echo "" >> $(COMPONENT_NAME)/Makefile
	@echo ".PHONY: build clean test dev help" >> $(COMPONENT_NAME)/Makefile
	@echo "" >> $(COMPONENT_NAME)/Makefile
	@echo "# Default target" >> $(COMPONENT_NAME)/Makefile
	@echo "build: ## Build this component" >> $(COMPONENT_NAME)/Makefile
	@echo "\techo \"Building $(COMPONENT_NAME)...\"" >> $(COMPONENT_NAME)/Makefile
	@echo "\techo \"Build target not implemented yet for $(COMPONENT_NAME)\"" >> $(COMPONENT_NAME)/Makefile
	@echo "" >> $(COMPONENT_NAME)/Makefile
	@echo "clean: ## Clean build artifacts" >> $(COMPONENT_NAME)/Makefile
	@echo "\techo \"Cleaning $(COMPONENT_NAME)...\"" >> $(COMPONENT_NAME)/Makefile
	@echo "\techo \"Clean target not implemented yet for $(COMPONENT_NAME)\"" >> $(COMPONENT_NAME)/Makefile
	@echo "" >> $(COMPONENT_NAME)/Makefile
	@echo "test: ## Run tests" >> $(COMPONENT_NAME)/Makefile
	@echo "\techo \"Testing $(COMPONENT_NAME)...\"" >> $(COMPONENT_NAME)/Makefile
	@echo "\techo \"Test target not implemented yet for $(COMPONENT_NAME)\"" >> $(COMPONENT_NAME)/Makefile
	@echo "" >> $(COMPONENT_NAME)/Makefile
	@echo "dev: ## Run in development mode" >> $(COMPONENT_NAME)/Makefile
	@echo "\techo \"Starting dev mode for $(COMPONENT_NAME)...\"" >> $(COMPONENT_NAME)/Makefile
	@echo "\techo \"Dev target not implemented yet for $(COMPONENT_NAME)\"" >> $(COMPONENT_NAME)/Makefile
	@echo "" >> $(COMPONENT_NAME)/Makefile
	@echo "help: ## Show this help message" >> $(COMPONENT_NAME)/Makefile
	@echo "\t@echo \"$(COMPONENT_NAME) - Available targets:\"" >> $(COMPONENT_NAME)/Makefile
	@echo "\t@grep -E '^[a-zA-Z_-]+:.*?## .*$$' \$$(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = \":.*?## \"}; {printf \"\\033[36m%-20s\\033[0m %s\\n\", \$$1, \$$2}'" >> $(COMPONENT_NAME)/Makefile
