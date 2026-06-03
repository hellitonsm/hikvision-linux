use std::path::PathBuf;
use std::env;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir.parent().unwrap();

    let lib_search_dirs = [
        manifest_dir.join("lib"),
        workspace_root.join("Linux64").join("lib"),
        workspace_root.join("Linux32").join("lib"),
        workspace_root.join("win32").join("lib"),
    ];
    for dir in &lib_search_dirs {
        if dir.exists() {
            println!("cargo:rustc-link-search=native={}", dir.display());
        }
    }
    println!("cargo:rustc-link-lib=dylib=hcnetsdk");
    println!("cargo:rustc-link-lib=dylib=PlayCtrl");
    println!("cargo:rustc-link-lib=dylib=AudioRender");
    println!("cargo:rustc-link-lib=dylib=SuperRender");

    slint_build::compile("ui/main.slint").expect("slint build failed");
}
