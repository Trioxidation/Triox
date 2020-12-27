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
//!
//!🔬 **Modern Technologies** - Authentication with [JWT](https://jwt.io) and a front-end based on [WebAssembly](https://webassembly.org).

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate log;

/// "Apps" in this module take care of certain parts of the API. For example the files app will provide services for uploading and downloading files.
mod apps;

/// This module defines a configuration struct for Triox that allows more robust and efficient access to configuration.
mod app_conf;

/// This module defines a configuration struct for Triox that allows more robust and efficient access to configuration.
mod app_state;

/// API for authentication. Including sign in, sign out and user information.
mod auth;

/// Database structures and helper functions for loading, setting and updating the database.
mod database;

/// Helper functions for hashing and comparing passwords.
mod hash;

/// Structures and extractors for JWT authentication.
mod jwt;

/// Services for handling http errors.
mod error_handler;

/// Tests.
mod tests;

// Cli options
mod cli;

use actix_files::NamedFile;
use actix_web::middleware::errhandlers::ErrorHandlers;
use actix_web::{http, middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use env_logger::Env;

use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

/// index page
async fn index(_req: HttpRequest) -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?.set_content_type(mime::TEXT_HTML_UTF_8))
}

async fn redirect(
    optional_jwt: Option<jwt::JWT>,
    app_state: web::Data<app_state::AppState>,
) -> HttpResponse {
    if let Some(jwt) = optional_jwt {
        if jwt::extract_claims(&jwt.0, &app_state.config.jwt.secret)
            .await
            .is_ok()
        {
            return HttpResponse::Found()
                .header(http::header::LOCATION, "/static/files.html")
                .finish();
        }
    }

    HttpResponse::Found()
        .header(http::header::LOCATION, "/sign_in")
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

    let app_state = app_state::load_app_state(cli_options.config_dir.as_ref());

    // clone config before it is moved into the closure
    let config = app_state.config.clone();

    // setup HTTP server
    let mut server = HttpServer::new(move || {
        let app = App::new()
            // setup error handlers
            .wrap(
                ErrorHandlers::new()
                    .handler(http::StatusCode::NOT_FOUND, error_handler::render_404),
            )
            .wrap(middleware::Compress::default())
            // setup application state extractor
            .data(app_state.clone())
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(redirect))
            // static pages
            .route("/index", web::get().to(index))
            .route("/sign_in", web::get().to(auth::sign_in_page))
            .route("/sign_up", web::get().to(auth::sign_up_page))
            // serve static files from ./static/ to /static/
            .service(actix_files::Files::new("/static", "static"))
            // setup files API
            .configure(apps::files::service_config)
            // setup auth API
            .configure(auth::service_config);

        app
    });

    let listen_address = config.server.listen_address();

    server = if config.ssl.enabled {
        let mut ssl_acceptor_builder =
            SslAcceptor::mozilla_intermediate(SslMethod::tls())
                .expect("Couldn't create SslAcceptor");
        ssl_acceptor_builder
            .set_private_key_file(config.ssl.key_path.as_ref(), SslFiletype::PEM)
            .expect("Couldn't set private key");
        ssl_acceptor_builder
            .set_certificate_chain_file(config.ssl.certificate_path.as_ref())
            .expect("Couldn't set certificate chain file");
        server.bind_openssl(listen_address, ssl_acceptor_builder)?
    } else {
        server.bind(listen_address)?
    };

    if config.server.workers != 0 {
        server = server.workers(config.server.workers);
    }

    server.server_hostname(config.server.url).run().await
}
