use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::SETTINGS;

/// Storing the state of the application
/// Can be accessed using the AppData extractor.
#[derive(Clone)]
pub struct AppState {
    pub creds: argon2_creds::Config,
    // sqlx
    pub db: PgPool,
}

impl AppState {
    pub async fn new() -> Self {
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

        let db = PgPoolOptions::new()
            .max_connections(SETTINGS.database.pool)
            .connect(&SETTINGS.database.url())
            .await
            .expect("Unable to form database pool");
        init.join().unwrap();
        AppState { creds, db }
    }
}
