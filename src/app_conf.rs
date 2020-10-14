//!
//! Using the config crate, the configuration is first loaded from `config/default.toml` and
//! then overwritten by the values in `config/local.toml`.
//!
//! The values are then converted into an `AppConfig` struct that allows faster access
//! and also enforced the type system.

use config::Config;

/// Stores a database type (currently only MySQL).
#[derive(Debug, Clone, Copy)]
pub enum DbServerType {
    Mysql,
}

/// Configurations for the http server.
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub url: Box<str>,
    pub listen: Box<str>,
    pub port: u32,
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
}

/// Configurations for JWT authentification.
#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub secret: Box<str>,
}

/// Collection of all partial configurations.
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
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
            println!(
                "CONFIG: Couldn't find field '{}', falling back to default value '{}'",
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
                println!(
                "CONFIG: Couldn't find field '{}', falling back to default value '{}'",
                    key, default_val
                );
                default_val.to_string()
            })
            .into_boxed_str()
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
        },
        jwt: JwtConfig {
            secret: conf.get_str("jwt.secret", "secret"),
        },
    }
}

impl ServerConfig {
    /// Builds sever address from config parameters.
    pub fn listen_address(&self) -> String {
        let mut url: String = String::new();

        url += &self.listen;
        url += ":";
        url += &self.port.to_string();

        url
    }
}

impl DatabaseConfig {
    /// Builds database url from config parameters.
    pub fn url(&self) -> String {
        let mut url: String = String::new();

        url += match self.server_type {
            DbServerType::Mysql => "mysql",
        };

        url += "://";
        url += &self.user;
        url += ":";
        url += &self.password;
        url += "@";
        url += &self.address;
        url += "/";
        url += &self.name;

        url
    }
}
