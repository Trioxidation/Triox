use std::time::SystemTime;

use actix_web::{web, HttpResponse};
use futures::Stream;
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::fs::DirEntry;
use tokio_stream::wrappers::ReadDirStream;

use super::QueryPath;
use crate::errors::*;

#[derive(Deserialize, Serialize)]
pub struct File {
    pub name: String,
    pub size: u64,
    pub last_modified: u64,
}

#[derive(Deserialize, Serialize)]
pub struct Directory {
    pub name: String,
    pub last_modified: u64,
}

/// File list returned by the `list` and `list_root` services as JSON
#[derive(Deserialize, Serialize)]
pub struct ListResponse {
    pub files: Vec<File>,
    pub directories: Vec<Directory>,
}

/// Service for listing files via an API
#[my_codegen::get(path = "crate::FILE_ROUTES.list", wrap = "crate::CheckLogin")]
pub async fn list(
    id: actix_identity::Identity,
    web::Query(query_path): web::Query<QueryPath>,
) -> ServiceResult<HttpResponse> {
    let username = id.identity().unwrap();

    let full_path = super::resolve_path(&username, &query_path.path)?;

    let dir = ReadDirStream::new(fs::read_dir(&full_path).await?);

    let (lower_bound, upper_bound) = dir.size_hint();

    let upper_bound = match upper_bound {
        Some(bound) => bound,
        None => lower_bound,
    };

    let dir_size = upper_bound - lower_bound;

    let mut dir = dir.into_inner();

    let mut entries: Vec<DirEntry> = Vec::with_capacity(dir_size);

    loop {
        let new_entry = dir.next_entry().await;
        if let Ok(Some(entry)) = new_entry {
            entries.push(entry);
        } else {
            // None -> assuming there are no further entries
            break;
        }
    }

    let mut files: Vec<File> = Vec::new();
    let mut directories: Vec<Directory> = Vec::new();

    for entry in entries {
        let file_name = entry
            .file_name()
            .into_string()
            .map_err(|_| ServiceError::InternalServerError)?;
        let last_modified: u64 = entry
            .metadata()
            .await?
            .modified()
            .unwrap_or(SystemTime::UNIX_EPOCH)
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_else(|_| std::time::Duration::new(0, 0))
            .as_secs();

        match entry.file_type().await?.is_file() {
            true => files.push(File {
                name: file_name,
                size: entry.metadata().await?.len(),
                last_modified,
            }),
            false => directories.push(Directory {
                name: file_name,
                last_modified,
            }),
        }
    }

    Ok(HttpResponse::Ok().json(ListResponse { files, directories }))
}
