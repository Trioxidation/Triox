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
#[derive(Debug, Clone, Copy)]
pub struct ServerConfig<'a> {
    pub url: &'a str,
    pub listen: &'a str,
    pub port: u32,
}

/// Configurations for the database connector.
#[derive(Debug, Clone, Copy)]
pub struct DatabaseConfig<'a> {
    pub server_type: DbServerType,
    pub user: &'a str,
    pub password: &'a str,
    pub address: &'a str,
    pub name: &'a str,
}

/// Configurations for SSL.
#[derive(Debug, Clone, Copy)]
pub struct SslConfig {
    pub enabled: bool,
}

/// Configurations for JWT authentification.
#[derive(Debug, Clone, Copy)]
pub struct JwtConfig<'a> {
    pub secret: &'a str,
}

/// Collection of all partial configurations.
#[derive(Debug, Clone, Copy)]
pub struct AppConfig<'a> {
    pub server: ServerConfig<'a>,
    pub database: DatabaseConfig<'a>,
    pub ssl: SslConfig,
    pub jwt: JwtConfig<'a>,
}

/// Converts config HashMap into `AppConfig` struct.
pub fn load_config(config: &Config) -> AppConfig<'static> {
    AppConfig {
        server: ServerConfig {
            url: config.get("server.url").unwrap_or("127.0.0.1"),
            listen: config.get("server.listen").unwrap_or("127.0.0.1"),
            port: config.get("server.port").unwrap_or(443),
        },
        database: DatabaseConfig {
            server_type: DbServerType::Mysql,
            user: config.get("database.user").unwrap_or("triox"),
            password: config.get("database.password").unwrap_or("triox"),
            address: config.get("database.address").unwrap_or("localhost"),
            name: config.get("database.name").unwrap_or("triox"),
        },
        ssl: SslConfig {
            enabled: config.get("ssl.enabled").unwrap_or(true),
        },
        jwt: JwtConfig {
            secret: config.get("jwt.secret").unwrap_or("secret"),
        },
    }
}

impl<'a> ServerConfig<'a> {
    /// Builds sever address from config parameters.
    pub fn listen_address(&self) -> String {
        let mut url: String = String::new();

        url += self.listen;
        url += ":";
        url += &self.port.to_string();

        url
    }
}

impl<'a> DatabaseConfig<'a> {
    /// Builds database url from config parameters.
    pub fn url(&self) -> String {
        let mut url: String = String::new();

        url += match self.server_type {
            DbServerType::Mysql => "mysql",
        };

        url += "://";
        url += self.user;
        url += ":";
        url += self.password;
        url += "@";
        url += self.address;
        url += "/";
        url += self.name;

        url
    }
}
