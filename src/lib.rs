use std::path::PathBuf;

pub mod config;
pub mod db;
pub mod soundcloud;
pub mod youtube;
pub mod yt_dlp;

pub fn ensure_dir_exists(dir_path: &PathBuf) {
    let path = std::path::Path::new(dir_path);
    if !path.exists() {
        std::fs::create_dir_all(path).unwrap();
    }
}

/// Generic module implementation
///
/// Each module gets it's own thread to work for itself.
pub trait Module: Send {
    /// friendly name for module
    fn name(&self) -> String;
    /// module main loop
    fn run(&self);
}
