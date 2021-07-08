use actix_web::HttpResponse;
use actix_web::Responder;

const SIGN_IN: &str = include_str!("../static/sign_in.html");
const SIGN_UP: &str = include_str!("../static/sign_up.html");

/// Give user sign in page.
pub async fn sign_in_page() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(SIGN_IN)
}

/// Give user sign up page.
pub async fn sign_up_page() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(SIGN_UP)
}
