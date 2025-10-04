# weather-service - Monorepo Component

.PHONY: build clean test dev help install-deps fmt lint check

# Default target
build: ## Build this component
	echo "Building weather-service..."
	uv pip install -e .

clean: ## Clean build artifacts
	echo "Cleaning weather-service..."
	rm -rf build/ dist/ *.egg-info/ .pytest_cache/ __pycache__/
	find . -type d -name "__pycache__" -exec rm -rf {} + 2>/dev/null || true
	find . -type f -name "*.pyc" -delete 2>/dev/null || true

test: ## Run tests
	echo "Testing weather-service..."
	uv run pytest tests/ -v

test-coverage: ## Run tests with coverage
	echo "Running tests with coverage..."
	uv run pytest tests/ -v --cov=weather_service --cov-report=html --cov-report=term

dev: ## Run in development mode
	echo "Starting dev mode for weather-service..."
	uv run python main.py

install-deps: ## Install dependencies
	echo "Installing dependencies..."
	uv pip install -e .

fmt: ## Format code
	echo "Formatting code..."
	uv run black .
	uv run isort .

lint: ## Run linters
	echo "Running linters..."
	uv run flake8 .
	uv run mypy .

check: ## Check code without building
	echo "Checking code..."
	uv run black --check .
	uv run isort --check-only .
	uv run flake8 .
	uv run mypy .

help: ## Show this help message
	@echo "weather-service - Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'
