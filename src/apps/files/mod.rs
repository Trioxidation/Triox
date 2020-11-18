/// Download files
pub mod get;

/// List files in directory
pub mod list;

/// Upload files to the server
pub mod up;

/// Create directories
pub mod create_dir;

/// Delete files and directories
pub mod remove;

/// Move files and directories
// move is also a keyword, so it necessary to escape it
pub mod r#move;

/// Copy files and directories
pub mod copy;

/// Shared struct for moving or copying files
#[derive(serde::Deserialize)]
pub struct SourceAndDest {
    from: String,
    to: String,
}
