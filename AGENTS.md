# Sapphillon Core Agent Instructions

This document provides instructions for AI agents working with the `sapphillon_core` repository.

## Project Overview

`sapphillon_core` is a Rust-based workflow runtime. It forms the core of the Sapphillon project, providing the necessary components to execute workflows.

## Technology Stack

The key technologies used in this project are:

- **Rust**: The primary programming language.
- **Tonic**: A gRPC framework for Rust.
- **Prost**: A Protocol Buffers implementation for Rust.
- **Deno**: Used as a JavaScript/TypeScript runtime.

## Development

### Building the Project

To build the project, run the following command:

```bash
make rust_build
```

### Running Tests

To run the test suite, use the following command:

```bash
make rust_test
```

### Formatting and Linting

This project uses `rustfmt` for formatting and `clippy` for linting.

To check the formatting, run:

```bash
make rust_check_format
```

To automatically fix formatting and linting issues, run:

```bash
make rust_fix_format
```

## Protocol Buffers

This project uses Protocol Buffers for defining gRPC services and messages. The `.proto` files are located in the `src/proto` directory.

If you modify any of the `.proto` files, you must regenerate the Rust code by running:

```bash
make buf_generate
```
