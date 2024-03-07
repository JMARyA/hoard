use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoardConfig {
    // Top level data download directory
    pub data_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    // Hoard Configuration
    pub hoard: HoardConfig,
    // Configuration for the YouTube Module
    pub youtube: Option<crate::youtube::YouTubeConfig>,
}
