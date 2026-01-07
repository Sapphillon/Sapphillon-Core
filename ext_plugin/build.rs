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
    {
        create_empty_snapshot();
        println!(
            "cargo:warning=Skipping snapshot generation on Windows to avoid DLL linking issues."
        );
    }
}

#[cfg(not(windows))]
fn build_snapshot() {
    use std::fs::File;
    use std::io::Write;

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
            snapshot_path.clone(),
            snapshot_options,
            vec![], // No custom extensions for the snapshot
        )
    });

    if let Err(e) = res {
        println!("cargo:warning=Runtime snapshot creation panicked: {e:?}");
        println!(
            "cargo:warning=Proceeding without generated snapshot. Consider generating it on a supported host or ensuring required native runtimes are available."
        );

        // Create an empty snapshot file so that include_bytes! doesn't fail
        if let Ok(mut file) = File::create(&snapshot_path) {
            let _ = file.write_all(&[]);
            println!(
                "cargo:warning=Created empty snapshot file at {}",
                snapshot_path.display()
            );
        }
    }
}

#[cfg(windows)]
fn create_empty_snapshot() {
    use std::fs::File;
    use std::io::Write;

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let snapshot_path = out_dir.join("EXT_PLUGIN_SNAPSHOT.bin");

    // Create an empty snapshot file for Windows builds
    if let Ok(mut file) = File::create(&snapshot_path) {
        let _ = file.write_all(&[]);
        println!(
            "cargo:warning=Created empty snapshot file for Windows at {}",
            snapshot_path.display()
        );
    }
}
