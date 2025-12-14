.PHONY: all build release test test-quick test-bulk test-blocklist clean install check fmt clippy lint run dev help

BINARY_NAME := noorDNS
INSTALL_PATH := /usr/local/bin
CONFIG_PATH := /etc/noorDNS

all: build

build:
	@echo "Building $(BINARY_NAME)..."
	@cargo build

release:
	@echo "Building $(BINARY_NAME) in release mode..."
	@cargo build --release
	@strip target/release/$(BINARY_NAME)

test:
	@echo "Running all tests..."
	@cargo test --locked

test-unit:
	@echo "Running unit tests..."
	@cargo test --lib --locked

test-integration:
	@echo "Running integration tests..."
	@cargo test --test '*' --locked

test-quick:
	@echo "Running quick tests..."
	@./test_quick.sh

test-bulk:
	@echo "Running bulk tests..."
	@./test_bulk.sh

test-blocklist:
	@echo "Running blocklist tests..."
	@./test_blocklists.sh

check:
	@echo "Running cargo check..."
	@cargo check --locked --all-targets

fmt:
	@echo "Formatting code..."
	@cargo fmt --all

fmt-check:
	@echo "Checking code format..."
	@cargo fmt --all -- --check

clippy:
	@echo "Running clippy..."
	@cargo clippy --locked --all-targets -- -D warnings

clippy-fix:
	@echo "Auto-fixing clippy warnings..."
	@cargo clippy --locked --all-targets --fix --allow-dirty --allow-staged

lint: fmt-check clippy

fix: fmt clippy-fix

clean:
	@echo "Cleaning build artifacts..."
	@cargo clean

install: release
	@echo "Installing $(BINARY_NAME) to $(INSTALL_PATH)..."
	@sudo install -m 755 target/release/$(BINARY_NAME) $(INSTALL_PATH)/
	@sudo mkdir -p $(CONFIG_PATH)
	@sudo install -m 644 acl.txt $(CONFIG_PATH)/acl.txt
	@echo "Installed successfully!"

uninstall:
	@echo "Uninstalling $(BINARY_NAME)..."
	@sudo rm -f $(INSTALL_PATH)/$(BINARY_NAME)
	@echo "Uninstalled successfully!"

run:
	@echo "Running $(BINARY_NAME)..."
	@cargo run -- --acl-file acl.txt --upstream 8.8.8.8 --firewall none --bind 127.0.0.1 --bind-port 8053

dev:
	@echo "Running $(BINARY_NAME) in dev mode..."
	@RUST_LOG=debug cargo run -- --acl-file acl.txt --upstream 1.1.1.1 --firewall none --bind 127.0.0.1 --bind-port 8053

deb:
	@echo "Building Debian package..."
	@cargo deb

bench:
	@echo "Running benchmarks..."
	@cargo bench

update:
	@echo "Updating dependencies..."
	@cargo update

upgrade:
	@echo "Upgrading dependencies (requires cargo-edit)..."
	@cargo upgrade --compatible --incompatible --ignore-rust-version
	@cargo update

msrv:
	@echo "Checking MSRV (requires cargo-msrv)..."
	@cargo msrv verify

ci: check lint test
	@echo "CI checks passed!"

pre-commit: fmt clippy test-quick
	@echo "Pre-commit checks passed!"

bloat:
	@echo "Analyzing binary size (requires cargo-bloat)..."
	@cargo bloat --release

audit:
	@echo "Security audit (requires cargo-audit)..."
	@cargo audit

tree:
	@echo "Dependency tree..."
	@cargo tree

outdated:
	@echo "Checking for outdated dependencies (requires cargo-outdated)..."
	@cargo outdated

help:
	@echo "noorDNS Makefile targets:"
	@echo ""
	@echo "  make build          - Build in debug mode"
	@echo "  make release        - Build optimized release binary"
	@echo "  make test           - Run all tests"
	@echo "  make test-unit      - Run unit tests only"
	@echo "  make test-integration - Run integration tests only"
	@echo "  make test-quick     - Run quick test script"
	@echo "  make test-bulk      - Run bulk test script"
	@echo "  make test-blocklist - Run blocklist test script"
	@echo "  make check          - Run cargo check"
	@echo "  make fmt            - Format all code"
	@echo "  make fmt-check      - Check code formatting"
	@echo "  make clippy         - Run clippy linter"
	@echo "  make clippy-fix     - Auto-fix clippy warnings"
	@echo "  make lint           - Run fmt-check and clippy"
	@echo "  make fix            - Auto-format and auto-fix clippy"
	@echo "  make clean          - Remove build artifacts"
	@echo "  make install        - Install to $(INSTALL_PATH) (requires sudo)"
	@echo "  make uninstall      - Remove from $(INSTALL_PATH) (requires sudo)"
	@echo "  make run            - Run server in debug mode"
	@echo "  make dev            - Run server with debug logging"
	@echo "  make deb            - Build Debian package"
	@echo "  make bench          - Run benchmarks"
	@echo "  make update         - Update Cargo.lock"
	@echo "  make upgrade        - Upgrade dependencies (requires cargo-edit)"
	@echo "  make msrv           - Check MSRV (requires cargo-msrv)"
	@echo "  make ci             - Run all CI checks"
	@echo "  make pre-commit     - Run pre-commit checks"
	@echo "  make bloat          - Analyze binary size (requires cargo-bloat)"
	@echo "  make audit          - Security audit (requires cargo-audit)"
	@echo "  make tree           - Show dependency tree"
	@echo "  make outdated       - Check outdated deps (requires cargo-outdated)"
	@echo "  make help           - Show this help"
	@echo ""
