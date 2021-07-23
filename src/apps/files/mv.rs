use actix_web::{web, HttpResponse};

use crate::errors::*;

/// Service for deleting files or directories
#[my_codegen::post(path = "crate::FILE_ROUTES.mv", wrap = "crate::CheckLogin")]
pub async fn mv(
    id: actix_identity::Identity,
    params: web::Json<super::SourceAndDest>,
) -> ServiceResult<HttpResponse> {
    super::read_only_guard()?;

    let username = id.identity().unwrap();

    let source_path = super::resolve_path(&username, &params.from)?;
    let destination_path = super::resolve_path(&username, &params.to)?;

    let metadata = tokio::fs::metadata(&source_path).await?;

    tokio::fs::rename(&source_path, &destination_path).await?;

    if metadata.is_dir() {
        Ok(HttpResponse::Ok().body("Directory successfully moved"))
    } else {
        Ok(HttpResponse::Ok().body("File successfully moved"))
    }
}
