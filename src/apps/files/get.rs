use actix_files::NamedFile;
use actix_web::error::ErrorInternalServerError;
use actix_web::{get, web, Error};

use crate::app_state::AppState;
use crate::jwt;

/// Service for downloading files via an API
#[get("/app/files/get")]
pub async fn get(
    app_state: web::Data<AppState>,
    jwt: jwt::JWT,
    web::Query(query_path): web::Query<super::QueryPath>,
) -> Result<NamedFile, Error> {
    let claims = jwt::extract_claims(&jwt.0, &app_state.config.server.secret).await?;

    let full_path = super::resolve_path(claims.id, &query_path.path)?;

    Ok(NamedFile::open(&full_path).map_err(ErrorInternalServerError)?)
}
