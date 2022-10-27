extern crate core;

use actix_web::{App, get, HttpServer, post, web};
use actix_web::http::StatusCode;
use actix_web::middleware::{ErrorHandlers, Logger};

use crate::middleware::error::add_error_header;

// fn main() {
//     println!("Hello, world!");
// }
mod api;
mod entity;
mod middleware;
mod service;
mod test;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    test::test::test_study();

    let port = 8080;
    println!("http://localhost:{}", port);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(actix_web::middleware::DefaultHeaders::new())
            .wrap(ErrorHandlers::new().handler(StatusCode::OK, add_error_header))
            .wrap(middleware::auth::Auth)

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
                    .service(api::test::test2)
                    .service(api::test::test3)
                    .service(api::test::test4)
                    .service(api::test::file)
                ,
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
