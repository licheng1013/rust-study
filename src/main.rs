extern crate core;

use actix_cors::Cors;
use actix_web::{App, get, HttpServer, post, web};
use actix_web::http::header;
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
    util::token::test();
    test::test::test_study();
    let port = 8080;
    println!("http://localhost:{}", port);

    let redis = redis::Client::open("redis://:root@192.168.101.11:6379").unwrap();

    let db_url = "mysql://root:root@192.168.101.11:3306/t_ldx";
    let options = ConnectOptions::new(db_url.to_string());
    let conn = sea_orm::Database::connect(options).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("send_wildcard")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(web::Data::new(conn.clone())) //mysql
            .app_data(web::Data::new(redis.clone()))//redis
            .wrap(middleware::auth::Auth)

            .service(api::index::hello)
            .service(api::index::echo)
            .configure(api::user::user_api)
            .configure(api::test::test_api)
            .configure(api::redis::redis_api)
            .configure(api::mysql::mysql_api)

    })
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
