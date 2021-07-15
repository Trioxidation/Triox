//!
//! Using the config crate, the configuration is first loaded from `config/default.toml` and
//! then overwritten by the values in `config/local.toml`.
//!
//! The values are then converted into an `AppConfig` struct that allows faster access
//! and also enforces the type system.

use serde::Deserialize;

use config::{Config, Environment, File};
/// Stores a database type (currently only MySQL).
#[derive(Debug, Clone, Deserialize)]
pub enum DbServerType {
    Mysql,
}

/// Configurations for the http server.
#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub host: String,
    pub ip: String,
    pub port: u32,
    pub workers: usize,
    pub registration: bool,
    pub secret: String,
    pub domain: String,
    pub rate_limit_period: Option<u64>,
    pub rate_limit_burst_size: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Files {
    pub read_only: bool,
}

/// Configurations for the database connector.
#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    // TODO changed to string, need to use the correct Pool type
    // while creating database connection
    pub db: String,
    pub user: String,
    pub password: String,
    pub host: String,
    pub name: String,
    pub pool: u32,
}

/// Configurations for tls.
#[derive(Debug, Clone, Deserialize)]
pub struct Tls {
    pub enabled: bool,
    pub certificate_path: Option<String>,
    pub key_path: Option<String>,
}

/// Collection of all partial configurations.
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: Server,
    pub files: Files,
    pub database: Database,
    pub tls: Tls,
}

fn set_default(config: &mut Config, field: &str, val: &str) {
    config
        .set_default(field, val)
        .expect("Couldn't set default workers");
}

impl AppConfig {
    /// pass in configuration dir
    pub fn new(dir: &str) -> Result<Self, config::ConfigError> {
        // configuration merge order:
        // 1. Set defaults
        // 2. default.toml
        // 3. local.toml
        // 4. From environment var(prefix = TRIOX")
        use std::env;
        use std::path::PathBuf;

        let mut config = Config::new();

        set_default(&mut config, "server.workers", "1");
        set_default(&mut config, "server.url", "127.0.0.1");
        set_default(&mut config, "server.listen", "127.0.0.1");
        set_default(&mut config, "server.port", "8080");
        set_default(&mut config, "server.registration", "false");
        set_default(&mut config, "files.read_only", "true");
        set_default(&mut config, "tls.enabled", "false");

        let default: PathBuf = [dir, "default"].iter().collect();
        let default_path = default.to_str().unwrap_or("config/default.toml");
        config
            .merge(File::with_name(default_path))
            .unwrap_or_else(|_| panic!("couldn't read config from: {:?}", default));

        let local: PathBuf = [dir, "local"].iter().collect();
        let local_path = local.to_str().unwrap_or("config/local");
        config
            .merge(File::with_name(local_path))
            .unwrap_or_else(|_| panic!("couldn't read config from: {:?}", local_path));

        config
            .merge(Environment::with_prefix("TRIOX").separator("_"))
            .expect("Problem reading env vars");

        if let Ok(val) = env::var("PORT") {
            config.set("server.port", val).unwrap();
        };

        config
            .get::<String>("server.secret")
            .expect("Please set a secret in configuration file");

        config.try_into()
    }
}

impl Server {
    /// Builds sever address from config parameters.
    pub fn listen_address(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

impl Database {
    /// Builds database url from config parameters.
    pub fn url(&self) -> String {
        if let Ok(val) = std::env::var("DATABASE_URL") {
            val
        } else {
            format!(
                "{}://{}:{}@{}/{}",
                self.db,
                //            match self.server_type {
                //                DbServerType::Mysql => "mysql",
                //            },
                self.user,
                self.password,
                self.host,
                self.name
            )
        }
    }
}
