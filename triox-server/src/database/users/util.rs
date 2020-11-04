use super::{DbError, DbErrorType};

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
    // Expected patter: semething@something.something
    } else if !email.contains('@')
        || !email.contains('.')
        || email.contains("..")
        || email.find('@') != email.rfind('@')
        || email.find('@').unwrap() > email.rfind('.').unwrap()
    {
        Err(DbError {
            err_type: DbErrorType::BadRequest,
            cause: "badly formatted email address".to_owned(),
        })
    } else {
        Ok(())
    }
}
