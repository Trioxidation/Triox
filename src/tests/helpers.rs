use std::sync::Arc;

use actix_web::{body, test};
use actix_web::{dev::ServiceResponse, http::StatusCode};
use serde::{Deserialize, Serialize};

use crate::api::v1::auth::runners::{Login, Register};
use crate::api::v1::ROUTES;
use crate::app_state::AppState;
use crate::errors::*;

#[derive(Serialize, Deserialize)]
pub struct ErrorToResponse {
    pub error: String,
}

#[macro_export]
macro_rules! get_cookie {
    ($resp:expr) => {
        $resp.response().cookies().next().unwrap().to_owned()
    };
}

pub async fn delete_user(name: &str, data: &AppState) {
    let r = sqlx::query!("DELETE FROM triox_users WHERE name = ($1)", name,)
        .execute(&data.db)
        .await;

    // delete storage path of the user
    let path: std::path::PathBuf = [".", "data", "users", name].iter().collect();

    let _ = tokio::fs::remove_dir_all(path);
    println!();
    println!();
    println!();
    println!("Deleting user: {:?}", &r);
}

#[macro_export]
macro_rules! post_request {
    ($uri:expr) => {
        test::TestRequest::post().uri($uri)
    };

    ($serializable:expr, $uri:expr) => {
        test::TestRequest::post()
            .uri($uri)
            .insert_header((actix_web::http::header::CONTENT_TYPE, "application/json"))
            .set_payload(serde_json::to_string($serializable).unwrap())
    };
}

#[macro_export]
macro_rules! get_req {
    ($route:expr) => {
        test::TestRequest::get().uri($route)
    };
}

#[macro_export]
macro_rules! get_works {
    ($app:expr,$route:expr) => {
        test::call_service(
            &mut $app,
        get_req!($route).to_request().await;
        )

        assert_eq!(get_resp.status(), StatusCode::OK);
    };
}

#[macro_export]
macro_rules! get_app {
    () => {
        test::init_service(
            App::new()
                .wrap(get_identity_service())
                .wrap(actix_middleware::NormalizePath::new(
                    actix_middleware::TrailingSlash::Trim,
                ))
                .configure(crate::api::v1::services)
                .configure(crate::app::files::services),
        )
    };
    ($data:expr) => {
        test::init_service(get_app!($data, "app"))
    };

    ($data:expr, "app") => {
        actix_web::App::new()
            .wrap(crate::get_identity_service())
            .wrap(actix_web::middleware::NormalizePath::new(
                actix_web::middleware::TrailingSlash::Trim,
            ))
            .configure(crate::apps::files::services)
            .configure(crate::api::v1::services)
            .app_data(actix_web::web::Data::new($data.clone()))
    };
}

/// register and signin utility
pub async fn register_and_signin(
    name: &str,
    email: Option<String>,
    password: &str,
) -> (
    Arc<AppState>,
    Login,
    ServiceResponse<body::EitherBody<body::BoxBody>>,
) {
    register(name, email, password).await;
    signin(name, password).await
}

/// register utility
pub async fn register(name: &str, email: Option<String>, password: &str) {
    let data = AppState::new().await;
    let app = get_app!(data).await;

    // 1. Register
    let msg = Register {
        username: name.into(),
        password: password.into(),
        confirm_password: password.into(),
        email,
    };
    let resp =
        test::call_service(&app, post_request!(&msg, ROUTES.auth.register).to_request())
            .await;
    assert_eq!(resp.status(), StatusCode::OK);
}

/// signin util
pub async fn signin(
    name: &str,
    password: &str,
) -> (
    Arc<AppState>,
    Login,
    ServiceResponse<body::EitherBody<body::BoxBody>>,
) {
    let data = AppState::new().await;
    let app = get_app!(data.clone()).await;

    // 2. signin
    let creds = Login {
        login: name.into(),
        password: password.into(),
    };
    let signin_resp =
        test::call_service(&app, post_request!(&creds, ROUTES.auth.login).to_request())
            .await;
    assert_eq!(signin_resp.status(), StatusCode::OK);
    (data, creds, signin_resp)
}

/// pub duplicate test
pub async fn bad_post_req_test<T: Serialize>(
    name: &str,
    password: &str,
    url: &str,
    payload: &T,
    dup_err: ServiceError,
    s: StatusCode,
) {
    let (data, _, signin_resp) = signin(name, password).await;
    let cookies = get_cookie!(signin_resp);
    let app = get_app!(data).await;

    let dup_token_resp = test::call_service(
        &app,
        post_request!(&payload, url)
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(dup_token_resp.status(), s);
    let txt: ErrorToResponse = test::read_body_json(dup_token_resp).await;
    assert_eq!(txt.error, format!("{}", dup_err));
}

pub fn path(route: &str, param: &str) -> String {
    let param = param.trim();
    if route.ends_with('/') {
        format!("{}?path={}", &route[0..route.len() - 1], param)
    } else {
        format!("{}?path={}", route, param)
    }
}
