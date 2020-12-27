use actix_web::web;

/// Download files
pub mod get;

/// List files in directory
pub mod list;

/// Upload files to the server
pub mod upload;

/// Create directories
pub mod create_dir;

/// Delete files and directories
pub mod remove;

/// Move files and directories
// move is also a keyword, so it necessary to escape it
pub mod r#move;

/// Copy files and directories
pub mod copy;

/// Shared struct for extracting paths
#[derive(serde::Deserialize)]
pub struct QueryPath {
    path: String,
}

/// Shared struct for moving or copying files
#[derive(serde::Deserialize)]
pub struct SourceAndDest {
    from: String,
    to: String,
}

/// Helper function to translate paths from requests into absolute path
fn resolve_path(
    user_id: u32,
    query_path: &str,
) -> Result<std::path::PathBuf, actix_web::Error> {
    if query_path.contains("..") {
        Err(actix_web::error::ErrorBadRequest(
            "Moving up directories is not allowed",
        ))
    } else {
        Ok(std::path::PathBuf::from(format!(
            "data/users/{}/files/{}",
            user_id, query_path
        )))
    }
}

/// Helper function to
fn read_only_guard(config: &crate::app_conf::AppConfig) -> Result<(), actix_web::Error> {
    if config.files.read_only {
        Err(actix_web::error::ErrorForbidden("Read only mode is active"))
    } else {
        Ok(())
    }
}

// Configure files app services
pub fn service_config(cfg: &mut web::ServiceConfig) {
    cfg
        // read only file app API
        .service(get::get)
        .service(list::list)
        // file modifying file app API
        .service(upload::upload)
        .service(r#move::r#move)
        .service(copy::copy)
        .service(remove::remove)
        .service(create_dir::create_dir);
}
