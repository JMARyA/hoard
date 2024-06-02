use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{yt_dlp::config::YtDlpConfig, yt_dlp::YtDlpModule, Module};

/// Configuration for the `YouTube` Module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YouTubeConfig {
    /// Interval in minutes between checks
    interval: u64,
    /// Amount of videos to query
    limit: Option<u64>,
    /// Channels to check
    channels: HashMap<String, toml::Value>,
    /// Format of the Thumbnail
    thumbnail_format: Option<String>,
    /// Output Template for yt-dlp
    output_format: Option<String>,
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
    /// Embed subtitles
    pub embed_subs: Option<bool>,
    /// Embed thumbnail
    pub embed_thumbnail: Option<bool>,
    /// Embed metadata
    pub embed_metadata: Option<bool>,
    /// Embed chapters
    embed_chapters: Option<bool>,
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

#[derive(Clone)]
pub struct YouTubeModule {
    yt_dlp: YtDlpModule,
}

impl YouTubeModule {
    pub fn new(config: YouTubeConfig, db: crate::db::Database, root_dir: PathBuf) -> Self {
        Self {
            yt_dlp: YtDlpModule::new(
                YtDlpConfig {
                    name: Some("youtube".to_string()),
                    interval: config.interval,
                    limit: config.limit,
                    items: config.channels,
                    thumbnail_format: config.thumbnail_format,
                    output_format: config.output_format.clone(),
                    write_description: Some(config.write_description.unwrap_or(true)),
                    write_info_json: config.write_info_json,
                    write_comments: config.write_comments,
                    write_thumbnail: Some(config.write_thumbnail.unwrap_or(true)),
                    write_subs: config.write_subs,
                    audio_format: None,
                    embed_subs: config.embed_subs,
                    embed_thumbnail: config.embed_thumbnail,
                    embed_metadata: config.embed_metadata,
                    embed_chapters: config.embed_chapters,
                    embed_info_json: config.embed_info_json,
                    split_chapters: config.split_chapters,
                    format: config.format,
                    cookie: config.cookie,
                    audio_only: Some(false),
                    webhooks: config.webhooks,
                },
                db,
                root_dir,
            ),
        }
    }
}

impl Module for YouTubeModule {
    fn name(&self) -> String {
        "YouTube".to_string()
    }

    fn run(&self) {
        self.yt_dlp.run();
    }
}
