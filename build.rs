use std::path::Path;
use std::process::Command;

fn main() {
    //icon, old manual way
    //x86_64-w64-mingw32-windres app_icon.rc -O coff -o app_icon.res -> compile .res file
    compile_embedded_resource("app_icon.rc", "app_icon.res");
    println!("cargo:rerun-if-changed=app_icon.rc");
    println!("cargo:rustc-link-arg=app_icon.res");

    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        // Compile the version.rc file into version.res
        compile_embedded_resource("version.rc", "version.res");
        // Tell Cargo to rerun the build script if version.rc changes
        println!("cargo:rerun-if-changed=version.rc");
        // Link the .res file
        println!("cargo:rustc-link-arg=version.res");
    }

    println!(r"cargo:rustc-cdylib-link-arg=-Wl,exports.def");
    //in our case, exports.def contain thymbcache.dll proxy
    /*
    Windows Registry Editor Version 5.00

    [HKEY_CURRENT_USER\Software\Classes\CLSID\{2155fee3-2419-4373-b102-6843707eb41f}\InprocServer32]
    @="C:\\Users\\bufos\\Desktop\\FOURNOS\\ntapcsheel_dll_rs.dll"
    "ThreadingModel"="Both"


    Original dll location: C:\\Windows\\System32\\thumbcache.dll
    reg query "HKCU\Software\Classes\CLSID\{2155fee3-2419-4373-b102-6843707eb41f}\InprocServer32"
    */
}

/// Compiles a Windows resource file (.rc) into a .res file using windres
fn compile_embedded_resource(input: &str, output: &str) {
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
