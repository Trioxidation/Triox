use actix_web::{get, web, Error, HttpResponse};

use crate::jwt;
use crate::app_state::AppState;

/// Service for deleting files or directories
#[get("/app/files/remove")]
pub async fn remove(
    app_state: web::Data<AppState>,
    jwt: jwt::JWT,
    web::Query(query_path): web::Query<super::QueryPath>,
) -> Result<HttpResponse, Error> {
    let claims = jwt::extract_claims(&jwt.0, &app_state.config.jwt.secret).await?;

    let full_path = super::resolve_path(claims.id, &query_path.path)?;

    let metadata = tokio::fs::metadata(&full_path).await?;

    if metadata.is_dir() {
        tokio::fs::remove_dir_all(&full_path).await?;
        Ok(HttpResponse::Ok().body("directory successfully deleted!"))
    } else {
        tokio::fs::remove_file(&full_path).await?;
        Ok(HttpResponse::Ok().body("file successfully deleted!"))
    }
}
