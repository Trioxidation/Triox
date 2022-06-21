use actix_web::http::StatusCode;
use actix_web::test;
use tokio::fs;

use crate::app_state::AppState;
use crate::apps::files::list::ListResponse;
use crate::apps::files::SourceAndDest;
use crate::tests::*;
use crate::*;

#[actix_rt::test]
async fn file_works() {
    const NAME: &str = "fileuser";
    const PASSWORD: &str = "randompassword";
    const CONTENT: &str = "filecontent";
    const FILE_NAME: &str = "test_file";
    const DIR_NAME: &str = "test_dir";
    let file_name2 = format!("{}/{}", DIR_NAME, FILE_NAME);
    const FILE_NAME3: &str = "test_file2";
    const MOVE_FILE: &str = "moved";

    {
        let data = AppState::new().await;
        delete_user(NAME, &data).await;
    }

    let (data, _, signin_resp) = register_and_signin(NAME, None, PASSWORD).await;
    let cookies = get_cookie!(signin_resp);
    let app = get_app!(data).await;

    let response = test::call_service(
        &app,
        get_req!(&path(FILE_ROUTES.get, "fakefile.txt"))
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // upload file

    let file_name = format!("./data/users/{}/files/test_file", NAME);
    fs::File::create(&file_name).await.unwrap();
    fs::write(&file_name, CONTENT).await.unwrap();

    // move up a dir
    let response = test::call_service(
        &app,
        get_req!("/app/files/get?path=../fakefile.txt")
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let response = test::call_service(
        &app,
        get_req!(&path(FILE_ROUTES.get, FILE_NAME))
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    let content = String::from_utf8(test::read_body(response).await.to_vec()).unwrap();
    assert_eq!(CONTENT, content);

    // create dir
    let response = test::call_service(
        &app,
        get_req!(&path(FILE_ROUTES.create_dir, DIR_NAME))
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);

    // copy file
    let mut payload = SourceAndDest {
        from: FILE_NAME.into(),
        to: file_name2.clone(),
    };

    let response = test::call_service(
        &app,
        post_request!(&payload, FILE_ROUTES.copy)
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);

    payload.to = FILE_NAME3.into();
    let response = test::call_service(
        &app,
        post_request!(&payload, FILE_ROUTES.copy)
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);

    // move file
    let payload = SourceAndDest {
        from: FILE_NAME3.into(),
        to: MOVE_FILE.into(),
    };

    let response = test::call_service(
        &app,
        post_request!(&payload, FILE_ROUTES.mv)
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);

    // list contents
    let response = test::call_service(
        &app,
        get_req!(&path(FILE_ROUTES.list, ""))
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    let body: ListResponse = test::read_body_json(response).await;
    assert!(body.files.iter().any(|file| file.name == FILE_NAME));
    assert!(body.files.iter().any(|file| file.name == MOVE_FILE));
    assert!(body.directories.iter().any(|dir| dir.name == DIR_NAME));

    let response = test::call_service(
        &app,
        get_req!(&path(FILE_ROUTES.list, DIR_NAME))
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    let body: ListResponse = test::read_body_json(response).await;
    assert!(body.files.iter().any(|file| file.name == FILE_NAME));

    // remove and list files
    let response = test::call_service(
        &app,
        get_req!(&path(FILE_ROUTES.remove, FILE_NAME))
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    let response = test::call_service(
        &app,
        get_req!(&path(FILE_ROUTES.list, ""))
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    let body: ListResponse = test::read_body_json(response).await;
    assert!(!body.files.iter().any(|file| file.name == FILE_NAME));
}
