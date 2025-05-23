# Development commands using just (https://github.com/casey/just)

# Run the application in development mode
dev:
    cargo run

# Run all tests
test:
    cargo test

# Run tests with coverage
test-coverage:
    cargo tarpaulin --out html

# Run unit tests only
test-unit:
    cargo test --lib

# Run integration tests only
test-integration:
    cargo test --test '*'

# Check code formatting
fmt-check:
    cargo fmt -- --check

# Format code
fmt:
    cargo fmt

# Run clippy lints
lint:
    cargo clippy -- -D warnings

# Run all checks (formatting, linting, tests)
check: fmt-check lint test

# Build release version
build-release:
    cargo build --release

# Clean build artifacts
clean:
    cargo clean

# Watch for changes and run tests
watch-test:
    cargo watch -x test

# Watch for changes and run the app
watch-run:
    cargo watch -x run