use std::path::PathBuf;

mod config;
mod db;
mod youtube;

use config::GlobalConfig;

// todo : migrate to async code?

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

    let db = db::Database::new("download.db");
    let config: GlobalConfig =
        toml::from_str(&std::fs::read_to_string("config.toml").unwrap()).unwrap();

    ensure_dir_exists(&config.hoard.data_dir);

    let modules: Vec<Box<dyn Module>> = vec![Box::new(youtube::YouTubeModule::new(
        config.youtube.unwrap(),
        db,
        config.hoard.data_dir.join("youtube"),
    ))];

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
