.PHONY: rust_test, rust_build, rust_check_format, rust_fix_format, buf_generate

buf_generate:
	@echo "Generate Protocol Buffer Code"
	@echo "----------------------------------------------------------"
	buf generate
	@echo "----------------------------------------------------------"

rust_test:
	@echo "Run Rust Tests"
	@echo "----------------------------------------------------------"
	RUST_TEST_THREADS=1 cargo test --lib --workspace --all-features
	@echo "----------------------------------------------------------"

rust_build:
	@echo "Build Rust Project"
	@echo "----------------------------------------------------------"
	cargo build --workspace --all-features
	@echo "----------------------------------------------------------"

rust_check_format:
	@echo "Check Rust Format"
	@echo "----------------------------------------------------------"
	cargo fmt --check || true
	@echo "----------------------------------------------------------"
	cargo clippy --workspace || true
	@echo "----------------------------------------------------------"

rust_fix_format:
	@echo "Fix Rust Format"
	@echo "----------------------------------------------------------"
	cargo fmt || true
	@echo "----------------------------------------------------------"
	cargo clippy --workspace --fix --allow-dirty || true
	@echo "----------------------------------------------------------"

