// Sapphillon-Core
// Copyright 2025 Yuta Takahashi
//
// This file is part of Sapphillon-Core

use std::env;
use std::path::PathBuf;

fn main() {
    // Determine output path for snapshot
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let snapshot_path = out_dir.join("EXT_PLUGIN_SNAPSHOT.bin");

    // Create snapshot options with default values for a minimal runtime
    let snapshot_options = deno_runtime::ops::bootstrap::SnapshotOptions::default();

    // Generate the runtime snapshot using deno_runtime's built-in function
    deno_runtime::snapshot::create_runtime_snapshot(
        snapshot_path,
        snapshot_options,
        vec![], // No custom extensions for the snapshot
    );
}
