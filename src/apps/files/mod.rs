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

pub fn resolve_path(user_id: u32, query_path: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(format!("data/users/{}/files/{}", user_id, query_path))
}

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
