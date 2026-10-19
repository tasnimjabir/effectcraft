#[path = "../../build-support/windows_gnu.rs"]
mod windows_gnu;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    windows_gnu::configure()
}
