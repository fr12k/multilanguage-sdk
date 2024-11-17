extern crate cbindgen;

use std::env;
use std::path::PathBuf;

fn main() {

    println!("cargo:rerun-if-changed=src/rust-core.udl");
    // uniffi::generate_scaffolding("src/rust-core.udl").unwrap();

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let config_path = PathBuf::from(&crate_dir).join("cbindgen.toml");

    println!("cargo:rerun-if-changed=cbindings.toml");

    cbindgen::Builder::new()
      .with_crate(crate_dir)
      .with_config(cbindgen::Config::from_file(config_path).expect("Failed to read cbindgen.toml"))
      .generate()
      .expect("Unable to generate bindings")
      .write_to_file("bindings.h");

    // Get the target platform
    let target = env::var("TARGET").unwrap();

    // Determine the binary name based on the target platform
    let bin_name = if target.contains("x86_64-unknown-linux-gnu") {
        "rust_core_linux_x86_64"
    } else if target.contains("x86_64-unknown-linux-musl") {
        "rust_core_linux_x86_64_musl"
    } else if target.contains("aarch64-unknown-linux-gnu") {
        "rust_core_linux_aarch64"
    } else if target.contains("aarch64-unknown-linux-musl") {
        "rust_core_linux_aarch64_musl"
    } else if target.contains("x86_64-apple-darwin") {
        "rust_core_macos_x86_64"
    } else if target.contains("aarch64-apple-darwin") {
        "rust_core_macos_aarch64"
    } else {
        "rust_core" // Default name for non-target platforms
    };

    // Set the binary name using environment variables
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-env=BIN_NAME={}", bin_name);
}

/// Find the location of the `target/` directory. Note that this may be
/// overridden by `cmake`, so we also need to check the `CARGO_TARGET_DIR`
/// variable.
fn target_dir() -> PathBuf {
    if let Ok(target) = env::var("CARGO_TARGET_DIR") {
        PathBuf::from(target)
    } else {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target")
    }
}
