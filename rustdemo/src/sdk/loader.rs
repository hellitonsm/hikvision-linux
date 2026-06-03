//! Dynamic loader for `libhcnetsdk` and `libPlayCtrl`.
//!
//! We use `libloading` so the same binary can find the SDK in the
//! `Linux64/lib/` directory shipped with the original Qt demo (see
//! `Linux64/lib/run.sh`), without forcing a system-wide install.

use libloading::Library;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use crate::sdk::hcnetsdk;

pub struct SdkHandles {
    pub hcnet: Library,
    pub play: Library,
}

impl SdkHandles {
    pub fn load() -> Result<Self, String> {
        let candidates = sdk_search_dirs();
        let hcnet = find_lib(&candidates, "hcnetsdk")?;
        let play = find_lib(&candidates, "PlayCtrl")?;
        Ok(SdkHandles { hcnet, play })
    }
}

fn sdk_search_dirs() -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = Vec::new();

    if let Ok(env_dir) = std::env::var("HCNETSDK_LIB_DIR") {
        dirs.push(PathBuf::from(env_dir));
    }

    // Relative to the workspace (newqtdemo/) — same layout the Qt demo uses.
    if let Ok(cwd) = std::env::current_dir() {
        for rel in &[
            "Linux64/lib",
            "Linux32/lib",
            "../Linux64/lib",
            "../../Linux64/lib",
            "win32/lib",
        ] {
            let p = cwd.join(rel);
            if p.exists() {
                dirs.push(p);
            }
        }
    }

    // System fallbacks.
    for sys in &["/usr/local/lib", "/usr/lib", "/usr/lib/x86_64-linux-gnu"] {
        let p = Path::new(sys);
        if p.exists() {
            dirs.push(p.to_path_buf());
        }
    }

    dirs
}

fn find_lib(dirs: &[PathBuf], stem: &str) -> Result<Library, String> {
    let candidates: Vec<String> = ["", "lib"].iter().map(|p| format!("{}{}", p, stem)).collect();

    for dir in dirs {
        for name in &candidates {
            for suffix in &[".so", ".so.1", ".dylib", ".dll"] {
                let path = dir.join(format!("{}.{}", name.trim_start_matches("lib"), suffix.trim_start_matches(".")));
                // Also try the actual raw name (e.g. libhcnetsdk.so)
                let raw = dir.join(format!("{}.{}{}", name, "", suffix.trim_start_matches(".")));
                for candidate in &[path.to_string_lossy().into_owned(), raw.to_string_lossy().into_owned()] {
                    if Path::new(candidate).exists() {
                        if let Ok(lib) = unsafe { Library::new(candidate) } {
                            log::info!("Loaded {} from {}", stem, candidate);
                            return Ok(lib);
                        }
                    }
                }
            }
        }
    }
    Err(format!(
        "Could not find lib{}.so. Set HCNETSDK_LIB_DIR or place the SDK under Linux64/lib/.",
        stem
    ))
}

/// Cached handles. The SDK is a process-singleton (NET_DVR_Init), so
/// loading it once is enough for the whole program.
static HANDLES: OnceLock<Result<SdkHandles, String>> = OnceLock::new();

pub fn handles() -> Result<&'static SdkHandles, String> {
    HANDLES
        .get_or_init(SdkHandles::load)
        .as_ref()
        .map_err(|e| e.clone())
}

/// True if the SDK could be loaded. Some commands (UI only) still work
/// even when the SDK is missing, so the main window can show a banner
/// instead of crashing.
pub fn sdk_available() -> bool {
    handles().is_ok()
}

pub use hcnetsdk::*;
