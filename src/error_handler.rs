use actix_web::body::{Body, ResponseBody};
use actix_web::middleware::errhandlers::ErrorHandlerResponse;
use actix_web::{dev, Result};

pub fn render_404<B>(
    res: dev::ServiceResponse<B>,
) -> Result<ErrorHandlerResponse<actix_web::body::Body>> {
    let res = res.map_body(|_, _| {
        ResponseBody::Body(Body::from_message("<h1>404 not found</h1>"))
    });
    Ok(ErrorHandlerResponse::Response(res))
}
