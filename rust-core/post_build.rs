use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() ->  Result<(), Box<dyn std::error::Error>> {
    // Print the current working directory
    let cwd = env::current_dir()?;
    println!("Current working directory: {}", cwd.display());

    // Determine if the current working directory is `target`
    let target_dir = if cwd.ends_with("target") {
        PathBuf::from(".") // We're already in the `target` directory
    } else {
        cwd.join("target") // Adjust to the `target` directory relative to the current directory
    };

    let output_dir = if cwd.ends_with("target") {
        PathBuf::from("../go/pkg/api") // We're already in the `target` directory
    } else {
        cwd.join("go/pkg/api") // Adjust to the `target` directory relative to the current directory
    };

    // Ensure the output directory exists
    fs::create_dir_all(output_dir.clone())?;

    // Iterate over directories in `target`
    for entry in fs::read_dir(target_dir.clone())? {
        let entry = entry?;
        let path = entry.path();

        // Skip if not a directory or is `debug` or `release`
        if !path.is_dir() {
            continue;
        }
        let dir_name = path.file_name().unwrap_or_default().to_string_lossy();
        if dir_name == "debug" || dir_name == "release" {
            continue;
        }

        // Construct the path to `target/[dir_name]/release/librust_core.a`
        let release_file = target_dir.join(&dir_name.as_ref()).join("release").join("librust_core.a");

        // Debug output for the constructed path
        println!("Checking for file: {}", release_file.display());

        if release_file.exists() {
            // Create the new file name and copy it to `go/pkg/api`
            let new_file_name = format!("librust_core_{}.a", dir_name);
            let new_file_path = output_dir.join(new_file_name);

            // Debug output for the copy operation
            println!("Copying to: {}", new_file_path.display());

            fs::copy(&release_file, &new_file_path)?;
            println!(
                "Copied and renamed: {} -> {}",
                release_file.display(),
                new_file_path.display()
            );
        } else {
            // Debug output for file not found
            println!("File not found: {}", release_file.display());
        }
    }

    Ok(())
}
