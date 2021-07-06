use dashmap::DashMap;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::config::AppConfig;
//use crate::database;
use crate::SETTINGS;

/// Storing the state of the application
/// Can be accessed using the AppData extractor.
#[derive(Clone)]
pub struct AppState {
    //  pub db_pool: database::DbPool,
    pub config: AppConfig,
    pub login_count: DashMap<u32, u8>,
    pub creds: argon2_creds::Config,
    // sqlx
    pub db: PgPool,
}

impl AppState {
    pub async fn new(config_path: &str) -> Self {
        let creds = argon2_creds::ConfigBuilder::default()
            .username_case_mapped(true)
            .profanity(true)
            .blacklist(true)
            .password_policy(argon2_creds::PasswordPolicy::default())
            .build()
            .unwrap();

        let c = creds.clone();

        let init = std::thread::spawn(move || {
            log::info!("Initializing credential manager");
            c.init();
            log::info!("Initialized credential manager");
        });

        // generate struct from config HashMap
        let config = AppConfig::new(&config_path).unwrap();

        //     // create database pool
        //     let db_pool = database::connect(&config.database.url())
        //         .expect("Failed to create database pool.");

        let db = PgPoolOptions::new()
            .max_connections(SETTINGS.database.pool)
            .connect(&SETTINGS.database.url())
            .await
            .expect("Unable to form database pool");
        init.join().unwrap();
        AppState {
            config,
            login_count: DashMap::new(),
            //db_pool,
            creds,
            db,
        }
    }
}
