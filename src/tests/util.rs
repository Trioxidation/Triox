use crate::app_state::AppState;

pub fn default_app_state() -> AppState {
    // Tests expect the config to be placed in the "config" directory
    crate::app_state::AppState::new("config")
}

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn random_string(len: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

use crate::auth;

pub fn random_creds() -> auth::SignUpForm {
    auth::SignUpForm {
        username: random_string(10),
        password: random_string(30),
        email: format!("{}@test.com", random_string(10)),
    }
}

use crate::jwt;
use actix_web::web;

/// Test user used for test that require authentication
/// Automatically generates valid JWT and removes user from database when dropped
pub struct TestUser {
    pub creds: auth::SignUpForm,
    pub jwt: String,
    pub app_state: AppState,
    pub id: u32,
}

#[test]
fn test_test_user() {
    let _user = test_user(default_app_state());
}

/// Return test user
/// Can be used to test services that require authentication
/// User will be automatically deleted when the user variable is dropped
pub fn test_user(app_state: AppState) -> TestUser {
    // generate random credentials
    let creds = random_creds();

    // create new user
    let user =
        crate::database::users::add_user(&creds, web::Data::new(app_state.clone()))
            .expect("Couldn't create user");

    use std::time::{SystemTime, UNIX_EPOCH};

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

    TestUser {
        creds: creds.clone(),
        jwt: jwt::encode_claims(&claims, &app_state.config.server.secret)
            .expect("JWT encoding failed"),
        app_state,
        id: user.id,
    }
}

impl Drop for TestUser {
    fn drop(&mut self) {
        let form = auth::DeleteUserForm {
            username: self.creds.username.clone(),
            password: self.creds.password.clone(),
        };

        crate::database::users::delete_user(
            &form,
            web::Data::new(self.app_state.clone()),
        )
        .expect("Couldn't delete user");
    }
}
