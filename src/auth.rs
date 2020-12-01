use crate::jwt;
use jsonwebtoken::{encode, EncodingKey, Header};

use actix_files::NamedFile;
use actix_web::error::BlockingError;
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorUnauthorized};
use actix_web::{http, web, Error, HttpRequest, HttpResponse};

use std::time::{SystemTime, UNIX_EPOCH};

use crate::database::users::DbErrorType;

use crate::{app_state::AppState, database};

/// Information required for sign in.
#[derive(serde::Deserialize)]
pub struct SignInForm {
    pub user_name: String,
    pub password: String,
    pub cookie: Option<bool>,
}

/// Information required for sign up.
#[derive(serde::Deserialize)]
pub struct SignUpForm {
    pub user_name: String,
    pub password: String,
    pub email: String, //TODO: captcha: Vec<u8>
}

/// Information required for deleting a user.
#[derive(serde::Deserialize)]
pub struct DeleteUserForm {
    pub user_name: String,
    pub password: String,
}

/// Return user information stored inside the JWT.
pub async fn user_info(
    app_state: web::Data<AppState>,
    jwt: jwt::JWT,
) -> Result<HttpResponse, Error> {
    let claims = jwt::extract_claims(&jwt.0, &app_state.config.jwt.secret).await?;
    Ok(HttpResponse::Ok().json(claims))
}

/// Give user sign in page.
pub async fn sign_in_page(_req: HttpRequest) -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("static/sign_in.html")?.set_content_type(mime::TEXT_HTML_UTF_8))
}

/// Give user sign up page.
pub async fn sign_up_page(_req: HttpRequest) -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("static/sign_up.html")?.set_content_type(mime::TEXT_HTML_UTF_8))
}

/// Sign in user and return JWT on success.
pub async fn sign_in(
    app_state: web::Data<AppState>,
    form: web::Json<SignInForm>,
) -> Result<HttpResponse, Error> {
    let closure_app_state = app_state.clone();
    let use_cookies = form.cookie == Some(true);

    let user =
        web::block(move || database::users::authenticate_user(&form, closure_app_state))
            .await
            .map_err(|outer_err| match outer_err {
                BlockingError::Error(err) => match err.err_type {
                    DbErrorType::InternalServerError => {
                        ErrorInternalServerError(err.cause)
                    }
                    DbErrorType::BadRequest => ErrorBadRequest(err.cause),
                    DbErrorType::Unauthorized => ErrorUnauthorized(err.cause),
                },
                BlockingError::Canceled => {
                    ErrorInternalServerError("database request canceled")
                }
            })?;

    // get unix time stamp
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

    let res_token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&app_state.config.jwt.secret),
    );

    if let Ok(token) = res_token {
        if use_cookies {
            Ok(HttpResponse::Ok()
                .cookie(
                    http::Cookie::build("triox_jwt", token)
                        .domain(app_state.config.server.url.to_string())
                        .path("/")
                        .secure(true)
                        .http_only(true)
                        .finish(),
                )
                .body("Success: Cookie is set"))
        } else {
            Ok(HttpResponse::Ok().body(token))
        }
    } else {
        Err(ErrorInternalServerError("JWTs generation failed"))
    }
}

/// Sign up user by creating files and database entries.
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
            BlockingError::Canceled => {
                ErrorInternalServerError("database request canceled")
            }
        })?;

    Ok(HttpResponse::Ok().body("user created"))
}

pub async fn delete_user(
    app_state: web::Data<AppState>,
    form: web::Json<DeleteUserForm>,
) -> Result<HttpResponse, Error> {
    let _user =
        web::block(move || database::users::delete_user(&form, app_state))
            .await
            .map_err(|outer_err| match outer_err {
                BlockingError::Error(err) => match err.err_type {
                    DbErrorType::InternalServerError => {
                        ErrorInternalServerError(err.cause)
                    }
                    DbErrorType::BadRequest => ErrorBadRequest(err.cause),
                    DbErrorType::Unauthorized => ErrorUnauthorized(err.cause),
                },
                BlockingError::Canceled => {
                    ErrorInternalServerError("database request canceled")
                }
            })?;

    Ok(HttpResponse::Ok().body("user successfully deleted"))
}
