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

        let config = Config::builder()
            .set_default("server.workers", "1")
            .unwrap()
            .set_default("server.url", "127.0.0.1")
            .unwrap()
            .set_default("server.listen", "127.0.0.1")
            .unwrap()
            .set_default("server.port", "8080")
            .unwrap()
            .set_default("server.registration", "false")
            .unwrap()
            .set_default("files.read_only", "true")
            .unwrap()
            .set_default("tls.enabled", "false")
            .unwrap();

        let default: PathBuf = [dir, "default"].iter().collect();
        let default_path = default.to_str().unwrap_or("config/default.toml");
        let config = config.add_source(File::with_name(default_path));

        let local: PathBuf = [dir, "local"].iter().collect();
        let local_path = local.to_str().unwrap_or("config/local");
        let config = config.add_source(File::with_name(local_path));

        let config = config.add_source(Environment::with_prefix("TRIOX").separator("_"));

        let config = if let Ok(val) = env::var("PORT") {
            config.set_override("server.port", val).unwrap()
        } else {
            config
        };

        let config = config.build().unwrap();

        if config
            .get::<String>("server.secret")
            .expect("Please set a secret in configuration file")
            .len()
            < 32
        {
            panic!("Please set a secret that's at least 32 bytes long");
        }

        config.try_deserialize()
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
