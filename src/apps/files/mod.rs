use actix_web::web;
use serde::{Deserialize, Serialize};

use crate::errors::*;

/// Copy files and directories
pub mod copy;
/// Create directories
pub mod create_dir;
/// Download files
pub mod get;
/// List files in directory
pub mod list;
/// Move files and directories
// move is also a keyword, so it necessary to escape it
pub mod mv;
/// Delete files and directories
pub mod remove;
/// Upload files to the server
pub mod upload;

pub const FILE_ROUTES: routes::Files = routes::Files::new();

/// Shared struct for extracting paths
#[derive(Deserialize)]
pub struct QueryPath {
    pub path: String,
}

/// Shared struct for moving or copying files
#[derive(Deserialize, Serialize)]
pub struct SourceAndDest {
    pub from: String,
    pub to: String,
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
        .service(mv::mv)
        .service(copy::copy)
        .service(remove::remove)
        .service(create_dir::create_dir);
}

pub mod routes {
    pub struct Files {
        pub get: &'static str,
        pub list: &'static str,
        pub upload: &'static str,
        pub mv: &'static str,
        pub copy: &'static str,
        pub remove: &'static str,
        pub create_dir: &'static str,
    }

    impl Files {
        pub const fn new() -> Files {
            Files {
                get: "/app/files/get",
                list: "/app/files/list",
                upload: "/app/files/upload",
                mv: "/app/files/move",
                copy: "/app/files/copy",
                remove: "/app/files/remove",
                create_dir: "/app/files/create_dir",
            }
        }
    }
}
