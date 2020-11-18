use actix_web::{post, web, Error, HttpResponse};

use crate::jwt;
use crate::AppState;

/// Service for deleting files or directories
#[post("/app/files/move")]
pub async fn r#move(
    app_state: web::Data<AppState>,
    jwt: jwt::JWT,
    params: web::Json<super::SourceAndDest>,
) -> Result<HttpResponse, Error> {
    let claims = jwt::extract_claims(&jwt.0, &app_state.config.jwt.secret).await?;

    let mut source_path: std::path::PathBuf =
        [".", "data", "users", &claims.id.to_string(), "files"]
            .iter()
            .collect();

    let mut destination_path = source_path.clone();

    source_path.push(&params.from);
    destination_path.push(&params.to);

    let metadata = tokio::fs::metadata(&source_path).await?;

    if metadata.is_dir() {
        let copy_options = fs_extra::dir::CopyOptions::new();
        web::block(move || {
            fs_extra::dir::move_dir(&source_path, &destination_path, &copy_options)
        })
        .await?;

        Ok(HttpResponse::Ok().body("directory successfully moved!"))
    } else {
        let copy_options = fs_extra::file::CopyOptions::new();
        web::block(move || {
            fs_extra::file::move_file(&source_path, &destination_path, &copy_options)
        })
        .await?;

        Ok(HttpResponse::Ok().body("file successfully moved!"))
    }
}
