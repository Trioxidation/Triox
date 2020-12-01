use std::io::Write;

use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};

use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::{post, web, Error, HttpResponse};

use crate::app_state::AppState;
use crate::jwt;

#[derive(serde::Serialize)]
struct Response {
    path: String,
    files: Vec<String>,
    dirs: Vec<String>,
}

/// Service for listing files
#[post("/app/files/upload")]
pub async fn upload(
    app_state: web::Data<AppState>,
    jwt: jwt::JWT,
    web::Query(query_path): web::Query<super::QueryPath>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    let claims = jwt::extract_claims(&jwt.0, &app_state.config.jwt.secret).await?;

    let base_path = super::resolve_path(claims.id, &query_path.path)?;

    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field
            .content_disposition()
            .ok_or_else(|| ErrorBadRequest("Unknown content type"))?;
        let filename = content_type
            .get_filename()
            .ok_or_else(|| ErrorBadRequest("Unknown file name"))?;

        if filename.contains("..") {
            return Err(actix_web::error::ErrorBadRequest(
                "Moving up directories is not allowed!",
            ));
        }

        let mut file_path = base_path.clone();
        file_path.push(filename);

        // File::create is blocking operation, use thread pool
        let mut f = web::block(|| std::fs::File::create(file_path))
            .await
            .map_err(ErrorInternalServerError)?;
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(ErrorInternalServerError)?;
            // filesystem operations are blocking, we have to use thread pool
            f = web::block(move || f.write_all(&data).map(|_| f))
                .await
                .map_err(ErrorInternalServerError)?;
        }
    }
    Ok(HttpResponse::Ok().body("upload finished!"))
}
