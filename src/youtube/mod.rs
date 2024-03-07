use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    path::PathBuf,
    process::Command,
};

use serde::{Deserialize, Serialize};

use crate::{ensure_dir_exists, Module};

/// Configuration for the YouTube Module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YouTubeConfig {
    // Interval in minutes between checks
    interval: u64,
    // Channels to check
    channels: HashMap<String, String>,
    // Format of the Thumbnail
    thumbnail_format: Option<String>,
    // Output Template for yt-dlp
    output_format: Option<String>,
}

impl YouTubeConfig {
    pub fn download_options(&self) -> DownloadOptions {
        DownloadOptions {
            thumbnail_format: self.thumbnail_format.clone(),
            output_format: self.output_format.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct YouTubeModule {
    config: YouTubeConfig,
    db: crate::db::Database,
    root_dir: PathBuf,
}

impl YouTubeModule {
    pub const fn new(config: YouTubeConfig, db: crate::db::Database, root_dir: PathBuf) -> Self {
        Self {
            config,
            db,
            root_dir,
        }
    }
}

impl Module for YouTubeModule {
    fn name(&self) -> String {
        "YouTube".to_string()
    }

    fn run(&self) {
        log::info!("Running YouTube Module");
        let download_options = self.config.download_options();
        for (channel, channel_url) in &self.config.channels {
            log::info!("Fetching {channel} videos");
            match Self::get_latest_channel_videos(channel_url) {
                Ok(latest_videos) => {
                    for (video_title, video_url) in latest_videos {
                        if self.db.check_for_url(&video_url).unwrap() {
                            log::trace!("Skipping {video_title} because it was already downloaded");
                        } else {
                            match Self::download_video(
                                &video_url,
                                &self.root_dir.join(channel),
                                &download_options,
                            ) {
                                Ok(()) => {
                                    // mark as downloaded
                                    self.db.insert_url(&video_url).unwrap();
                                    log::info!("Downloaded {video_title}");
                                }
                                Err(e) => {
                                    log::error!("Error downloading {video_title}; Reason: {e}");
                                    // todo : error handling
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    log::error!("Could not get videos from {channel}. Reason: {e}");
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(self.config.interval * 60));
    }
}

impl YouTubeModule {
    fn get_latest_channel_videos(channel: &str) -> Result<Vec<(String, String)>, String> {
        let output = Command::new("yt-dlp")
            .arg("--no-warnings")
            .arg("--flat-playlist")
            .arg("--skip-download")
            .arg("--print")
            .arg("title,webpage_url")
            .arg("--playlist-end")
            .arg("10")
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

        Ok(videos)
    }

    fn download_video(video_url: &str, cwd: &PathBuf, opt: &DownloadOptions) -> Result<(), String> {
        ensure_dir_exists(cwd);
        let output = Command::new("yt-dlp")
            .current_dir(cwd)
            .arg("--downloader")
            .arg("aria2c")
            .arg("--write-thumbnail")
            .arg("-o")
            .arg(opt.output_format.as_deref().unwrap_or("%(title)s.%(ext)s"))
            .arg("--embed-thumbnail")
            .arg("--embed-chapters")
            .arg("--embed-info-json")
            .arg("--convert-thumbnails")
            .arg(opt.thumbnail_format.as_deref().unwrap_or("jpg"))
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

pub struct DownloadOptions {
    thumbnail_format: Option<String>,
    output_format: Option<String>,
}
