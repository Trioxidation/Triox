pub mod util;

use crate::AppState;

use actix_web::web;

use crate::database::schema::users::dsl::*;
use diesel::prelude::*;

use crate::auth::{SignInForm, SignUpForm};
use crate::hash;

#[derive(diesel::Queryable, serde::Serialize, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub language_code: Option<String>,
    pub role: u8,
    pub status: Option<String>,
}

#[derive(Debug)]
pub enum DbErrorType {
    BadRequest,
    Unauthorized,
    InternalServerError,
}

/* I'd prefer to use the actix::Error type instead.
 * But sadly actix::Error doesn't implement std::marker::Sync
 * so I can't use it in a thread pool any more.
 * Therefore I created this *ugly* additional Error type
 * and need to use .map_err all the time...*/
#[derive(Debug)]
pub struct DbError {
    pub err_type: DbErrorType,
    pub cause: String,
}

pub fn authenticate_user(
    data: &SignInForm,
    app_state: web::Data<AppState>,
) -> Result<User, DbError> {
    let db_conn = app_state.db_pool.get().map_err(|_| DbError {
        err_type: DbErrorType::InternalServerError,
        cause: "couldn't connect to database".to_owned(),
    })?;

    // Check whether username and password have a reasonable length
    if let Err(error) = util::validate_credentials(&data.user_name, &data.password) {
        return Err(error);
    }

    // Load user from database
    let user = users
        .filter(NAME.eq(&data.user_name))
        .first::<User>(&db_conn)
        .map_err(|_| DbError {
            err_type: DbErrorType::Unauthorized,
            cause: "unable to locate user".to_owned(),
        })?;

    // Return error if login count is above 4
    if let Some(count) = app_state.login_count.get(&user.id) {
        if *count <= 4 {
            return Err(DbError {
                err_type: DbErrorType::Unauthorized,
                cause: "you have too many active sessions".to_owned(),
            });
        }
    };

    // Check passworrd
    if !crate::hash::compare_passwords(&data.password.as_bytes(), &user.password_hash).map_err(
        |err| DbError {
            err_type: DbErrorType::InternalServerError,
            cause: err.to_owned(),
        },
    )? {
        return Err(DbError {
            err_type: DbErrorType::Unauthorized,
            cause: "wrong password".to_owned(),
        });
    }

    Ok(user)
}

pub fn add_user(data: &SignUpForm, app_state: web::Data<AppState>) -> Result<User, DbError> {
    let db_conn = app_state.db_pool.get().map_err(|_| DbError {
        err_type: DbErrorType::InternalServerError,
        cause: "couldn't connect to database".to_owned(),
    })?;

    // Check whether username and password have a reasonable length
    // and whether the email address is available
    if let Err(error) = util::validate_credentials(&data.user_name, &data.password) {
        return Err(error);
    }

    if let Err(error) = util::validate_email(&data.email) {
        return Err(error);
    }

    // Check whether user name already exists in database
    if users
        .select(ID)
        .filter(NAME.eq(&data.user_name))
        .first::<u32>(&db_conn)
        .is_ok()
    {
        return Err(DbError {
            err_type: DbErrorType::BadRequest,
            cause: "user name is already taken!".to_owned(),
        });
    }

    // Check whether email address already exists in database
    if users
        .select(ID)
        .filter(EMAIL.eq(&data.email))
        .first::<u32>(&db_conn)
        .is_ok()
    {
        return Err(DbError {
            err_type: DbErrorType::BadRequest,
            cause: "email address is already taken!".to_owned(),
        });
    }

    // Find a new, random user id
    let id: u32 = loop {
        let random_id: u32 = rand::random();
        if users
            .select(ID)
            .filter(ID.eq(&random_id))
            .first::<u32>(&db_conn)
            .is_err()
        {
            break random_id;
        }
    };

    // Generate Hash
    let hash = hash::from_password(&data.password).map_err(|err| DbError {
        err_type: DbErrorType::InternalServerError,
        cause: err.to_owned(),
    })?;

    // Create new user in the database
    diesel::insert_into(users)
        .values((
            ID.eq(id),
            NAME.eq(&data.user_name),
            PASSWORD_HASH.eq(hash),
            EMAIL.eq(&data.email),
            ROLE.eq(0),
        ))
        .execute(&db_conn)
        .map_err(|err| {
            error!("DATABASE: {:?}", err);
            DbError {
                err_type: DbErrorType::BadRequest,
                cause: err.to_string(),
            }
        })?;

    // Check whether new user can be located
    let user = users
        .filter(NAME.eq(&data.user_name))
        .first::<User>(&db_conn)
        .map_err(|err| {
            error!("DATABASE: {:?}", err);
            DbError {
                err_type: DbErrorType::BadRequest,
                cause: "Unable to receive user".to_owned(),
            }
        })?;

    Ok(user)
}
