use actix_web::{post, web, Error, HttpResponse};

use crate::app_state::AppState;
use crate::jwt;

/// Service for deleting files or directories
#[post("/app/files/move")]
pub async fn r#move(
    app_state: web::Data<AppState>,
    jwt: jwt::JWT,
    params: web::Json<super::SourceAndDest>,
) -> Result<HttpResponse, Error> {
    super::read_only_guard(&app_state.config)?;

    let claims = jwt::extract_claims(&jwt.0, &app_state.config.jwt.secret).await?;

    let source_path = super::resolve_path(claims.id, &params.from)?;
    let destination_path = super::resolve_path(claims.id, &params.to)?;

    let metadata = tokio::fs::metadata(&source_path).await?;

    if metadata.is_dir() {
        let copy_options = fs_extra::dir::CopyOptions::new();
        web::block(move || {
            fs_extra::dir::move_dir(&source_path, &destination_path, &copy_options)
        })
        .await?;

        Ok(HttpResponse::Ok().body("Directory successfully moved"))
    } else {
        let copy_options = fs_extra::file::CopyOptions::new();
        web::block(move || {
            fs_extra::file::move_file(&source_path, &destination_path, &copy_options)
        })
        .await?;

        Ok(HttpResponse::Ok().body("File successfully moved"))
    }
}
