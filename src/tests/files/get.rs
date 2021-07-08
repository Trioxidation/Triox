use actix_web::http::StatusCode;
use actix_web::test;
use tokio::fs;

use crate::app_state::AppState;
use crate::tests::*;
use crate::*;

#[actix_rt::test]
async fn file_works() {
    const NAME: &str = "fileuser";
    const PASSWORD: &str = "randompassword";

    {
        let data = AppState::new().await;
        delete_user(NAME, &data).await;
    }

    let (data, _, signin_resp) = register_and_signin(NAME, None, PASSWORD).await;
    let cookies = get_cookie!(signin_resp);
    let app = get_app!(data).await;

    let response = test::call_service(
        &app,
        get_req!("/app/files/get?path=fakefile.txt")
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // move up a dir
    let response = test::call_service(
        &app,
        get_req!("/app/files/get?path=../fakefile.txt")
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let file_name = format!("./data/users/{}/files/test_file", NAME);
    fs::File::create(&file_name).await.unwrap();
    let response = test::call_service(
        &app,
        get_req!("/app/files/get?path=test_file")
            .cookie(cookies)
            .to_request(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    fs::remove_file(file_name).await.unwrap();
}
