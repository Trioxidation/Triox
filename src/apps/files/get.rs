use actix_files::NamedFile;
use actix_web::error::ErrorInternalServerError;
use actix_web::{get, web, Error};

use crate::jwt;

#[get("/app/files/get/{path}")]
pub async fn get(
    claims: jwt::Claims,
    web::Path(path): web::Path<String>,
) -> Result<NamedFile, Error> {
    let path: std::path::PathBuf = [".", "data", "users", &claims.id.to_string(), "files", &path]
        .iter()
        .collect();

    Ok(NamedFile::open(path).map_err(ErrorInternalServerError)?)
}
