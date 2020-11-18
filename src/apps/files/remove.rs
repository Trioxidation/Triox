use actix_web::{get, web, Error, HttpResponse};

use crate::jwt;
use crate::AppState;

/// Service for deleting files or directories
#[get("/app/files/remove/{path}")]
pub async fn remove(
    app_state: web::Data<AppState>,
    jwt: jwt::JWT,
    web::Path(path): web::Path<String>,
) -> Result<HttpResponse, Error> {
    let claims = jwt::extract_claims(&jwt.0, &app_state.config.jwt.secret).await?;

    let path: std::path::PathBuf =
        [".", "data", "users", &claims.id.to_string(), "files", &path]
            .iter()
            .collect();

    let metadata = tokio::fs::metadata(&path).await?;

    if metadata.is_dir() {
        tokio::fs::remove_dir_all(&path).await?;
        Ok(HttpResponse::Ok().body("directory successfully deleted!"))
    } else {
        tokio::fs::remove_file(&path).await?;
        Ok(HttpResponse::Ok().body("file successfully deleted!"))
    }
}
