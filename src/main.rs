// fn main() {
//     println!("Hello, world!");
// }
mod api;
mod entity;
mod middleware;
mod service;
mod test;
mod util;

use crate::middleware::error::add_error_header;
use actix_web::http::StatusCode;
use actix_web::middleware::{ErrorHandlers, Logger};
use actix_web::{get, post, web, App, HttpServer};

use futures::future::FutureExt;
use actix_web::dev::Service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    test::test::test_study();

    let port = 8080;
    println!("http://localhost:{}", port);

    HttpServer::new(|| {
        App::new()
            .wrap_fn(|req, srv| {
                println!("Hi from start. You requested: {}", req.path());
                srv.call(req).map(|res| {
                    println!("Hi from response");
                    res
                })
            })

            .wrap(Logger::default())
            .wrap(actix_web::middleware::DefaultHeaders::new())

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
                    .service(api::test::test2)
                    .service(api::test::test3),
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
