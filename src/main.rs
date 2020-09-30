// this is required to fix an issue with Rust 1.46...
#![type_length_limit = "100000000"]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate log;

mod app_conf;
mod apps;
mod auth;
mod database;
mod hash;
mod jwt;
mod tests;

use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use env_logger::Env;

use config::Config;
use dashmap::DashMap;

use diesel::r2d2::{self, ConnectionManager};
use diesel::MysqlConnection;
pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

async fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("<h1>INDEX PAGE</h1>")
}

#[derive(Clone)]
pub struct AppState {
    db_pool: DbPool,
    config: app_conf::AppConfig<'static>,
    login_count: DashMap<u32, u8>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
    let db_pool =
        database::connect(&config.database.url()).expect("Failed to create database pool.");

    // setup logger
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    // setup HTTP server
    let mut server = HttpServer::new(move || {
        App::new()
            // setup application state extractor
            .data(AppState {
                config,
                login_count: DashMap::new(),
                db_pool: db_pool.clone(),
            })
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
            .route("/user_info", web::get().to(auth::user_info))
            .route("/sign_up", web::post().to(auth::sign_up))
            .route("/sign_in", web::post().to(auth::sign_in))
            .service(apps::files::get::get)
            .service(apps::files::list::list)
            .service(apps::files::list::list_root)
    });

    let listen_address = config.server.listen_address();

    server = if config.ssl.enabled {
        // enable HTTPS
        let mut ssl_acceptor_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())
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

    server
        .server_hostname(config.server.url)
        .workers(4)
        .run()
        .await
}
