// Sapphillon-Core
// Copyright 2025 Yuta Takahashi
//
// This file is part of Sapphillon-Core

use std::env;
use std::path::PathBuf;

fn main() {
    #[cfg(not(windows))]
    build_snapshot();

    #[cfg(windows)]
    println!("cargo:warning=Skipping snapshot generation on Windows to avoid DLL linking issues.");
}

#[cfg(not(windows))]
fn build_snapshot() {
    // Determine output path for snapshot
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let snapshot_path = out_dir.join("EXT_PLUGIN_SNAPSHOT.bin");

    // Create snapshot options with default values for a minimal runtime
    let snapshot_options = deno_runtime::ops::bootstrap::SnapshotOptions::default();

    // Generate the runtime snapshot using deno_runtime's built-in function.
    // Wrap in catch_unwind to avoid aborting the build if the snapshot
    // creation fails due to missing native libraries at runtime.
    let res = std::panic::catch_unwind(|| {
        deno_runtime::snapshot::create_runtime_snapshot(
            snapshot_path,
            snapshot_options,
            vec![], // No custom extensions for the snapshot
        )
    });

    if let Err(e) = res {
        println!("cargo:warning=Runtime snapshot creation panicked: {e:?}");
        println!(
            "cargo:warning=Proceeding without generated snapshot. Consider generating it on a supported host or ensuring required native runtimes are available."
        );
    }
}
