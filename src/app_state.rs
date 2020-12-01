use config::Config;
use dashmap::DashMap;

use crate::app_conf;
use crate::database;

/// Storing the state of the application
/// Can be accessed using the AppData extractor.
#[derive(Clone)]
pub struct AppState {
    pub db_pool: database::DbPool,
    pub config: app_conf::AppConfig,
    pub login_count: DashMap<u32, u8>,
}

pub fn load_app_state(config_path: &str) -> AppState {
    use std::path::PathBuf;

    let default_config_path: PathBuf = [config_path, "default"].iter().collect();
    let local_config_path: PathBuf = [config_path, "local"].iter().collect();

    let mut config = Config::default();

    // open default config
    if config
        .merge(config::File::with_name(
            default_config_path.to_str().unwrap_or("config/default"),
        ))
        .is_err()
    {
        eprintln!("Could not open default config file!");
    }

    // open user config
    if config
        .merge(config::File::with_name(
            local_config_path.to_str().unwrap_or("config/local"),
        ))
        .is_err()
    {
        eprintln!("Could not open local config file!");
    }

    // generate struct from config HashMap
    let config = app_conf::load_config(&config);

    // create database pool
    let db_pool = database::connect(&config.database.url())
        .expect("Failed to create database pool.");

    AppState {
        config,
        login_count: DashMap::new(),
        db_pool,
    }
}
