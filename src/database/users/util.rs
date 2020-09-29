use super::{DbError, DbErrorType};
use check_if_email_exists::{check_email, CheckEmailInput, Reachable};
use tokio::runtime::Runtime;

pub fn validate_credentials(user_name: &str, password: &str) -> Result<(), DbError> {
    // Check whether username and password have a reasonable length
    if user_name.len() > 40 {
        Err(DbError {
            err_type: DbErrorType::BadRequest,
            cause: "user name too long".to_owned(),
        })
    } else if user_name.len() < 5 {
        Err(DbError {
            err_type: DbErrorType::BadRequest,
            cause: "user name too short".to_owned(),
        })
    } else if password.len() > 32 {
        Err(DbError {
            err_type: DbErrorType::BadRequest,
            cause: "password too long".to_owned(),
        })
    } else if password.len() < 8 {
        Err(DbError {
            err_type: DbErrorType::BadRequest,
            cause: "password too short".to_owned(),
        })
    } else {
        // Nothing bad found, return Ok
        Ok(())
    }
}

pub fn validate_email(email: &str) -> Result<(), DbError> {
    if email.len() > 40 {
        Err(DbError {
            err_type: DbErrorType::BadRequest,
            cause: "email address too long".to_owned(),
        })
    } else if email.len() < 5 {
        Err(DbError {
            err_type: DbErrorType::BadRequest,
            cause: "email address too short".to_owned(),
        })
    } else {
        let input = CheckEmailInput::new(vec![email.into()]);

        // Create a tokio runtime to run async block
        let mut rt = Runtime::new().expect("Couln't create tokio Runtime");

        // Verify email address
        let result = rt.block_on(async { check_email(&input).await });
        let result = &result[0];

        // Email address should be at leat risky to be accepted.
        // Risky is ok because users need to verify themselves via
        // email anyway
        match result.is_reachable {
            Reachable::Safe | Reachable::Risky | Reachable::Unknown => Ok(()),
            _ => Err(DbError {
                err_type: DbErrorType::BadRequest,
                cause: "Couldn't verify email reachability".to_owned(),
            }),
        }
    }
}
