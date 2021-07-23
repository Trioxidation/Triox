use actix_web::{web, HttpResponse};

use crate::errors::*;

/// Service for deleting files or directories
#[my_codegen::post(path = "crate::FILE_ROUTES.copy", wrap = "crate::CheckLogin")]
pub async fn copy(
    id: actix_identity::Identity,
    payload: web::Json<super::SourceAndDest>,
) -> ServiceResult<HttpResponse> {
    super::read_only_guard()?;

    let username = id.identity().unwrap();

    let source_path = super::resolve_path(&username, &payload.from)?;
    let destination_path = super::resolve_path(&username, &payload.to)?;

    let metadata = tokio::fs::metadata(&source_path).await?;

    tokio::fs::copy(&source_path, &destination_path).await?;

    if metadata.is_dir() {
        Ok(HttpResponse::Ok().body("Directory successfully copied"))
    } else {
        Ok(HttpResponse::Ok().body("File successfully copied"))
    }
}
