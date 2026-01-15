# Developers — Makefile targets

This file documents the `Makefile` targets and their purpose so contributors know what commands to run.

- `help`: Show a short help message listing available `make` targets.
- `buf_generate`: Generate Protocol Buffer code (runs `buf generate`).
- `test`: Build the `extplugin_test_server` binary and run the Rust test suite for the workspace (`cargo test --workspace --all-features --all-targets`).
- `build`: Build the Rust project with all features (`cargo build --workspace --all-features`).
- `fmt`: Check Rust formatting with `cargo fmt --check` and run `cargo clippy` (non-fatal).
- `fix_fmt`: Automatically format code (`cargo fmt`) and attempt to fix clippy warnings (`cargo clippy --fix --allow-dirty`).
- `build_release`: Build the Rust project in release mode (`cargo build --workspace --all-features --release`).

- `clean`: Remove build artifacts (`cargo clean` and `rm -rf target`).

If you update targets in the `Makefile`, please keep this document in sync.
