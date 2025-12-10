.PHONY: build run test clean help check fmt clippy

help:
	@echo "trace-deck CLI - Rust Document Processing"
	@echo ""
	@echo "Build Commands:"
	@echo "  make build        - Build optimized release binary"
	@echo "  make dev          - Build debug binary"
	@echo "  make check        - Check compilation without building"
	@echo ""
	@echo "Run Commands:"
	@echo "  make run          - Build and show app info"
	@echo "  make run-help     - Show CLI help"
	@echo ""
	@echo "Testing & Quality:"
	@echo "  make test         - Run tests"
	@echo "  make fmt          - Format code"
	@echo "  make clippy       - Lint code"
	@echo ""
	@echo "Maintenance:"
	@echo "  make clean        - Remove build artifacts"
	@echo ""

check:
	@echo "Checking project..."
	cargo check

build:
	@echo "Building trace-deck (release)..."
	cargo build --release
	@echo ""
	@echo "✓ Build successful!"
	@echo "Binary: ./target/release/trace-deck"
	@echo ""
	@echo "Try: ./target/release/trace-deck --help"
	@echo "Or:  make run"

dev:
	@echo "Building trace-deck (debug)..."
	cargo build
	@echo "Binary: ./target/debug/trace-deck"

run: build
	@./target/release/trace-deck info

run-help: build
	@./target/release/trace-deck --help

test:
	@echo "Running tests..."
	cargo test

fmt:
	@echo "Formatting code..."
	cargo fmt

clippy:
	@echo "Linting code..."
	cargo clippy --all-targets --all-features

clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	@echo "✓ Clean complete"
