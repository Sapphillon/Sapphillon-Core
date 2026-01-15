.PHONY: help buf_generate test build fmt fix_fmt build_release clean

help:
	@echo "Usage: make <target>"
	@echo ""
	@echo "Available targets:"
	@echo "  help            : Show this help message"
	@echo "  buf_generate    : Generate Protocol Buffer Code"
	@echo "  test            : Build extplugin_test_server binary and run Rust tests"
	@echo "  build           : Build Rust project (all features)"
	@echo "  fmt             : Check Rust format and run clippy (non-fatal)"
	@echo "  fix_fmt         : Fix Rust format and run clippy fixes"
	@echo "  build_release   : Build Rust project in release mode"

buf_generate:
	@echo "Generate Protocol Buffer Code"
	@echo "----------------------------------------------------------"
	buf generate
	@echo "----------------------------------------------------------"

test:
	@echo "Build extplugin_test_server binary"
	@echo "----------------------------------------------------------"
	cargo build --package ext_plugin --bin extplugin_test_server
	@echo "----------------------------------------------------------"
	@echo "Run Rust Tests"
	@echo "----------------------------------------------------------"
	cargo test --workspace --all-features --all-targets
	@echo "----------------------------------------------------------"

build:
	@echo "Build Rust Project"
	@echo "----------------------------------------------------------"
	cargo build --workspace --all-features
	@echo "----------------------------------------------------------"

fmt:
	@echo "Check Rust Format"
	@echo "----------------------------------------------------------"
	cargo fmt --check || true
	@echo "----------------------------------------------------------"
	cargo clippy --workspace || true
	@echo "----------------------------------------------------------"

fix_fmt:
	@echo "Fix Rust Format"
	@echo "----------------------------------------------------------"
	cargo fmt || true
	@echo "----------------------------------------------------------"
	cargo clippy --workspace --fix --allow-dirty || true
	@echo "----------------------------------------------------------"

build_release:
	@echo "Build Rust Project in Release Mode"
	@echo "----------------------------------------------------------"
	cargo build --workspace --all-features --release
	@echo "----------------------------------------------------------"

clean:
	@echo "Cleaning build artifacts"
	@echo "----------------------------------------------------------"
	cargo clean
	rm -rf target
	@echo "----------------------------------------------------------"
