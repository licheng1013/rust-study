use actix_cors::Cors;
use actix_web::http::header;

pub fn cors() -> Cors {
    return Cors::default()
        .allowed_origin("send_wildcard")
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        .allowed_header(header::CONTENT_TYPE)
        .supports_credentials()
        .max_age(3600);
}