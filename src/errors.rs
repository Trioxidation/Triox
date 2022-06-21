use std::convert::From;
use std::io::{Error as IOError, ErrorKind as IOErrorKind};

use actix_multipart::MultipartError;
use actix_web::{
    dev::ServiceResponse,
    error::ResponseError,
    http::{self, header, StatusCode},
    middleware::ErrorHandlerResponse,
    HttpResponse, HttpResponseBuilder, Result,
};
use argon2_creds::errors::CredsError;
use derive_more::{Display, Error};
use serde::Serialize;

#[derive(Debug, Display, Clone, PartialEq, Error)]
#[cfg(not(tarpaulin_include))]
pub enum ServiceError {
    #[display(fmt = "internal server error")]
    InternalServerError,
    #[display(fmt = "The value you entered for email is not an email")] //405j
    NotAnEmail,
    #[display(fmt = "Bad request")]
    BadRequest,
    #[display(fmt = "File not found")]
    FileNotFound,
    #[display(fmt = "File exists")]
    FileExists,
    #[display(fmt = "Permission denied")]
    PermissionDenied,
    #[display(fmt = "Server in readonly mode")]
    FSReadOnly,
    #[display(fmt = "Invalid credentials")]
    InvalidCredentials,
    #[display(fmt = "{}", _0)]
    CredentialError(CredsError),
    /// when the a username is already taken
    #[display(fmt = "Username not available")]
    UsernameTaken,
    #[display(fmt = "Email not available")]
    EmailTaken,

    #[display(fmt = "Account not found")]
    AccountNotFound,
    #[display(fmt = "Passwords don't match")]
    PasswordsDontMatch,
}

#[derive(Serialize)]
#[cfg(not(tarpaulin_include))]
struct ErrorToResponse {
    error: String,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .append_header((header::CONTENT_TYPE, "application/json; charset=UTF-8"))
            .body(
                serde_json::to_string(&ErrorToResponse {
                    error: self.to_string(),
                })
                .unwrap(),
            )
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::AccountNotFound => StatusCode::NOT_FOUND,
            ServiceError::BadRequest => StatusCode::BAD_REQUEST,
            ServiceError::NotAnEmail => StatusCode::BAD_REQUEST,
            ServiceError::PasswordsDontMatch => StatusCode::BAD_REQUEST,
            ServiceError::UsernameTaken => StatusCode::BAD_REQUEST,
            ServiceError::EmailTaken => StatusCode::BAD_REQUEST,
            ServiceError::FileNotFound => StatusCode::NOT_FOUND,
            ServiceError::FileExists => StatusCode::METHOD_NOT_ALLOWED,
            ServiceError::PermissionDenied => StatusCode::UNAUTHORIZED,
            ServiceError::FSReadOnly => StatusCode::METHOD_NOT_ALLOWED,
            ServiceError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            ServiceError::CredentialError(_e) => StatusCode::BAD_REQUEST,
        }
    }
}

impl From<IOError> for ServiceError {
    fn from(e: IOError) -> ServiceError {
        match e.kind() {
            IOErrorKind::NotFound => ServiceError::FileNotFound,
            IOErrorKind::PermissionDenied => ServiceError::PermissionDenied,
            IOErrorKind::AlreadyExists => ServiceError::FileExists,
            _ => ServiceError::InternalServerError,
        }
    }
}

impl From<MultipartError> for ServiceError {
    fn from(_: MultipartError) -> ServiceError {
        ServiceError::InternalServerError
    }
}

impl From<CredsError> for ServiceError {
    #[cfg(not(tarpaulin_include))]
    fn from(e: CredsError) -> ServiceError {
        match e {
            CredsError::NotAnEmail => ServiceError::NotAnEmail,
            CredsError::Argon2Error(_) => ServiceError::InternalServerError,
            _ => ServiceError::CredentialError(e),
        }
    }
}

#[cfg(not(tarpaulin_include))]
impl From<sqlx::Error> for ServiceError {
    #[cfg(not(tarpaulin_include))]
    fn from(e: sqlx::Error) -> Self {
        use sqlx::error::Error;
        use std::borrow::Cow;
        if let Error::Database(err) = e {
            if err.code() == Some(Cow::from("23505")) {
                return ServiceError::UsernameTaken;
            }
        }
        ServiceError::InternalServerError
    }
}

pub fn render_404<B>(mut res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::header::HeaderValue::from_static("Error"),
    );
    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}

pub type ServiceResult<V> = std::result::Result<V, ServiceError>;
