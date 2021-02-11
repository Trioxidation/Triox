use actix_web::{get, web, Error, HttpResponse};

use crate::app_state::AppState;
use crate::jwt;

/// Service for creating directories
#[get("/app/files/create_dir")]
pub async fn create_dir(
    app_state: web::Data<AppState>,
    jwt: jwt::JWT,
    web::Query(query_path): web::Query<super::QueryPath>,
) -> Result<HttpResponse, Error> {
    super::read_only_guard(&app_state.config)?;

    let claims = jwt::extract_claims(&jwt.0, &app_state.config.server.secret)?;

    let full_path = super::resolve_path(claims.id, &query_path.path)?;

    tokio::fs::create_dir_all(&full_path).await?;

    Ok(HttpResponse::Ok().body("Directory successfully created"))
}
