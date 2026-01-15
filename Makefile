.PHONY: test, build, fmt, fix_fmt, buf_generate, build_release

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
