use std::path::PathBuf;

mod config;
mod db;
mod soundcloud;
mod youtube;
mod yt_dlp;

use config::GlobalConfig;

use crate::yt_dlp::YtDlpModule;

// todo : migrate to async code?
// todo : better log options

pub fn ensure_dir_exists(dir_path: &PathBuf) {
    let path = std::path::Path::new(dir_path);
    if !path.exists() {
        std::fs::create_dir_all(path).unwrap();
    }
}

trait Module: Send {
    fn name(&self) -> String;
    fn run(&self);
}

fn main() {
    #[cfg(debug_assertions)]
    {
        std::env::set_var("RUST_LOG", "trace");
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    #[cfg(not(debug_assertions))]
    {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", "warn");
        }
    }
    env_logger::init();

    log::info!("Starting hoard");

    let db = db::DatabaseBackend::new("data/download.db");
    let config: GlobalConfig =
        toml::from_str(&std::fs::read_to_string("config.toml").unwrap()).unwrap();
    ensure_dir_exists(&config.hoard.data_dir);

    let mut modules: Vec<Box<dyn Module>> = vec![Box::new(youtube::YouTubeModule::new(
        config.youtube.unwrap(),
        db.take_db(),
        config.hoard.data_dir.join("youtube"),
    ))];

    if let Some(sc_config) = config.soundcloud {
        modules.push(Box::new(soundcloud::SoundCloudModule::new(
            sc_config,
            db.take_db(),
            config.hoard.data_dir.join("soundcloud"),
        )));
    }

    for yt_dlp_mod in config.yt_dlp.unwrap_or_default() {
        let mod_name = yt_dlp_mod
            .name
            .clone()
            .unwrap_or_else(|| "yt_dlp".to_string());
        modules.push(Box::new(YtDlpModule::new(
            yt_dlp_mod,
            db.take_db(),
            config.hoard.data_dir.join(mod_name),
        )));
    }

    let _db_thread = std::thread::spawn(move || {
        db.run();
    });

    let threads: Vec<_> = modules
        .into_iter()
        .map(|x| {
            std::thread::spawn(move || {
                x.run();
            })
        })
        .collect();

    for t in threads {
        // todo : fix dying threads
        t.join().unwrap();
    }
}
