use actix_files::NamedFile;
use actix_web::{get, web};

use crate::errors::*;

/// Service for downloading files via an API
#[get("/app/files/get", wrap = "crate::CheckLogin")]
pub async fn get(
    id: actix_identity::Identity,
    web::Query(query_path): web::Query<super::QueryPath>,
) -> ServiceResult<NamedFile> {
    let username = id.identity().unwrap();
    let full_path = super::resolve_path(&username, &query_path.path)?;
    Ok(NamedFile::open(&full_path)?)
}
