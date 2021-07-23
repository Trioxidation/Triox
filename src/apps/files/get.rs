use actix_files::NamedFile;
use actix_web::web;

use crate::errors::*;

/// Service for downloading files via an API
#[my_codegen::get(path = "crate::FILE_ROUTES.get", wrap = "crate::CheckLogin")]
pub async fn get(
    id: actix_identity::Identity,
    web::Query(query_path): web::Query<super::QueryPath>,
) -> ServiceResult<NamedFile> {
    let username = id.identity().unwrap();
    let full_path = super::resolve_path(&username, &query_path.path)?;
    Ok(NamedFile::open(&full_path)?)
}
