//! Link support shared by the desktop app and CLI on native GNU Windows.

pub fn configure() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../../build-support/windows_gnu.rs");
    println!("cargo:rerun-if-env-changed=SystemRoot");
    if std::env::var("TARGET").as_deref() != Ok("x86_64-pc-windows-gnu") || std::env::var("HOST").as_deref() != Ok("x86_64-pc-windows-gnu") {
        return Ok(());
    }
    // Both apps depend on webbrowser, which links shlwapi. Rust does not ship
    // its import library, but LLD can import directly from the native system DLL.
    let root = std::env::var_os("SystemRoot").ok_or("missing SystemRoot for native GNU Windows linking")?;
    let dll = std::path::PathBuf::from(root).join("System32/shlwapi.dll");
    let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").ok_or("missing OUT_DIR")?);
    // Expose only this DLL: adding System32 itself would shadow Rust's bundled
    // runtime import libraries. This is a link input, not bundled with the apps.
    std::fs::copy(dll, out.join("shlwapi.dll"))?;
    println!("cargo:rustc-link-search=native={}", out.display());
    Ok(())
}
