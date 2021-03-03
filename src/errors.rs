use std::io::{Error as IOError, ErrorKind as IOErrorKind};

use actix_multipart::MultipartError;
use actix_web::{
    dev::HttpResponseBuilder,
    error::ResponseError,
    http::{header, StatusCode},
    HttpResponse,
};

use derive_more::{Display, Error};
use jsonwebtoken::errors::{Error as JwtError, ErrorKind as JwtErrorKind};
use serde::Serialize;
// use validator::ValidationErrors;

use std::convert::From;

#[derive(Debug, Display, Clone, PartialEq, Error)]
#[cfg(not(tarpaulin_include))]
pub enum ServiceError {
    #[display(fmt = "internal server error")]
    InternalServerError,
    #[display(fmt = "The value you entered for email is not an email")] //405j
    NotAnEmail,
    #[display(fmt = "Bad request")]
    BadRequest,
    #[display(fmt = "Unknown MIME type")]
    UnknownMIME,
    #[display(fmt = "Expired token")]
    TokenExpired,
    #[display(fmt = "Invalid token")]
    InvalidToken,
    #[display(fmt = "File not found")]
    FileNotFouond,
    #[display(fmt = "File exists")]
    FileExists,
    #[display(fmt = "Permission denied")]
    PermissionDenied,
    #[display(fmt = "Server in readonly mode")]
    FSReadOnly,
    #[display(fmt = "Invalid credentials")]
    InvalidCredentials,
}

#[derive(Serialize)]
#[cfg(not(tarpaulin_include))]
struct ErrorToResponse {
    error: String,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json; charset=UTF-8")
            .json(ErrorToResponse {
                error: self.to_string(),
            })
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::BadRequest => StatusCode::BAD_REQUEST,
            ServiceError::NotAnEmail => StatusCode::BAD_REQUEST,
            ServiceError::UnknownMIME => StatusCode::BAD_REQUEST,
            ServiceError::TokenExpired => StatusCode::UNAUTHORIZED,
            ServiceError::InvalidToken => StatusCode::UNAUTHORIZED,
            ServiceError::FileNotFouond => StatusCode::NOT_FOUND,
            ServiceError::FileExists => StatusCode::METHOD_NOT_ALLOWED,
            ServiceError::PermissionDenied => StatusCode::UNAUTHORIZED,
            ServiceError::FSReadOnly => StatusCode::METHOD_NOT_ALLOWED,
            ServiceError::InvalidCredentials => StatusCode::UNAUTHORIZED,
        }
    }
}

impl From<JwtError> for ServiceError {
    fn from(e: JwtError) -> ServiceError {
        match e.kind() {
            JwtErrorKind::ExpiredSignature => ServiceError::TokenExpired,
            JwtErrorKind::InvalidSignature => ServiceError::InvalidToken,
            _ => ServiceError::InternalServerError,
        }
    }
}

impl From<IOError> for ServiceError {
    fn from(e: IOError) -> ServiceError {
        match e.kind() {
            IOErrorKind::NotFound => ServiceError::FileNotFouond,
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

// impl From<ValidationErrors> for ServiceError {
//     fn from(_: ValidationErrors) -> ServiceError {
//         ServiceError::NotAnEmail
//     }
// }
//
// impl From<sqlx::Error> for ServiceError {
//     fn from(e: sqlx::Error) -> Self {
//         use sqlx::error::Error;
//         use std::borrow::Cow;
//         if let Error::Database(err) = e {
//             if err.code() == Some(Cow::from("23505")) {
//                 return ServiceError::DuplicateResponse;
//             }
//         }
//
//         ServiceError::InternalServerError
//     }
// }

pub type ServiceResult<V> = std::result::Result<V, ServiceError>;
