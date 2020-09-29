use crate::jwt;
use jsonwebtoken::{encode, EncodingKey, Header};

use actix_web::error::BlockingError;
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorUnauthorized};
use actix_web::{web, Error, HttpResponse};

use std::time::{SystemTime, UNIX_EPOCH};

use crate::database::users::DbErrorType;

use crate::{database, AppState};

#[derive(serde::Deserialize)]
pub struct SignInForm {
    pub user_name: String,
    pub password: String,
}

#[derive(serde::Deserialize)]
pub struct SignUpForm {
    pub user_name: String,
    pub password: String,
    pub email: String, //TODO: captcha: Vec<u8>
}

pub async fn user_info(claims: jwt::Claims) -> HttpResponse {
    HttpResponse::Ok().json(claims)
}

pub async fn sign_in(
    app_state: web::Data<AppState>,
    form: web::Json<SignInForm>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || database::users::authenticate_user(&form, app_state.clone()))
        .await
        .map_err(|outer_err| match outer_err {
            BlockingError::Error(err) => match err.err_type {
                DbErrorType::InternalServerError => ErrorInternalServerError(err.cause),
                DbErrorType::BadRequest => ErrorBadRequest(err.cause),
                DbErrorType::Unauthorized => ErrorUnauthorized(err.cause),
            },
            BlockingError::Canceled => ErrorInternalServerError("database request canceled"),
        })?;

    // Get unix time stamp
    let time_now = SystemTime::now();
    let timestamp = time_now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize;

    // create claims
    let claims = jwt::Claims {
        sub: user.name,
        id: user.id,
        role: user.role,
        exp: timestamp + 7200, // now + two hours
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    );

    match token {
        Ok(token) => Ok(HttpResponse::Ok().body(token)),
        Err(_) => Err(ErrorInternalServerError("JWTs generation failed")),
    }
}

pub async fn sign_up(
    app_state: web::Data<AppState>,
    form: web::Json<SignUpForm>,
) -> Result<HttpResponse, Error> {
    let _user = web::block(move || database::users::add_user(&form, app_state.clone()))
        .await
        .map_err(|outer_err| match outer_err {
            BlockingError::Error(err) => match err.err_type {
                DbErrorType::InternalServerError => ErrorInternalServerError(err.cause),
                DbErrorType::BadRequest => ErrorBadRequest(err.cause),
                DbErrorType::Unauthorized => ErrorUnauthorized(err.cause),
            },
            BlockingError::Canceled => ErrorInternalServerError("database request canceled"),
        })?;

    Ok(HttpResponse::Ok().body("user created"))
}
