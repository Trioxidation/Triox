use actix_web::web;

use crate::errors::*;

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
fn resolve_path(username: &str, query_path: &str) -> ServiceResult<std::path::PathBuf> {
    if query_path.contains("..") {
        Err(ServiceError::PermissionDenied)
    } else {
        Ok(std::path::PathBuf::from(format!(
            "data/users/{}/files/{}",
            username, query_path
        )))
    }
}

/// Helper function to
fn read_only_guard() -> ServiceResult<()> {
    if crate::SETTINGS.files.read_only {
        Err(ServiceError::FSReadOnly)
    } else {
        Ok(())
    }
}

// Configure files app services
pub fn services(cfg: &mut web::ServiceConfig) {
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
