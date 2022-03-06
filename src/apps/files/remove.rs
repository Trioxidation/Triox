use actix_web::{web, HttpResponse};

use crate::errors::*;

/// Service for deleting files or directories
#[my_codegen::get(path = "crate::FILE_ROUTES.remove", wrap = "crate::CheckLogin")]
pub async fn remove(
    id: actix_identity::Identity,
    web::Query(query_path): web::Query<super::QueryPath>,
) -> ServiceResult<HttpResponse> {
    super::read_only_guard()?;

    let username = id.identity().unwrap();

    let full_path = super::resolve_path(&username, &query_path.path)?;

    let metadata = tokio::fs::metadata(&full_path).await?;

    if metadata.is_dir() {
        tokio::fs::remove_dir_all(&full_path).await?;
        Ok(HttpResponse::Ok().body("Directory successfully deleted"))
    } else {
        tokio::fs::remove_file(&full_path).await?;
        Ok(HttpResponse::Ok().body("File successfully deleted"))
    }
}
