use actix_web::{web, HttpResponse};

use crate::errors::*;

/// Service for creating directories
#[my_codegen::get(path = "crate::FILE_ROUTES.create_dir", wrap = "crate::CheckLogin")]
pub async fn create_dir(
    web::Query(query_path): web::Query<super::QueryPath>,
    id: actix_identity::Identity,
) -> ServiceResult<HttpResponse> {
    super::read_only_guard()?;

    let username = id.identity().unwrap();

    let full_path = super::resolve_path(&username, &query_path.path)?;

    tokio::fs::create_dir_all(&full_path).await?;

    Ok(HttpResponse::Ok().body("Directory successfully created"))
}
