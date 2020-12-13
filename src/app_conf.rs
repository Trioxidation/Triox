//!
//! Using the config crate, the configuration is first loaded from `config/default.toml` and
//! then overwritten by the values in `config/local.toml`.
//!
//! The values are then converted into an `AppConfig` struct that allows faster access
//! and also enforces the type system.

use config::Config;
use log::{info, warn};
use rand::distributions::Standard;
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::Read;

/// Stores a database type (currently only MySQL).
#[derive(Debug, Clone)]
pub enum DbServerType {
    Mysql,
}

/// Configurations for the http server.
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub url: Box<str>,
    pub listen: Box<str>,
    pub port: u32,
    pub workers: usize,
}

#[derive(Debug, Clone)]
pub struct UserConfig {
    pub disable_sign_up: bool,
}

#[derive(Debug, Clone)]
pub struct FilesConfig {
    pub read_only: bool,
}

/// Configurations for the database connector.
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub server_type: DbServerType,
    pub user: Box<str>,
    pub password: Box<str>,
    pub address: Box<str>,
    pub name: Box<str>,
}

/// Configurations for SSL.
#[derive(Debug, Clone)]
pub struct SslConfig {
    pub enabled: bool,
    pub certificate_path: Box<str>,
    pub key_path: Box<str>,
}

/// Configurations for JWT authentication.
#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub secret: Box<[u8]>,
}

/// Collection of all partial configurations.
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub user: UserConfig,
    pub files: FilesConfig,
    pub database: DatabaseConfig,
    pub ssl: SslConfig,
    pub jwt: JwtConfig,
}

/// Helper struct to allow easier extraction of config values
struct ConfWrapper<'a>(&'a Config);

impl<'a> ConfWrapper<'a> {
    /// Get `Box<T>` from the config or use default value.
    /// The user is notified if a default value is used.
    fn get<'b, T: serde::Deserialize<'b> + std::fmt::Display>(
        &self,
        key: &str,
        default_val: T,
    ) -> T {
        self.0.get::<T>(key).unwrap_or_else(|_| {
            warn!(
                "CONFIG: Couldn't find entry '{}', falling back to default value '{}'",
                key, default_val
            );
            default_val
        })
    }

    /// Get `Box<str>` from the config or use default value.
    /// The user is notified if a default value is used.
    fn get_str(&self, key: &str, default_val: &str) -> Box<str> {
        self.0
            .get_str(key)
            .unwrap_or_else(|_| {
                warn!(
                    "CONFIG: Couldn't find entry '{}', falling back to default value '{}'",
                    key, default_val
                );
                default_val.to_string()
            })
            .into_boxed_str()
    }

    /// Get `Box<u8>` from the configured path or use default value.
    /// The user is notified if a default value is used.
    fn get_bytes_from_path(&self, key: &str, default_key_length: usize) -> Box<[u8]> {
        let res_path = self.0.get_str(key);

        // Try to open file and read bytes - otherwise generate random bytes
        if let Ok(str_path) = res_path {
            if let Ok(mut file) = File::open(&str_path) {
                let mut bytes: Vec<u8> = Vec::new();
                if file.read_to_end(&mut bytes).is_ok() {
                    // Returns bytes on success
                    return bytes.into_boxed_slice();
                } else {
                    warn!(
                        "CONFIG: Read bytes from file at path '{}' specified in '{}', generating random secret instead",
                        &str_path, key
                    );
                }
            } else {
                warn!(
                    "CONFIG: Couldn't open file at path '{}' specified in '{}', generating random secret instead",
                    &str_path, key
                );
            }
        } else {
            info!(
                "CONFIG: Entry '{}' is empty, generating random secret instead",
                key
            );
        }

        // generate random bytes with specified length
        thread_rng()
            .sample_iter(Standard)
            .take(default_key_length)
            .collect::<Vec<u8>>()
            .into_boxed_slice()
    }
}

/// Converts config HashMap into `AppConfig` struct.
pub fn load_config(config: &Config) -> AppConfig {
    // wrap config into helper struct
    let conf = ConfWrapper(config);

    AppConfig {
        server: ServerConfig {
            url: conf.get_str("server.url", "127.0.0.1"),
            listen: conf.get_str("server.listen", "127.0.0.1"),
            port: conf.get("server.port", 8080),
            workers: conf.get("server.workers", 1),
        },
        user: UserConfig {
            disable_sign_up: conf.get("user.disable_sign_up", true),
        },
        files: FilesConfig {
            read_only: conf.get("files.read_only", true),
        },
        database: DatabaseConfig {
            server_type: DbServerType::Mysql,
            user: conf.get_str("database.user", "triox"),
            password: conf.get_str("database.password", "triox"),
            address: conf.get_str("database.address", "localhost"),
            name: conf.get_str("database.name", "triox"),
        },
        ssl: SslConfig {
            enabled: conf.get("ssl.enabled", true),
            certificate_path: conf.get_str("ssl.certificate", "ssl/cert.pem"),
            key_path: conf.get_str("ssl.key", "ssl/key.pem"),
        },
        jwt: JwtConfig {
            secret: conf.get_bytes_from_path("jwt.secret", 2048),
        },
    }
}

impl ServerConfig {
    /// Builds sever address from config parameters.
    pub fn listen_address(&self) -> String {
        format!("{}:{}", self.listen, self.port)
    }
}

impl DatabaseConfig {
    /// Builds database url from config parameters.
    pub fn url(&self) -> String {
        format!(
            "{}://{}:{}@{}/{}",
            match self.server_type {
                DbServerType::Mysql => "mysql",
            },
            self.user,
            self.password,
            self.address,
            self.name
        )
    }
}
