use super::{DbError, DbErrorType};

pub fn validate_credentials(username: &str, password: &str) -> Result<(), DbError> {
    // Check whether username and password have a reasonable length
    if username.len() > 40 {
        Err(DbError {
            err_type: DbErrorType::BadRequest,
            cause: "Username is too long".to_owned(),
        })
    } else if username.len() < 5 {
        Err(DbError {
            err_type: DbErrorType::BadRequest,
            cause: "Username is too short".to_owned(),
        })
    } else if password.len() > 32 {
        Err(DbError {
            err_type: DbErrorType::BadRequest,
            cause: "Password is too long".to_owned(),
        })
    } else if password.len() < 8 {
        Err(DbError {
            err_type: DbErrorType::BadRequest,
            cause: "Password is too short".to_owned(),
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
            cause: "Email address is too long".to_owned(),
        })
    } else if email.len() < 5 {
        Err(DbError {
            err_type: DbErrorType::BadRequest,
            cause: "Email address is too short".to_owned(),
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
            cause: "Incorrect email address".to_owned(),
        })
    } else {
        Ok(())
    }
}
