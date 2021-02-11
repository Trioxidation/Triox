use dashmap::DashMap;

use crate::config::AppConfig;
use crate::database;

/// Storing the state of the application
/// Can be accessed using the AppData extractor.
#[derive(Clone)]
pub struct AppState {
    pub db_pool: database::DbPool,
    pub config: AppConfig,
    pub login_count: DashMap<u32, u8>,
}

impl AppState {
    pub fn new(config_path: &str) -> Self {
        // generate struct from config HashMap
        let config = AppConfig::new(&config_path).unwrap();

        // create database pool
        let db_pool = database::connect(&config.database.url())
            .expect("Failed to create database pool.");

        AppState {
            config,
            login_count: DashMap::new(),
            db_pool,
        }
    }
}
