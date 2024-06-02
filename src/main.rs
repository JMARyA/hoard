use hoard::config::GlobalConfig;
use hoard::{ensure_dir_exists, Module};

// todo : migrate to async code?
// todo : better log options

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

    let db = hoard::db::DatabaseBackend::new("data/download.db");
    let config: GlobalConfig =
        toml::from_str(&std::fs::read_to_string("config.toml").unwrap()).unwrap();
    ensure_dir_exists(&config.hoard.data_dir);

    let mut modules: Vec<Box<dyn Module>> = vec![];

    if let Some(yt_config) = config.youtube {
        modules.push(Box::new(hoard::youtube::YouTubeModule::new(
            yt_config,
            db.take_db(),
            config.hoard.data_dir.join("youtube"),
        )));
    }

    if let Some(sc_config) = config.soundcloud {
        modules.push(Box::new(hoard::soundcloud::SoundCloudModule::new(
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
        modules.push(Box::new(hoard::yt_dlp::YtDlpModule::new(
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
