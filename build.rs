fn main() {
    //version
    //x86_64-w64-mingw32-windres version.rc -O coff -o version.res
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        // Tell Cargo to rerun build script if app.res changes
        println!("cargo:rerun-if-changed=version.res");
        // Link the .res file
        println!("cargo:rustc-link-arg=version.res");
    }
}
