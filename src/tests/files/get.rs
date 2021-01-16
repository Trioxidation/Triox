use actix_web::web;
use actix_web::{test, App};
use actix_web::http::StatusCode;

use crate::app_state::AppState;
use crate::apps::files::get::get;
use crate::tests::util;

fn get_jwt(app_state: &web::Data<AppState>) -> String {
    use crate::jwt;
    use std::time::{SystemTime, UNIX_EPOCH};

    let time_now = SystemTime::now();
    let timestamp = time_now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize;

    let claims = jwt::Claims {
        sub: "someusername".to_owned(),
        id: 0,
        role: 1,
        exp: timestamp + 7200 // now + two hours
    };

    let res_token = jwt::encode_claims(&claims, &app_state.config.jwt.secret).unwrap();

    res_token
}

#[actix_rt::test]
async fn verification_fails() {
    let app_state: web::Data<AppState> = web::Data::new(util::default_app_state());

    let app = App::new()
    .app_data(app_state)
    .configure(|i: &mut web::ServiceConfig|{
        i.service(get);
    });

    let mut app = test::init_service(app).await;

    let request = test::TestRequest::get().uri("http://127.0.0.1:8080/app/files/get?path=fakefile.txt").header("Authorization", "Bearerinvalidjwt");

    let response = test::call_service(&mut app, request.to_request()).await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let response_body = test::read_body(response).await;
    assert_eq!(response_body, "Invalid token");
}

#[actix_rt::test]
async fn file_doesnt_exist() {
    let app_state: web::Data<AppState> = web::Data::new(util::default_app_state());
    let jwt = get_jwt(&app_state);

    let app = App::new()
    .app_data(app_state.clone())
    .configure(|i: &mut web::ServiceConfig|{
        i.service(get);
    });

    let mut app = test::init_service(app).await;

    let request = test::TestRequest::get().uri("http://127.0.0.1:8080/app/files/get?path=fakefile.txt")
        .header("Authorization", format!("Bearer{}", jwt));

    let response = test::call_service(&mut app, request.to_request()).await;
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

    let response_body = test::read_body(response).await;
    assert_eq!(response_body, "No such file or directory (os error 2)");
}

#[actix_rt::test]
async fn move_up_directory() {
    let app_state: web::Data<AppState> = web::Data::new(util::default_app_state());
    let jwt = get_jwt(&app_state);

    let app = App::new()
    .app_data(app_state.clone())
    .configure(|i: &mut web::ServiceConfig|{
        i.service(get);
    });

    let mut app = test::init_service(app).await;

    let request = test::TestRequest::get().uri("http://127.0.0.1:8080/app/files/get?path=../fakefile.txt")
        .header("Authorization", format!("Bearer{}", jwt));


    let response = test::call_service(&mut app, request.to_request()).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let response_body = test::read_body(response).await;
    assert_eq!(response_body, "Moving up directories is not allowed");
}

#[actix_rt::test]
async fn ok_case() {
    use tokio::fs;
    let app_state = util::default_app_state();
    let user = util::test_user(app_state.clone());
    fs::File::create(format!("./data/users/{}/files/test_file", user.id)).await.unwrap();

    let app_state: web::Data<AppState> = web::Data::new(app_state);

    let app = App::new()
    .app_data(app_state.clone())
    .configure(|i: &mut web::ServiceConfig|{
        i.service(get);
    });

    let mut app = test::init_service(app).await;

    let request = test::TestRequest::get().uri("http://127.0.0.1:8080/app/files/get?path=test_file")
        .header("Authorization", format!("Bearer{}", user.jwt));


    let response = test::call_service(&mut app, request.to_request()).await;
    assert_eq!(response.status(), StatusCode::OK);
}
