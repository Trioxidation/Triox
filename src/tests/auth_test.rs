use crate::app_state::AppState;

fn default_app_state() -> AppState {
    // Tests expect the config to be placed in the "config" directory
    crate::app_state::load_app_state("config")
}

#[cfg(test)]
mod sign_in {
    use crate::{app_state::AppState, auth};
    use actix_web::web;

    #[actix_rt::test]
    async fn unknown_username() {
        let app_state: web::Data<AppState> = web::Data::new(super::default_app_state());
        let form = web::Json(auth::SignInForm {
            username: "unknown user".to_owned(),
            password: "decent password".to_owned(),
            cookie: None,
        });

        let resp = auth::sign_in(app_state, form)
            .await
            .expect_err("request should throw an error");

        assert_eq!(resp.to_string(), "unable to locate user");
    }

    #[actix_rt::test]
    async fn short_password() {
        let app_state: web::Data<AppState> = web::Data::new(super::default_app_state());
        let form = web::Json(auth::SignInForm {
            username: "decent user name".to_owned(),
            password: "1234".to_owned(),
            cookie: None,
        });

        let resp = auth::sign_in(app_state, form)
            .await
            .expect_err("request should throw an error");

        assert_eq!(resp.to_string(), "password too short");
    }

    #[actix_rt::test]
    async fn long_password() {
        let app_state: web::Data<AppState> = web::Data::new(super::default_app_state());
        let form = web::Json(auth::SignInForm {
            username: "decent user name".to_owned(),
            password: "01234567890123456789012345678901234567890".to_owned(),
            cookie: None,
        });

        let resp = auth::sign_in(app_state, form)
            .await
            .expect_err("request should throw an error");

        assert_eq!(resp.to_string(), "password too long");
    }

    #[actix_rt::test]
    async fn short_username() {
        let app_state: web::Data<AppState> = web::Data::new(super::default_app_state());
        let form = web::Json(auth::SignInForm {
            username: "user".to_owned(),
            password: "decent password".to_owned(),
            cookie: None,
        });

        let resp = auth::sign_in(app_state, form)
            .await
            .expect_err("request should throw an error");

        assert_eq!(resp.to_string(), "user name too short");
    }

    #[actix_rt::test]
    async fn long_username() {
        let app_state: web::Data<AppState> = web::Data::new(super::default_app_state());
        let form = web::Json(auth::SignInForm {
            username: "01234567890123456789012345678901234567890".to_owned(),
            password: "decent password".to_owned(),
            cookie: None,
        });

        let resp = auth::sign_in(app_state, form)
            .await
            .expect_err("request should throw an error");

        assert_eq!(resp.to_string(), "user name too long");
    }
}

#[cfg(test)]
mod sign_up {
    use crate::{app_state::AppState, auth};
    use actix_web::{http, web};
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    #[actix_rt::test]
    async fn sign_up_and_sign_in() {
        // generating and loading data
        let app_state: web::Data<AppState> = web::Data::new(super::default_app_state());

        let username: String =
            thread_rng().sample_iter(&Alphanumeric).take(10).collect();

        let password: String =
            thread_rng().sample_iter(&Alphanumeric).take(30).collect();

        let mut email: String =
            thread_rng().sample_iter(&Alphanumeric).take(10).collect();

        email.push_str("@test.com");

        // sign up
        let form = web::Json(auth::SignUpForm {
            username: username.clone(),
            password: password.clone(),
            email,
        });

        let resp = auth::sign_up(app_state.clone(), form).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        // sign in
        let form = web::Json(auth::SignInForm {
            username: username.clone(),
            password: password.clone(),
            cookie: None,
        });

        let resp = auth::sign_in(app_state.clone(), form)
            .await
            .expect("request should not throw an error");

        assert_eq!(resp.status(), http::StatusCode::OK);

        // delete user
        let form = web::Json(auth::DeleteUserForm {
            username: username.clone(),
            password: password.clone(),
        });

        let resp = auth::delete_user(app_state.clone(), form)
            .await
            .expect("request should not throw an error");

        assert_eq!(resp.status(), http::StatusCode::OK);

        // sign in again (should fail)
        let form = web::Json(auth::SignInForm {
            username: username.clone(),
            password: password.clone(),
            cookie: None,
        });

        let resp = auth::sign_in(app_state.clone(), form)
            .await
            .expect_err("request should throw an error");

        assert_eq!(
            resp.to_string(),
            actix_web::error::ErrorUnauthorized("unable to locate user").to_string()
        );
    }
}
