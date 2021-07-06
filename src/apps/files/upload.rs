use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};

use actix_web::{post, web, HttpResponse};

use tokio::fs;
use tokio::prelude::*;

use crate::app_state::AppState;
use crate::errors::*;

#[derive(serde::Serialize)]
struct Response {
    path: String,
    files: Vec<String>,
    dirs: Vec<String>,
}

/// Service for listing files
#[post("/app/files/upload", wrap = "crate::CheckLogin")]
pub async fn upload(
    id: actix_identity::Identity,
    web::Query(query_path): web::Query<super::QueryPath>,
    mut payload: Multipart,
) -> ServiceResult<HttpResponse> {
    super::read_only_guard()?;

    let username = id.identity().unwrap();

    let base_path = super::resolve_path(&username, &query_path.path)?;

    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field
            .content_disposition()
            .ok_or_else(|| ServiceError::UnknownMIME)?;

        if let Some(filename) = content_type.get_filename() {
            if filename.contains("..") {
                return Err(ServiceError::PermissionDenied);
            }

            let mut file_path = base_path.clone();
            file_path.push(filename);

            let mut file = fs::File::create(file_path).await?;

            // Field in turn is stream of *Bytes* object
            while let Some(chunk) = field.next().await {
                file.write_all(&chunk?).await?;
            }
        } else {
            return Err(ServiceError::BadRequest);
        }
    }

    Ok(HttpResponse::Ok().body("Upload finished"))
}
