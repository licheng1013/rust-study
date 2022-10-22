// fn main() {
//     println!("Hello, world!");
// }
mod api;
mod entity;
mod middleware;
mod service;
mod util;

use crate::middleware::error::add_error_header;
use actix_web::http::StatusCode;
use actix_web::middleware::ErrorHandlers;
use actix_web::{get, post, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addrs = "127.0.0.1";
    let port = 8080;
    println!("http://{}:{}", addrs, port);

    HttpServer::new(|| {
        App::new()
            //.wrap(Logger::default())
            .wrap(ErrorHandlers::new().handler(StatusCode::OK, add_error_header))
            .service(api::index::hello)
            .service(api::index::echo)
            .service(
                web::scope("/user")
                    .service(api::user::login)
                    .service(api::user::one)
                    .service(api::user::list),
            )
            .service(
                web::scope("/test")
                    .service(api::test::test1)
                    .service(api::test::test2),
            )
    })
    .bind((addrs, port))?
    .run()
    .await
}
