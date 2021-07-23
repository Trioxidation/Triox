//! # Triox - a cloud server for the next generation
//!
//!☘️ **Open Source** - We strongly believe in collaboration and transparency.
//!
//!⚡ **Speed** - Get the most out of your hardware! Triox runs fast, even on weak hardware.
//!
//!🔒 **Security** - We're using state-of-the-art algorithms and authentication methods to protect your data.
//!
//!⛓️ **Reliability** - Built on top of the strong guarantees of the [Rust programming language](https://rust-lang.org).
//!
//!🛫 **Easy Setup** - Triox comes with batteries included and is easy to configure.

mod api;
mod middleware;

/// "Apps" in this module take care of certain parts of the API. For example the files app will provide services for uploading and downloading files.
mod apps;

/// This module defines a configuration struct for Triox that allows more robust and efficient access to configuration.
mod config;

/// This module defines a configuration struct for Triox that allows more robust and efficient access to configuration.
mod app_state;

/// API for authentication. Including sign in, sign out and user information.
mod auth;

/// Tests.
#[cfg(test)]
#[macro_use]
mod tests;

/// errors.
mod errors;

// Cli options
mod cli;

use std::sync::Arc;

use actix_files::NamedFile;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{http, web, App, HttpRequest, HttpResponse, HttpServer};
use env_logger::Env;
use lazy_static::lazy_static;

use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use crate::api::v1::ROUTES as V1_API_ROUTES;
use crate::apps::files::FILE_ROUTES;
use crate::config::AppConfig;

pub use crate::app_state::AppState;
pub use crate::middleware::auth::CheckLogin;

pub type AppData = actix_web::web::Data<Arc<AppState>>;

pub const GIT_COMMIT_HASH: &str = env!("GIT_HASH");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub const PKG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const PKG_HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");

lazy_static! {
    pub static ref SETTINGS: AppConfig = {
        let cli_options = cli::Options::new();
        AppConfig::new(cli_options.config_dir.as_ref()).unwrap()
    };
}

/// index page
async fn index(_req: HttpRequest) -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?.set_content_type(mime::TEXT_HTML_UTF_8))
}

#[actix_web::get("/", wrap = "crate::CheckLogin")]
async fn redirect() -> HttpResponse {
    HttpResponse::Found()
        .append_header((http::header::LOCATION, "/static/files.html"))
        .finish()
}

/// For AGPL compliance Triox needs to allow users to download the source code over the network
async fn source_code(_req: HttpRequest) -> HttpResponse {
    // If you modify the source code and use it in for a public network service
    // you need to update this link to point to a copy of your modified version
    // More info: https://www.gnu.org/licenses/why-affero-gpl.html
    HttpResponse::SeeOther()
        .append_header((
            http::header::LOCATION,
            "https://github.com/Trioxidation/Triox",
        ))
        .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli_options = cli::Options::new();

    // setup logger
    env_logger::Builder::from_env(
        Env::default().default_filter_or(cli_options.log_level),
    )
    .init();

    // initialize static variables to prevent panicking later
    lazy_static::initialize(&SETTINGS);
    lazy_static::initialize(&middleware::rate_limit::RATE_LIMIT_CONFIG);

    let app_state = app_state::AppState::new().await;

    sqlx::migrate!("./migrations/")
        .run(&app_state.db)
        .await
        .unwrap();

    let app_state = actix_web::web::Data::new(app_state);

    // setup HTTP server
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Compress::default())
            .wrap(get_identity_service())
            .wrap(
                actix_web::middleware::ErrorHandlers::new()
                    .handler(http::StatusCode::NOT_FOUND, errors::render_404),
            )
            .wrap(actix_web::middleware::NormalizePath::new(
                actix_web::middleware::TrailingSlash::Trim,
            ))
            // setup application state extractor
            .app_data(app_state.clone())
            .wrap(actix_web::middleware::Logger::default())
            .service(redirect)
            .route("/source", web::get().to(source_code))
            // static pages
            .route("/index", web::get().to(index))
            .route("/sign_in", web::get().to(auth::sign_in_page))
            .route("/sign_up", web::get().to(auth::sign_up_page))
            // serve static files from ./static/ to /static/
            .service(actix_files::Files::new("/static", "static"))
            // setup files API
            .configure(apps::files::services)
            // setup auth API
            .configure(api::v1::services)
    });

    let listen_address = SETTINGS.server.listen_address();

    server = if SETTINGS.tls.enabled {
        let mut ssl_acceptor_builder =
            SslAcceptor::mozilla_intermediate(SslMethod::tls())
                .expect("Couldn't create SslAcceptor");
        ssl_acceptor_builder
            .set_private_key_file(
                SETTINGS.tls.key_path.as_ref().unwrap(),
                SslFiletype::PEM,
            )
            .expect("Couldn't set private key");
        ssl_acceptor_builder
            .set_certificate_chain_file(SETTINGS.tls.certificate_path.as_ref().unwrap())
            .expect("Couldn't set certificate chain file");
        server.bind_openssl(listen_address, ssl_acceptor_builder)?
    } else {
        server.bind(listen_address)?
    };

    if SETTINGS.server.workers != 0 {
        server = server.workers(SETTINGS.server.workers);
    }

    server.server_hostname(&SETTINGS.server.host).run().await
}

#[cfg(not(tarpaulin_include))]
pub fn get_identity_service() -> IdentityService<CookieIdentityPolicy> {
    let cookie_secret = &SETTINGS.server.secret;
    IdentityService::new(
        CookieIdentityPolicy::new(cookie_secret.as_bytes())
            .name("Authorization")
            //TODO change cookie age
            .max_age_secs(216000)
            .domain(&SETTINGS.server.domain)
            .secure(false),
    )
}
