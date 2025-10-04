# weather-service Component

## Overview

This is a Python component created with the Monorepo Template for weather data processing and services.

## Features

- Python-based weather data processing and analysis
- Integrated with monorepo build system
- Pre-commit hooks for code quality (black, flake8, isort)
- Type hints and mypy support
- UV package management for fast dependency handling

## Usage

```bash
# Install in development mode
make -C weather-service build

# Run tests
make -C weather-service test

# Format code
make -C weather-service fmt

# Run the application
cd weather-service
uv run python main.py
```
