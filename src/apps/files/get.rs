use actix_files::NamedFile;
use actix_web::error::ErrorInternalServerError;
use actix_web::{get, web, Error};

use crate::jwt;
use crate::AppState;

/// Service for downloading files via an API
#[get("/app/files/get/{path}")]
pub async fn get(
    app_state: web::Data<AppState>,
    jwt: jwt::JWT,
    web::Path(path): web::Path<String>,
) -> Result<NamedFile, Error> {
    let claims = jwt::extract_claims(&jwt.0, &app_state.config.jwt.secret).await?;

    let path: std::path::PathBuf = [".", "data", "users", &claims.id.to_string(), "files", &path]
        .iter()
        .collect();

    Ok(NamedFile::open(path).map_err(ErrorInternalServerError)?)
}
