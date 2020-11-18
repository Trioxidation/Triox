use actix_web::{get, web, Error, HttpResponse};

use crate::jwt;
use crate::AppState;

/// Service for creating directories
#[get("/app/files/create_dir/{path}")]
pub async fn create_dir(
    app_state: web::Data<AppState>,
    jwt: jwt::JWT,
    web::Path(path): web::Path<String>,
) -> Result<HttpResponse, Error> {
    let claims = jwt::extract_claims(&jwt.0, &app_state.config.jwt.secret).await?;

    let path: std::path::PathBuf =
        [".", "data", "users", &claims.id.to_string(), "files", &path]
            .iter()
            .collect();

    tokio::fs::create_dir_all(&path).await?;

    Ok(HttpResponse::Ok().body("directory successfully created!"))
}
