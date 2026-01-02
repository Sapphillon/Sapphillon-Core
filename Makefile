.PHONY: rust_test, rust_build, rust_check_format, rust_fix_format, buf_generate

ifeq ($(OS),Windows_NT)
RUST_TEST_CMD = set "RUST_TEST_THREADS=1" && set "AWS_LC_SYS_PREBUILT_NASM=1" && set "CMAKE_POLICY_VERSION_MINIMUM=3.5" && cargo test --workspace --all-features --all-targets
else
RUST_TEST_CMD = RUST_TEST_THREADS=1 cargo test --workspace --all-features --all-targets
endif

buf_generate:
	@echo "Generate Protocol Buffer Code"
	@echo "----------------------------------------------------------"
	buf generate
	@echo "----------------------------------------------------------"

rust_test:
	@echo "Run Rust Tests"
	@echo "----------------------------------------------------------"
	@$(RUST_TEST_CMD)
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
