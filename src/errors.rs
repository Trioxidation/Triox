use actix_web::{
    dev::HttpResponseBuilder,
    error::{Error as ActixWebError, ResponseError},
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
    #[display(fmt = "Response Doesn't exist")]
    BadRequest,
    #[display(fmt = "Expired token")]
    TokenExpired,
    #[display(fmt = "Invalid token")]
    InvalidToken,
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
            ServiceError::TokenExpired => StatusCode::UNAUTHORIZED,
            ServiceError::InvalidToken => StatusCode::UNAUTHORIZED,
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

// impl From<ActixWebError> for ServiceError {
//     fn from(_: ActixWebError) -> ServiceError {
//         ServiceError::InternalServerError
//     }
// }

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
