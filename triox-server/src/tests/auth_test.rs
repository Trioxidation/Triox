use crate::{app_conf, database, AppState};
use config::Config;
use dashmap::DashMap;

fn default_app_state() -> AppState {
    // Load configurations
    let config = Config::default();

    // gerate struct from config HashMap
    let config = app_conf::load_config(&config);

    // create database pool
    let db_pool = database::connect(&config.database.url())
        .expect("Failed to create database pool.");

    AppState {
        config,
        login_count: DashMap::new(),
        db_pool,
    }
}

#[cfg(test)]
mod sign_in {
    use crate::{auth, AppState};
    use actix_web::web;

    #[actix_rt::test]
    async fn unknown_user_name() {
        let app_state: web::Data<AppState> = web::Data::new(super::default_app_state());
        let form = web::Json(auth::SignInForm {
            user_name: "unknown user".to_owned(),
            password: "decent password".to_owned(),
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
            user_name: "decent user name".to_owned(),
            password: "1234".to_owned(),
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
            user_name: "decent user name".to_owned(),
            password: "01234567890123456789012345678901234567890".to_owned(),
        });

        let resp = auth::sign_in(app_state, form)
            .await
            .expect_err("request should throw an error");

        assert_eq!(resp.to_string(), "password too long");
    }

    #[actix_rt::test]
    async fn short_user_name() {
        let app_state: web::Data<AppState> = web::Data::new(super::default_app_state());
        let form = web::Json(auth::SignInForm {
            user_name: "user".to_owned(),
            password: "decent password".to_owned(),
        });

        let resp = auth::sign_in(app_state, form)
            .await
            .expect_err("request should throw an error");

        assert_eq!(resp.to_string(), "user name too short");
    }

    #[actix_rt::test]
    async fn long_user_name() {
        let app_state: web::Data<AppState> = web::Data::new(super::default_app_state());
        let form = web::Json(auth::SignInForm {
            user_name: "01234567890123456789012345678901234567890".to_owned(),
            password: "decent password".to_owned(),
        });

        let resp = auth::sign_in(app_state, form)
            .await
            .expect_err("request should throw an error");

        assert_eq!(resp.to_string(), "user name too long");
    }
}

#[cfg(test)]
mod sign_up {
    use crate::{auth, AppState};
    use actix_web::{http, web};

    #[actix_rt::test]
    async fn sign_up_and_sign_in() {
        let app_state: web::Data<AppState> = web::Data::new(super::default_app_state());

        let form = web::Json(auth::SignUpForm {
            user_name: "test user".to_owned(),
            password: "decent password".to_owned(),
            email: "admin@gmail.com".to_owned(),
        });

        let resp = auth::sign_up(app_state.clone(), form).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let form = web::Json(auth::SignInForm {
            user_name: "test user".to_owned(),
            password: "decent password".to_owned(),
        });

        let resp = auth::sign_in(app_state, form)
            .await
            .expect("request should not throw an error");

        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
