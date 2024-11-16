use std::process::Command;
use std::path::Path;

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        // Compile the version.rc file into version.res
        compile_version_resource("version.rc", "version.res");

        // Tell Cargo to rerun the build script if version.rc changes
        println!("cargo:rerun-if-changed=version.rc");

        // Link the .res file
        println!("cargo:rustc-link-arg=version.res");
    }
}

/// Compiles a Windows resource file (.rc) into a .res file using windres
fn compile_version_resource(input: &str, output: &str) {
    // Ensure the input file exists
    if !Path::new(input).exists() {
        panic!("The file {} does not exist!", input);
    }

    // Execute the windres command
    let output_status = Command::new("x86_64-w64-mingw32-windres")
        .args(&[input, "-O", "coff", "-o", output])
        .output()
        .expect("Failed to execute windres");

    // Check if the command succeeded
    if !output_status.status.success() {
        panic!(
            "windres failed with status {}. Output: {}",
            output_status.status,
            String::from_utf8_lossy(&output_status.stderr)
        );
    }

    println!("Successfully compiled {} to {}", input, output);
}
