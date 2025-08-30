# data-source-irwin Component Makefile

.PHONY: build clean test fmt lint help

help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$'  Makefile | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "[36m%-15s[0m %s
", , }'

build: ## Build the Rust component
	cargo build

test: ## Run tests
	cargo test

clean: ## Clean build artifacts
	cargo clean

fmt: ## Format Rust code
	cargo fmt

lint: ## Run Rust linters
	cargo clippy

check: ## Check code without building
	cargo check
