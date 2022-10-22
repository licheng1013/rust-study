use actix_web::middleware::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::{
    dev,
    http::{header, StatusCode},
    web, App, HttpResponse, HttpServer, Result,
};

fn add_error_header<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("Error"),
    );

    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}
