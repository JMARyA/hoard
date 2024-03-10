use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::yt_dlp::YtDlpConfig;

/// General settings for hoard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoardConfig {
    // Top level data download directory
    pub data_dir: PathBuf,
}

/// Top level global config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    // Hoard Configuration
    pub hoard: HoardConfig,
    // Configuration for the YouTube Module
    pub youtube: Option<crate::youtube::YouTubeConfig>,
    // Configuration for the SoundCloud Module
    pub soundcloud: Option<crate::soundcloud::SoundCloudConfig>,
    // Custom instances of yt-dlp
    pub yt_dlp: Option<Vec<YtDlpConfig>>,
}
