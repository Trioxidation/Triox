// this is required to fix an issue with Rust 1.46...
#![type_length_limit = "100000000"]

//! # Triox - a cloud server for the next generation
//!
//!☘️ **Open Source** - We strongly believe in collaboration and transparency.
//!
//!⚡ **Speed** - Get the most out of your hardware! Triox runs fast, even on less powerful setups.
//!
//!🔒 **Security** - We're using the Argon2 algorithm to protect your passwords.
//!
//!⛓️ **Reliability** - Built on top of the strong guarantees of the
//![Rust programming language](https://rust-lang.org).
//!
//!🛫 **Easy Setup** - Triox comes with batteries included and is easy to configure.
//!
//!🔬 **Modern Technologies** - No cookies but authentication with [JWT](https://jwt.io)
//! and a front-end based on [WebAssembly](https://webassembly.org).

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate log;

/// The API is split into apps that take care of certain features.
mod apps;

/// Global configuration struct for Triox.
mod app_conf;

/// API for authentification. Including sign in, sign out and user information.
mod auth;

/// Database structures and helper functions for loading, setting and updating the database.
mod database;

/// Helper functions for hashing and comparing passwords.
mod hash;

/// Structures and Extrators for JWT authentification.
mod jwt;

/// Tests.
mod tests;

use actix_web::{middleware, web, App, HttpRequest, HttpServer};
use actix_files::NamedFile;
use env_logger::Env;

use config::Config;
use dashmap::DashMap;

use diesel::r2d2::{self, ConnectionManager};
use diesel::MysqlConnection;
pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

/// Index page
async fn index(_req: HttpRequest) -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?.set_content_type(mime::TEXT_HTML_UTF_8))
}

/// Storing the state of the application
/// Can be accessed using the AppData extractor.
#[derive(Clone)]
pub struct AppState {
    db_pool: DbPool,
    config: app_conf::AppConfig,
    login_count: DashMap<u32, u8>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // setup logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Load configurations
    let mut config = Config::default();

    // open default config
    if config
        .merge(config::File::with_name("config/default"))
        .is_err()
    {
        eprintln!("Could not open default config file!");
    }

    // open user config
    if config
        .merge(config::File::with_name("config/local"))
        .is_err()
    {
        eprintln!("Could not open local config file!");
    }

    // gerate struct from config HashMap
    let config = app_conf::load_config(&config);

    // create database pool
    let db_pool = database::connect(&config.database.url())
        .expect("Failed to create database pool.");

    // Clone config before it is moved into the closure
    let server_conf = config.server.clone();
    let ssl_conf = config.ssl.clone();

    // setup HTTP server
    let mut server = HttpServer::new(move || {
        App::new()
            // setup application state extractor
            .data(AppState {
                config: config.clone(),
                login_count: DashMap::new(),
                db_pool: db_pool.clone(),
            })
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
            .route("/user_info", web::get().to(auth::user_info))
            .route("/sign_up", web::get().to(auth::sign_up_page))
            .route("/sign_up", web::post().to(auth::sign_up))
            .route("/sign_in", web::get().to(auth::sign_in_page))
            .route("/sign_in", web::post().to(auth::sign_in))
            .service(apps::files::get::get)
            .service(apps::files::list::list)
            .service(apps::files::list::list_root)
            .service(apps::files::up::up)
            .service(apps::files::up::up_root)

            // Serve static files from ./static/ to /static/
            .service(actix_files::Files::new("/static", "static"))
    });

    let listen_address = server_conf.listen_address();

    server = if ssl_conf.enabled {
        // enable HTTPS
        let mut ssl_acceptor_builder =
            SslAcceptor::mozilla_intermediate(SslMethod::tls())
                .expect("Couldn't create SslAcceptor");
        ssl_acceptor_builder
            .set_private_key_file("ssl/key.pem", SslFiletype::PEM)
            .expect("Couldn't set private key");
        ssl_acceptor_builder
            .set_certificate_chain_file("ssl/cert.pem")
            .expect("Couldn't set certificate chain file");
        server.bind_openssl(listen_address, ssl_acceptor_builder)?
    } else {
        // create normal HTTP server
        server.bind(listen_address)?
    };

    if server_conf.workers != 0 {
        server = server.workers(server_conf.workers);
    }

    server.server_hostname(server_conf.url).run().await
}
