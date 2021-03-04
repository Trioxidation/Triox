use actix_files::NamedFile;
use actix_web::{get, web};

use crate::app_state::AppState;
use crate::errors::*;
use crate::jwt;

/// Service for downloading files via an API
#[get("/app/files/get")]
pub async fn get(
    app_state: web::Data<AppState>,
    jwt: jwt::JWT,
    web::Query(query_path): web::Query<super::QueryPath>,
) -> ServiceResult<NamedFile> {
    let claims = jwt::extract_claims(&jwt.0, &app_state.config.server.secret)?;

    let full_path = super::resolve_path(claims.id, &query_path.path)?;
    Ok(NamedFile::open(&full_path)?)
}
