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

/// Tests.
mod tests;

use actix_files::NamedFile;
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
    // setup logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let app_state = app_state::load_app_state("config");

    // clone config before it is moved into the closure
    let server_conf = app_state.config.server.clone();
    let ssl_conf = app_state.config.ssl.clone();
    let users_conf = app_state.config.users.clone();

    // setup HTTP server
    let mut server = HttpServer::new(move || {
        let app = App::new()
            // setup application state extractor
            .data(app_state.clone())
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(redirect))
            // static pages
            .route("/index", web::get().to(index))
            .route("/user_info", web::get().to(auth::user_info))
            .route("/sign_in", web::get().to(auth::sign_in_page))
            // authentication API
            .route("/sign_in", web::post().to(auth::sign_in))
            .route("/delete_user", web::post().to(auth::delete_user))
            // file app API
            .service(apps::files::get::get)
            .service(apps::files::list::list)
            .service(apps::files::upload::upload)
            .service(apps::files::copy::copy)
            .service(apps::files::r#move::r#move)
            .service(apps::files::remove::remove)
            .service(apps::files::create_dir::create_dir)
            // serve static files from ./static/ to /static/
            .service(actix_files::Files::new("/static", "static"));

        let app = if !users_conf.disable_sign_up {
            app.route("/sign_up", web::get().to(auth::sign_up_page))
                .route("/sign_up", web::post().to(auth::sign_up))
        } else {
            app
        };

        app
    });

    let listen_address = server_conf.listen_address();

    server = if ssl_conf.enabled {
        let mut ssl_acceptor_builder =
            SslAcceptor::mozilla_intermediate(SslMethod::tls())
                .expect("Couldn't create SslAcceptor");
        ssl_acceptor_builder
            .set_private_key_file(ssl_conf.key_path.as_ref(), SslFiletype::PEM)
            .expect("Couldn't set private key");
        ssl_acceptor_builder
            .set_certificate_chain_file(ssl_conf.certificate_path.as_ref())
            .expect("Couldn't set certificate chain file");
        server.bind_openssl(listen_address, ssl_acceptor_builder)?
    } else {
        server.bind(listen_address)?
    };

    if server_conf.workers != 0 {
        server = server.workers(server_conf.workers);
    }

    server.server_hostname(server_conf.url).run().await
}
