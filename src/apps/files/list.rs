use actix_web::error::ErrorInternalServerError;
use actix_web::{get, web, Error, HttpResponse};

use futures::Stream;
use tokio::fs;
use tokio::fs::{DirEntry, ReadDir};

use std::time::SystemTime;

use super::QueryPath;
use crate::app_state::AppState;
use crate::jwt;

#[derive(serde::Serialize)]
struct File {
    name: String,
    size: u64,
    last_modified: u64,
}

#[derive(serde::Serialize)]
struct Directory {
    name: String,
    last_modified: u64,
}

/// File list returned by the `list` and `list_root` services as JSON
#[derive(serde::Serialize)]
struct Response {
    files: Vec<File>,
    directories: Vec<Directory>,
}

/// Service for listing files via an API
#[get("/app/files/list")]
pub async fn list(
    app_state: web::Data<AppState>,
    jwt: jwt::JWT,
    web::Query(query_path): web::Query<QueryPath>,
) -> Result<HttpResponse, Error> {
    let claims = jwt::extract_claims(&jwt.0, &app_state.config.server.secret)?;

    let full_path = super::resolve_path(claims.id, &query_path.path)?;

    let mut dir: ReadDir = fs::read_dir(&full_path)
        .await
        .map_err(ErrorInternalServerError)?;

    let (lower_bound, upper_bound) = dir.size_hint();

    let upper_bound = match upper_bound {
        Some(bound) => bound,
        None => lower_bound,
    };

    let dir_size = upper_bound - lower_bound;

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
        let file_name = entry.file_name().into_string().map_err(|err| {
            ErrorInternalServerError(
                err.to_str()
                    .unwrap_or("String conversion failed")
                    .to_owned(),
            )
        })?;
        let last_modified: u64 = entry
            .metadata()
            .await?
            .modified()
            .unwrap_or(SystemTime::UNIX_EPOCH)
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_else(|_| std::time::Duration::new(0, 0))
            .as_secs();

        match entry
            .file_type()
            .await
            .map_err(ErrorInternalServerError)?
            .is_file()
        {
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

    Ok(HttpResponse::Ok().json(Response { directories, files }))
}
