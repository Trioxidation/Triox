use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
//use std::future::Future;

use actix_web::{web, HttpResponse, Responder};

use tokio::fs;
use tokio::io::AsyncWriteExt;

use crate::errors::*;

#[derive(serde::Serialize)]
struct Response {
    path: String,
    files: Vec<String>,
    dirs: Vec<String>,
}

/// Service for listing files
#[my_codegen::post(path = "crate::FILE_ROUTES.upload", wrap = "crate::CheckLogin")]
pub async fn upload(
    id: actix_identity::Identity,
    web::Query(query_path): web::Query<super::QueryPath>,
    mut payload: Multipart,
) -> ServiceResult<impl Responder> {
    super::read_only_guard()?;

    let username = id.identity().unwrap();

    let base_path = super::resolve_path(&username, &query_path.path)?;

    loop {
        match payload.try_next().await {
            Ok(Some(mut field)) => {
                let content_type = field.content_disposition();
                if let Some(filename) = content_type.get_filename() {
                    if filename.contains("..") {
                        return Err(ServiceError::PermissionDenied);
                    }

                    let mut file_path = base_path.clone();
                    file_path.push(filename);
                    println!("uploading file: {} at {:?}", filename, file_path);

                    let mut file = fs::File::create(file_path).await?;

                    // Field in turn is stream of *Bytes* object
                    while let Some(chunk) = field.next().await {
                        file.write_all(&chunk?).await?;
                    }
                } else {
                    return Err(ServiceError::BadRequest);
                }
            }
            Ok(None) => {
                return Ok(HttpResponse::Ok().body("Upload finished"));
            }

            Err(e) => return Err(e.into()),
        }
    }
}
