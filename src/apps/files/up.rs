use std::io::Write;

use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};

use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::{post, web, Error, HttpResponse};

use crate::jwt;
use crate::AppState;

#[derive(serde::Serialize)]
struct Response {
    path: String,
    files: Vec<String>,
    dirs: Vec<String>,
}

#[post("/app/files/up/{path}")]
pub async fn up(
    app_state: web::Data<AppState>,
    jwt: jwt::JWT,
    web::Path(path): web::Path<String>,
    payload: Multipart,
) -> Result<HttpResponse, Error> {
    upload(app_state, jwt, path, payload).await
}

#[post("/app/files/up/")]
pub async fn up_root(
    app_state: web::Data<AppState>,
    jwt: jwt::JWT,
    payload: Multipart,
) -> Result<HttpResponse, Error> {
    upload(app_state, jwt, String::new(), payload).await
}

/// Service for listing files of the root directory via an API
pub async fn upload(
    app_state: web::Data<AppState>,
    jwt: jwt::JWT,
    path: String,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    let claims = jwt::extract_claims(&jwt.0, &app_state.config.jwt.secret).await?;

    let base_path: std::path::PathBuf =
        [".", "data", "users", &claims.id.to_string(), "files", &path]
            .iter()
            .collect();

    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field
            .content_disposition()
            .ok_or_else(|| ErrorBadRequest("Unknown content type"))?;
        let filename = content_type
            .get_filename()
            .ok_or_else(|| ErrorBadRequest("Unknown file name"))?;

        let mut file_path = base_path.clone();
        file_path.push(filename);

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(file_path))
            .await
            .map_err(ErrorInternalServerError)?;
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(ErrorInternalServerError)?;
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f))
                .await
                .map_err(ErrorInternalServerError)?;
        }
    }
    Ok(HttpResponse::Ok().body("upload finished!"))
}
