use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    yt_dlp::{YtDlpConfig, YtDlpModule},
    Module,
};

/// Configuration for the `SoundCloud` Module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundCloudConfig {
    // Interval in minutes between checks
    pub interval: u64,
    /// Amount of items to query
    pub limit: Option<u64>,
    // Items to check
    pub artists: HashMap<String, String>,
    // Output Template for yt-dlp
    pub output_format: Option<String>,
    // Download comments
    pub write_comments: Option<bool>,
    // Download description
    pub write_description: Option<bool>,
    // Download cover
    pub write_cover: Option<bool>,
    // Download subtitles
    pub write_subs: Option<bool>,
    // Audio Format
    pub audio_format: Option<String>,
    // Embed thumbnail
    pub embed_thumbnail: Option<bool>,
    // Embed metadata
    pub embed_metadata: Option<bool>,
    // Embed chapters
    pub embed_chapters: Option<bool>,
    // Embed info.json
    pub embed_info_json: Option<bool>,
    // Split by chapter
    pub split_chapters: Option<bool>,
    // Format Selection
    pub format: Option<String>,
    // Cookie File
    pub cookie: Option<String>,
}

#[derive(Clone)]
pub struct SoundCloudModule {
    yt_dlp: YtDlpModule,
}

impl SoundCloudModule {
    pub fn new(config: SoundCloudConfig, db: crate::db::Database, root_dir: PathBuf) -> Self {
        Self {
            yt_dlp: YtDlpModule::new(
                YtDlpConfig {
                    name: Some("soundcloud".to_string()),
                    interval: config.interval,
                    limit: config.limit,
                    items: config.artists,
                    thumbnail_format: Some("jpg".to_string()),
                    output_format: config.output_format.clone(),
                    write_description: Some(config.write_description.unwrap_or(true)),
                    write_info_json: Some(false),
                    write_comments: config.write_comments,
                    write_thumbnail: Some(true),
                    write_subs: config.write_subs,
                    audio_format: config.audio_format,
                    embed_subs: Some(false),
                    embed_thumbnail: Some(config.embed_thumbnail.unwrap_or(true)),
                    embed_metadata: Some(config.embed_metadata.unwrap_or(true)),
                    embed_chapters: Some(config.embed_chapters.unwrap_or(true)),
                    embed_info_json: Some(config.embed_info_json.unwrap_or(true)),
                    split_chapters: config.split_chapters,
                    format: config.format,
                    cookie: config.cookie,
                    audio_only: Some(true),
                },
                db,
                root_dir,
            ),
        }
    }
}

impl Module for SoundCloudModule {
    fn name(&self) -> String {
        "SoundCloud".to_string()
    }

    fn run(&self) {
        self.yt_dlp.run();
    }
}
