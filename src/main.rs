extern crate core;

use actix_web::{App, get, HttpServer, post, web};
use sea_orm::ConnectOptions;

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

    let redis = redis::Client::open("redis://:root@192.168.101.11:6379").unwrap();

    let db_url = "mysql://root:root@192.168.101.11:3306/t_ldx";
    let options = ConnectOptions::new(db_url.to_string());
    let conn = sea_orm::Database::connect(options).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(conn.clone())) //mysql
            .app_data(web::Data::new(redis.clone()))//redis
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
                    .service(api::test::file),
            )
            .service(
                web::scope("/redis")
                    .service(api::redis::set)
                    .service(api::redis::get)
                    .service(api::redis::del),
            )
            .service(
                web::scope("/mysql")
                    .service(api::mysql::list)
                    .service(api::mysql::delete)

            )
    })
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
