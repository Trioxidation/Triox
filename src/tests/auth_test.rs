#[cfg(test)]
mod sign_in {
    use crate::{app_state::AppState, auth, tests};
    use actix_web::web;

    #[actix_rt::test]
    async fn unknown_username() {
        let app_state: web::Data<AppState> =
            web::Data::new(tests::util::default_app_state());
        let form = web::Json(auth::SignInForm {
            username: "unknown user".to_owned(),
            password: "decent password".to_owned(),
            cookie: None,
        });

        let resp = auth::sign_in(app_state, form)
            .await
            .expect_err("request should throw an error");

        assert_eq!(resp.to_string(), "Username doesn't exist");
    }

    #[actix_rt::test]
    async fn short_password() {
        let app_state: web::Data<AppState> =
            web::Data::new(tests::util::default_app_state());
        let form = web::Json(auth::SignInForm {
            username: "decent user name".to_owned(),
            password: "1234".to_owned(),
            cookie: None,
        });

        let resp = auth::sign_in(app_state, form)
            .await
            .expect_err("request should throw an error");

        assert_eq!(resp.to_string(), "Password is too short");
    }

    #[actix_rt::test]
    async fn long_password() {
        let app_state: web::Data<AppState> =
            web::Data::new(tests::util::default_app_state());
        let form = web::Json(auth::SignInForm {
            username: "decent user name".to_owned(),
            password: "01234567890123456789012345678901234567890".to_owned(),
            cookie: None,
        });

        let resp = auth::sign_in(app_state, form)
            .await
            .expect_err("request should throw an error");

        assert_eq!(resp.to_string(), "Password is too long");
    }

    #[actix_rt::test]
    async fn short_username() {
        let app_state: web::Data<AppState> =
            web::Data::new(tests::util::default_app_state());
        let form = web::Json(auth::SignInForm {
            username: "user".to_owned(),
            password: "decent password".to_owned(),
            cookie: None,
        });

        let resp = auth::sign_in(app_state, form)
            .await
            .expect_err("request should throw an error");

        assert_eq!(resp.to_string(), "Username is too short");
    }

    #[actix_rt::test]
    async fn long_username() {
        let app_state: web::Data<AppState> =
            web::Data::new(tests::util::default_app_state());
        let form = web::Json(auth::SignInForm {
            username: "01234567890123456789012345678901234567890".to_owned(),
            password: "decent password".to_owned(),
            cookie: None,
        });

        let resp = auth::sign_in(app_state, form)
            .await
            .expect_err("request should throw an error");

        assert_eq!(resp.to_string(), "Username is too long");
    }
}

#[cfg(test)]
mod sign_up {
    use crate::{app_state::AppState, auth, tests};
    use actix_web::{http, web};

    #[actix_rt::test]
    async fn sign_up_and_sign_in() {
        // generating and loading data
        let app_state: web::Data<AppState> =
            web::Data::new(tests::util::default_app_state());

        let creds = tests::util::random_creds();

        // sign up
        let form = web::Json(creds.clone());

        let resp = auth::sign_up(app_state.clone(), form).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        // sign in
        let form = web::Json(auth::SignInForm {
            username: creds.username.clone(),
            password: creds.password.clone(),
            cookie: None,
        });

        let resp = auth::sign_in(app_state.clone(), form)
            .await
            .expect("request should not throw an error");

        assert_eq!(resp.status(), http::StatusCode::OK);

        // delete user
        let form = web::Json(auth::DeleteUserForm {
            username: creds.username.clone(),
            password: creds.password.clone(),
        });

        let resp = auth::delete_user(app_state.clone(), form)
            .await
            .expect("request should not throw an error");

        assert_eq!(resp.status(), http::StatusCode::OK);

        // sign in again (should fail)
        let form = web::Json(auth::SignInForm {
            username: creds.username.clone(),
            password: creds.password.clone(),
            cookie: None,
        });

        let resp = auth::sign_in(app_state.clone(), form)
            .await
            .expect_err("request should throw an error");

        assert_eq!(
            resp.to_string(),
            actix_web::error::ErrorUnauthorized("Username doesn't exist").to_string()
        );
    }
}
