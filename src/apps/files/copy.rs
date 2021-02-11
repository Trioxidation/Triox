use actix_web::{post, web, Error, HttpResponse};

use crate::app_state::AppState;
use crate::jwt;

/// Service for deleting files or directories
#[post("/app/files/copy")]
pub async fn copy(
    app_state: web::Data<AppState>,
    jwt: jwt::JWT,
    params: web::Json<super::SourceAndDest>,
) -> Result<HttpResponse, Error> {
    super::read_only_guard(&app_state.config)?;

    let claims = jwt::extract_claims(&jwt.0, &app_state.config.server.secret)?;

    let source_path = super::resolve_path(claims.id, &params.from)?;
    let destination_path = super::resolve_path(claims.id, &params.to)?;

    let metadata = tokio::fs::metadata(&source_path).await?;

    tokio::fs::copy(&source_path, &destination_path).await?;

    if metadata.is_dir() {
        Ok(HttpResponse::Ok().body("Directory successfully copied"))
    } else {
        Ok(HttpResponse::Ok().body("File successfully copied"))
    }
}
