use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    path::PathBuf,
    process::Command,
};

use serde::{Deserialize, Serialize};

use crate::{ensure_dir_exists, Module};

/// Configuration for the `YouTube` Module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YtDlpConfig {
    // Module Name
    pub name: Option<String>,
    // Interval in minutes between checks
    pub interval: u64,
    /// Amount of items to query
    pub limit: Option<u64>,
    // Items to check
    pub items: HashMap<String, String>,
    // Format of the Thumbnail
    pub thumbnail_format: Option<String>,
    // Output Template for yt-dlp
    pub output_format: Option<String>,
    // Download description
    pub write_description: Option<bool>,
    // Download info.json
    pub write_info_json: Option<bool>,
    // Download comments
    pub write_comments: Option<bool>,
    // Download thumbnail
    pub write_thumbnail: Option<bool>,
    // Download subtitles
    pub write_subs: Option<bool>,
    // Extract audio
    pub audio_only: Option<bool>,
    // Audio Format
    pub audio_format: Option<String>,
    // Embed subtitles
    pub embed_subs: Option<bool>,
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
pub struct YtDlpModule {
    config: YtDlpConfig,
    db: crate::db::Database,
    root_dir: PathBuf,
}

impl YtDlpModule {
    pub const fn new(config: YtDlpConfig, db: crate::db::Database, root_dir: PathBuf) -> Self {
        Self {
            config,
            db,
            root_dir,
        }
    }
}

impl Module for YtDlpModule {
    fn name(&self) -> String {
        self.config
            .name
            .clone()
            .unwrap_or_else(|| "yt-dlp".to_string())
    }

    fn run(&self) {
        loop {
            log::info!("Running {} Module", self.name());
            log::info!("Checking {} items", self.config.items.len());
            for (item, item_url) in &self.config.items {
                log::info!("Fetching \"{item}\" videos");
                match Self::get_latest_entries(item_url, self.config.limit.unwrap_or(10)) {
                    Ok(latest_videos) => {
                        for (video_title, video_url) in latest_videos {
                            if self.db.check_for_url(&video_url) {
                                log::trace!(
                                    "Skipping \"{video_title}\" because it was already downloaded"
                                );
                            } else {
                                match self.download(&video_url, &self.root_dir.join(item)) {
                                    Ok(()) => {
                                        // mark as downloaded
                                        self.db.insert_url(&video_url);
                                        log::info!("Downloaded \"{video_title}\"");
                                    }
                                    Err(e) => {
                                        log::error!(
                                            "Error downloading \"{video_title}\"; Reason: {e}"
                                        );
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Could not get videos from \"{item}\". Reason: {e}");
                    }
                }
            }
            log::info!(
                "{} complete. Sleeping for {} minutes...",
                self.name(),
                self.config.interval
            );
            std::thread::sleep(std::time::Duration::from_secs(self.config.interval * 60));
        }
    }
}

impl YtDlpModule {
    fn get_latest_entries(channel: &str, limit: u64) -> Result<Vec<(String, String)>, String> {
        let output = Command::new("yt-dlp")
            .arg("--no-warnings")
            .arg("--flat-playlist")
            .arg("--skip-download")
            .arg("--print")
            .arg("title,webpage_url")
            .arg("--playlist-end")
            .arg(limit.to_string())
            .arg(channel)
            .output()
            .expect("Failed to execute yt-dlp");

        if !output.status.success() {
            return Err(String::from_utf8(output.stderr).unwrap());
        }

        let reader = BufReader::new(&output.stdout[..]);
        let mut videos = Vec::new();
        let mut lines = reader.lines();
        while let (Some(title), Some(url)) = (lines.next(), lines.next()) {
            if let (Ok(title), Ok(url)) = (title, url) {
                videos.push((title, url));
            }
        }

        Ok(videos.into_iter().take(limit as usize).collect())
    }

    fn download(&self, video_url: &str, cwd: &PathBuf) -> Result<(), String> {
        ensure_dir_exists(cwd);
        let mut command = Command::new("yt-dlp");
        let mut command = command.current_dir(cwd).arg("--downloader").arg("aria2c");

        if self.config.write_thumbnail.unwrap_or(true) {
            command = command.arg("--write-thumbnail");
        }
        if self.config.write_description.unwrap_or(false) {
            command = command.arg("--write-description");
        }
        if self.config.write_info_json.unwrap_or(false) {
            command = command.arg("--write-info-json");
        }
        if self.config.write_comments.unwrap_or(false) {
            command = command.arg("--write-comments");
        }
        if self.config.write_subs.unwrap_or(false) {
            command = command.arg("--write-subs");
        }
        if self.config.audio_only.unwrap_or(false) {
            command = command.arg("--extract-audio");
        }
        if let Some(audio_format) = &self.config.audio_format {
            command = command.arg("--audio-format").arg(audio_format);
        }

        if self.config.embed_chapters.unwrap_or(true) {
            command = command.arg("--embed-chapters");
        }
        if self.config.embed_info_json.unwrap_or(true) {
            command = command.arg("--embed-info-json");
        }
        if self.config.embed_metadata.unwrap_or(true) {
            command = command.arg("--embed-metadata");
        }
        if self.config.embed_subs.unwrap_or(false) {
            command = command.arg("--embed-subs");
        }
        if self.config.embed_thumbnail.unwrap_or(true) {
            command = command.arg("--embed-thumbnail");
        }

        if self.config.split_chapters.unwrap_or(false) {
            command = command.arg("--split-chapters");
        }

        if let Some(format) = &self.config.format {
            command = command.arg("--format").arg(format);
        }
        if let Some(cookie) = &self.config.cookie {
            command = command.arg("--cookies").arg(cookie);
        }

        let output = command
            .arg("--convert-thumbnails")
            .arg(self.config.thumbnail_format.as_deref().unwrap_or("jpg"))
            .arg("-o")
            .arg(
                self.config
                    .output_format
                    .as_deref()
                    .unwrap_or("%(title)s.%(ext)s"),
            )
            .arg(video_url)
            .output()
            .map_err(|_| "yt-dlp command failed".to_string())?;

        if !output.status.success() {
            let error_message = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(error_message);
        }

        Ok(())
    }
}
