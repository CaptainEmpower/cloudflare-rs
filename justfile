# Cloudflare Rust SDK Justfile
# 
# Quick start:
#   just validate-dev  # Development validation (warnings only)
#   just validate      # Full validation (CI quality - fails on warnings)
#   just quick         # Fast subset for development iteration
#   just ci            # Complete CI pipeline simulation
#
# Run `just --list` to see all available commands

set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

# Default recipe - show available commands
default:
    @just --list

# Run comprehensive validation suite (strict - CI quality)
validate: fmt-check clippy test audit doc-check

# Run all validation steps with verbose output (strict)
validate-verbose: fmt-check-verbose clippy-verbose test-verbose audit-verbose doc-check-verbose

# Run validation in development mode (warnings don't fail)
validate-dev: fmt-check clippy-dev test doc-check

# Format code with rustfmt
fmt:
    @echo "🔧 Formatting code..."
    cargo fmt --all

# Check formatting without making changes
fmt-check:
    @echo "📝 Checking formatting..."
    @cargo fmt --all -- --check

# Check formatting with verbose output
fmt-check-verbose:
    @echo "📝 Checking formatting (verbose)..."
    cargo fmt --all -- --check

# Run Clippy lints (strict - fail on warnings)
clippy:
    @echo "🔍 Running Clippy..."
    @cargo clippy --all-targets --all-features -- -D warnings

# Run Clippy with verbose output (strict)
clippy-verbose:
    @echo "🔍 Running Clippy (verbose)..."
    cargo clippy --all-targets --all-features -- -D warnings

# Run Clippy in development mode (warnings only, no failure)
clippy-dev:
    @echo "🔍 Running Clippy (development mode)..."
    @cargo clippy --all-targets --all-features

# Run Clippy and automatically fix issues
clippy-fix:
    @echo "🔧 Running Clippy with auto-fix..."
    cargo clippy --all-targets --all-features --fix --allow-dirty --allow-staged

# Run tests
test:
    @echo "🧪 Running tests..."
    @cargo test --all-features

# Run tests with verbose output
test-verbose:
    @echo "🧪 Running tests (verbose)..."
    cargo test --all-features -- --nocapture

# Run tests with coverage (requires cargo-tarpaulin)
test-coverage:
    @echo "📊 Running tests with coverage..."
    cargo tarpaulin --all-features --out Html --output-dir coverage

# Run security audit (requires cargo-audit)
audit:
    @echo "🔒 Running security audit..."
    @if command -v cargo-audit >/dev/null 2>&1; then \
        cargo audit; \
    else \
        echo "⚠️  cargo-audit not installed. Install with: cargo install cargo-audit"; \
        echo "Skipping audit..."; \
    fi

# Run security audit with verbose output
audit-verbose:
    @echo "🔒 Running security audit (verbose)..."
    @if command -v cargo-audit >/dev/null 2>&1; then \
        cargo audit --verbose; \
    else \
        echo "⚠️  cargo-audit not installed. Install with: cargo install cargo-audit"; \
        echo "Skipping audit..."; \
    fi

# Check documentation builds
doc-check:
    @echo "📚 Checking documentation..."
    @RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps

# Check documentation with verbose output
doc-check-verbose:
    @echo "📚 Checking documentation (verbose)..."
    RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps --verbose

# Build documentation and open it
doc:
    @echo "📚 Building and opening documentation..."
    cargo doc --all-features --no-deps --open

# Check for outdated dependencies
outdated:
    @echo "📦 Checking for outdated dependencies..."
    @if command -v cargo-outdated >/dev/null 2>&1; then \
        cargo outdated; \
    else \
        echo "⚠️  cargo-outdated not installed. Install with: cargo install cargo-outdated"; \
        echo "Skipping outdated check..."; \
    fi

# Clean all build artifacts
clean:
    @echo "🧹 Cleaning build artifacts..."
    cargo clean

# Build in release mode
build-release:
    @echo "🏗️  Building in release mode..."
    cargo build --release --all-features

# Build all targets
build-all:
    @echo "🏗️  Building all targets..."
    cargo build --all-targets --all-features

# Run benchmarks (if any exist)
bench:
    @echo "⚡ Running benchmarks..."
    cargo bench --all-features

# Check if code compiles without building
check:
    @echo "✅ Checking compilation..."
    @cargo check --all-targets --all-features

# Run example code
examples:
    @echo "🚀 Running examples..."
    cargo run --example cloudflare-examples --features spec

# Install development dependencies
install-dev-deps:
    @echo "📦 Installing development dependencies..."
    cargo install cargo-audit cargo-outdated cargo-tarpaulin

# Full CI pipeline simulation
ci: clean check validate build-all examples doc-check
    @echo "✅ CI pipeline completed successfully!"

# Quick validation (faster subset for development)
quick: fmt-check clippy check test
    @echo "⚡ Quick validation completed!"

# Pre-commit hook simulation
pre-commit: fmt clippy-fix test
    @echo "🪝 Pre-commit validation completed!"

# Check for common issues and run comprehensive validation
health-check: check validate outdated
    @echo "🏥 Health check completed!"

# Show project statistics
stats:
    @echo "📈 Project Statistics:"
    @echo "Lines of code:"
    @find . -name "*.rs" -not -path "./target/*" -not -path "./tmp/*" | xargs wc -l | tail -n 1
    @echo ""
    @echo "Number of Rust files:"
    @find . -name "*.rs" -not -path "./target/*" -not -path "./tmp/*" | wc -l
    @echo ""
    @echo "Workspace members:"
    @cargo metadata --format-version 1 | jq -r '.workspace_members[]' | wc -l

# Watch for changes and run validation
watch:
    @echo "👀 Watching for changes (requires cargo-watch)..."
    @if command -v cargo-watch >/dev/null 2>&1; then \
        cargo watch -x "check" -x "test"; \
    else \
        echo "⚠️  cargo-watch not installed. Install with: cargo install cargo-watch"; \
    fi