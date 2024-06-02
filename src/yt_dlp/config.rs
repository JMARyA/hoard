use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Configuration for the `YouTube` Module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YtDlpConfig {
    /// Module Name
    pub name: Option<String>,
    /// Interval in minutes between checks
    pub interval: u64,
    /// Amount of items to query
    pub limit: Option<u64>,
    /// Items to check
    pub items: HashMap<String, toml::Value>,
    /// Format of the Thumbnail
    pub thumbnail_format: Option<String>,
    /// Output Template for yt-dlp
    pub output_format: Option<String>,
    /// Download description
    pub write_description: Option<bool>,
    /// Download info.json
    pub write_info_json: Option<bool>,
    /// Download comments
    pub write_comments: Option<bool>,
    /// Download thumbnail
    pub write_thumbnail: Option<bool>,
    /// Download subtitles
    pub write_subs: Option<bool>,
    /// Extract audio
    pub audio_only: Option<bool>,
    /// Audio Format
    pub audio_format: Option<String>,
    /// Embed subtitles
    pub embed_subs: Option<bool>,
    /// Embed thumbnail
    pub embed_thumbnail: Option<bool>,
    /// Embed metadata
    pub embed_metadata: Option<bool>,
    /// Embed chapters
    pub embed_chapters: Option<bool>,
    /// Embed info.json
    pub embed_info_json: Option<bool>,
    /// Split by chapter
    pub split_chapters: Option<bool>,
    /// Format Selection
    pub format: Option<String>,
    /// Cookie File
    pub cookie: Option<String>,
    /// Webhooks for notifications
    pub webhooks: Option<Vec<String>>,
}
