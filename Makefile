# Rojava SaveSlot - Makefile for common development tasks
# Usage: make <target>

.PHONY: help setup build run test clean fmt lint check watch docs docker dev prod install audit outdated coverage release

# Default target
.DEFAULT_GOAL := help

# Variables
CARGO := cargo
DOCKER := docker
DOCKER_COMPOSE := docker-compose
PROJECT_NAME := rojava-saveslot
RUST_VERSION := 1.75

# Colors for output
BLUE := \033[0;34m
GREEN := \033[0;32m
YELLOW := \033[1;33m
RED := \033[0;31m
NC := \033[0m # No Color

# Help target - shows available commands
help: ## Show this help message
	@echo "$(BLUE)Rojava SaveSlot - Development Commands$(NC)"
	@echo "======================================"
	@echo ""
	@echo "$(GREEN)Setup Commands:$(NC)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | grep -E "(setup|install)" | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(BLUE)%-15s$(NC) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(GREEN)Development Commands:$(NC)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | grep -E "(build|run|dev|watch)" | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(BLUE)%-15s$(NC) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(GREEN)Testing Commands:$(NC)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | grep -E "(test|check|coverage)" | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(BLUE)%-15s$(NC) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(GREEN)Code Quality Commands:$(NC)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | grep -E "(fmt|lint|audit)" | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(BLUE)%-15s$(NC) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(GREEN)Docker Commands:$(NC)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | grep -E "(docker)" | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(BLUE)%-15s$(NC) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(GREEN)Other Commands:$(NC)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | grep -vE "(setup|install|build|run|dev|watch|test|check|coverage|fmt|lint|audit|docker)" | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(BLUE)%-15s$(NC) %s\n", $$1, $$2}'

# Setup and Installation
setup: ## Run the complete setup script
	@echo "$(BLUE)Setting up Rojava SaveSlot development environment...$(NC)"
	@chmod +x setup.sh
	@./setup.sh

install: ## Install Rust and development dependencies
	@echo "$(BLUE)Installing Rust and dependencies...$(NC)"
	@if ! command -v rustc >/dev/null 2>&1; then \
		echo "$(YELLOW)Installing Rust...$(NC)"; \
		curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; \
		source ~/.cargo/env; \
	fi
	@rustup update stable
	@rustup component add rustfmt clippy
	@$(CARGO) install cargo-watch cargo-tarpaulin cargo-audit cargo-outdated || true

# Development Commands
build: ## Build the project in debug mode
	@echo "$(BLUE)Building project...$(NC)"
	@$(CARGO) build

build-release: ## Build the project in release mode
	@echo "$(BLUE)Building project in release mode...$(NC)"
	@$(CARGO) build --release

run: ## Run the application
	@echo "$(BLUE)Running Rojava SaveSlot...$(NC)"
	@$(CARGO) run

dev: ## Run in development mode with hot reload
	@echo "$(BLUE)Starting development mode with hot reload...$(NC)"
	@$(CARGO) watch -x run

watch: ## Watch for changes and rebuild
	@echo "$(BLUE)Watching for file changes...$(NC)"
	@$(CARGO) watch -x check

watch-test: ## Watch for changes and run tests
	@echo "$(BLUE)Watching for changes and running tests...$(NC)"
	@$(CARGO) watch -x test

# Testing Commands
test: ## Run all tests
	@echo "$(BLUE)Running all tests...$(NC)"
	@$(CARGO) test

test-unit: ## Run unit tests only
	@echo "$(BLUE)Running unit tests...$(NC)"
	@$(CARGO) test --lib

test-integration: ## Run integration tests only
	@echo "$(BLUE)Running integration tests...$(NC)"
	@$(CARGO) test --test '*'

test-verbose: ## Run tests with verbose output
	@echo "$(BLUE)Running tests with verbose output...$(NC)"
	@$(CARGO) test -- --nocapture

coverage: ## Generate test coverage report
	@echo "$(BLUE)Generating test coverage report...$(NC)"
	@$(CARGO) tarpaulin --out html --output-dir coverage
	@echo "$(GREEN)Coverage report generated in coverage/tarpaulin-report.html$(NC)"

# Code Quality Commands
check: ## Run all code quality checks
	@echo "$(BLUE)Running all code quality checks...$(NC)"
	@$(MAKE) fmt-check
	@$(MAKE) lint
	@$(MAKE) test

fmt: ## Format code
	@echo "$(BLUE)Formatting code...$(NC)"
	@$(CARGO) fmt

fmt-check: ## Check code formatting
	@echo "$(BLUE)Checking code formatting...$(NC)"
	@$(CARGO) fmt -- --check

lint: ## Run clippy lints
	@echo "$(BLUE)Running clippy lints...$(NC)"
	@$(CARGO) clippy -- -D warnings

lint-fix: ## Run clippy and automatically fix issues
	@echo "$(BLUE)Running clippy with automatic fixes...$(NC)"
	@$(CARGO) clippy --fix --allow-dirty --allow-staged

audit: ## Run security audit
	@echo "$(BLUE)Running security audit...$(NC)"
	@$(CARGO) audit

outdated: ## Check for outdated dependencies
	@echo "$(BLUE)Checking for outdated dependencies...$(NC)"
	@$(CARGO) outdated

# Documentation
docs: ## Generate and open documentation
	@echo "$(BLUE)Generating documentation...$(NC)"
	@$(CARGO) doc --open

docs-build: ## Build documentation without opening
	@echo "$(BLUE)Building documentation...$(NC)"
	@$(CARGO) doc --no-deps

# Docker Commands
docker-build: ## Build Docker image
	@echo "$(BLUE)Building Docker image...$(NC)"
	@$(DOCKER) build -t $(PROJECT_NAME) .

docker-build-dev: ## Build development Docker image
	@echo "$(BLUE)Building development Docker image...$(NC)"
	@$(DOCKER) build -f Dockerfile.dev -t $(PROJECT_NAME)-dev .

docker-run: ## Run Docker container
	@echo "$(BLUE)Running Docker container...$(NC)"
	@$(DOCKER) run -it --rm $(PROJECT_NAME)

docker-dev: ## Start development environment with Docker Compose
	@echo "$(BLUE)Starting development environment...$(NC)"
	@$(DOCKER_COMPOSE) up -d

docker-dev-logs: ## Show logs from development environment
	@$(DOCKER_COMPOSE) logs -f

docker-dev-stop: ## Stop development environment
	@echo "$(BLUE)Stopping development environment...$(NC)"
	@$(DOCKER_COMPOSE) down

docker-dev-clean: ## Stop and clean development environment
	@echo "$(BLUE)Cleaning development environment...$(NC)"
	@$(DOCKER_COMPOSE) down -v --remove-orphans

# Database Commands
db-setup: ## Set up database with Docker
	@echo "$(BLUE)Setting up database...$(NC)"
	@$(DOCKER_COMPOSE) up -d postgres
	@sleep 5
	@echo "$(GREEN)Database is ready at localhost:5432$(NC)"

db-migrate: ## Run database migrations (when implemented)
	@echo "$(BLUE)Running database migrations...$(NC)"
	@echo "$(YELLOW)Database migrations not yet implemented$(NC)"

db-reset: ## Reset database
	@echo "$(BLUE)Resetting database...$(NC)"
	@$(DOCKER_COMPOSE) down postgres
	@$(DOCKER) volume rm rojava-saveslot_postgres_data || true
	@$(MAKE) db-setup

# Cleanup Commands
clean: ## Clean build artifacts
	@echo "$(BLUE)Cleaning build artifacts...$(NC)"
	@$(CARGO) clean

clean-all: ## Clean everything including Docker
	@echo "$(BLUE)Cleaning everything...$(NC)"
	@$(CARGO) clean
	@$(DOCKER_COMPOSE) down -v --remove-orphans || true
	@$(DOCKER) system prune -f || true

# Release Commands
release: ## Create a release build and package
	@echo "$(BLUE)Creating release...$(NC)"
	@$(MAKE) check
	@$(MAKE) build-release
	@echo "$(GREEN)Release build completed successfully!$(NC)"

release-tag: ## Tag the current version for release
	@echo "$(BLUE)Tagging release...$(NC)"
	@read -p "Enter version (e.g., v0.1.0): " version; \
	git tag -a $$version -m "Release $$version"; \
	git push origin $$version
	@echo "$(GREEN)Release tagged successfully!$(NC)"

# Utility Commands
deps: ## Show dependency tree
	@echo "$(BLUE)Showing dependency tree...$(NC)"
	@$(CARGO) tree

size: ## Show binary size information
	@echo "$(BLUE)Analyzing binary size...$(NC)"
	@$(CARGO) build --release
	@ls -lh target/release/$(PROJECT_NAME) || ls -lh target/release/datamind-ide

bench: ## Run benchmarks (when implemented)
	@echo "$(BLUE)Running benchmarks...$(NC)"
	@$(CARGO) bench

profile: ## Profile the application (requires cargo-flamegraph)
	@echo "$(BLUE)Profiling application...$(NC)"
	@$(CARGO) flamegraph --bin $(PROJECT_NAME)

# Git Commands
git-setup: ## Set up git hooks and configuration
	@echo "$(BLUE)Setting up git hooks...$(NC)"
	@cp scripts/pre-commit .git/hooks/pre-commit || echo "pre-commit hook not found"
	@chmod +x .git/hooks/pre-commit || true
	@echo "$(GREEN)Git hooks set up successfully!$(NC)"

# Environment Commands
env-check: ## Check development environment
	@echo "$(BLUE)Checking development environment...$(NC)"
	@echo "Rust version: $$(rustc --version)"
	@echo "Cargo version: $$(cargo --version)"
	@echo "Docker version: $$(docker --version 2>/dev/null || echo 'Not installed')"
	@echo "Docker Compose version: $$(docker-compose --version 2>/dev/null || echo 'Not installed')"
	@echo "Git version: $$(git --version)"

# Quick development workflow
quick: ## Quick development workflow (format, lint, test, run)
	@echo "$(BLUE)Running quick development workflow...$(NC)"
	@$(MAKE) fmt
	@$(MAKE) lint
	@$(MAKE) test
	@$(MAKE) run

# CI/CD simulation
ci: ## Simulate CI/CD pipeline
	@echo "$(BLUE)Simulating CI/CD pipeline...$(NC)"
	@$(MAKE) fmt-check
	@$(MAKE) lint
	@$(MAKE) test
	@$(MAKE) build-release
	@$(MAKE) audit
	@echo "$(GREEN)CI/CD simulation completed successfully!$(NC)"

# Performance testing
perf: ## Run performance tests
	@echo "$(BLUE)Running performance tests...$(NC)"
	@$(CARGO) test --release -- --ignored perf

# Update everything
update: ## Update Rust, dependencies, and tools
	@echo "$(BLUE)Updating everything...$(NC)"
	@rustup update
	@$(CARGO) update
	@$(CARGO) install cargo-watch cargo-tarpaulin cargo-audit cargo-outdated --force
	@echo "$(GREEN)Everything updated successfully!$(NC)"