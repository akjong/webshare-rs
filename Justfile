# Default recipe to display help
default:
  @just --list

# Format all code
format:
  rumdl fmt .
  taplo fmt
  cargo +nightly fmt --all

# Auto-fix linting issues
fix:
  rumdl check --fix .

# Run all lints
lint:
  typos
  rumdl check .
  taplo fmt --check
  cargo +nightly fmt --all -- --check
  cargo +nightly clippy --all -- -D warnings
  cargo machete

# Run tests
test:
  cargo test --all-features

# Run integration tests against the real Webshare API
# Requires WEBSHARE_API_KEY to be set in the environment:
#   WEBSHARE_API_KEY=<your_key> just test-integration
test-integration:
  cargo test --test integration --all-features -- --test-threads=1

# Run all tests (unit + integration + doc) sequentially
# Requires WEBSHARE_API_KEY to be set in the environment
test-all:
  cargo test --all-features -- --test-threads=1

# Run tests with coverage
test-coverage:
  cargo tarpaulin --all-features --workspace --timeout 300

# Build entire workspace
build:
  cargo build --workspace

# Check all targets compile
check:
  cargo check --all-targets --all-features

# Check for Chinese characters
check-cn:
  rg --line-number --column "\p{Han}"

# Full CI check (unit tests only; integration tests require WEBSHARE_API_KEY)
ci: lint test build

# Publish crate to crates.io (requires CARGO_REGISTRY_TOKEN in environment)
publish:
  cargo publish

# ============================================================
# Maintenance & Tools
# ============================================================

# Clean build artifacts
clean:
  cargo clean

# Install all required development tools
setup:
  cargo install cargo-machete
  cargo install taplo-cli
  cargo install typos-cli

# Generate documentation for the workspace
docs:
  cargo doc --no-deps --open
