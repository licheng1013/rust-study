extern crate core;

use actix_web::{App, get, HttpServer, post, web};
use sea_orm::ConnectOptions;

use crate::config::config::cors;

// fn main() {
//     println!("Hello, world!");
// }
mod api;
mod config;
mod entity;
mod middleware;
mod service;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    util::token::test();
    let port = 8080;
    println!("http://localhost:{}", port);

    let redis = redis::Client::open("redis://:root@192.168.101.13:6379").unwrap();

    let db_url = "mysql://root:root@192.168.101.13:3306/t_ldx";
    let options = ConnectOptions::new(db_url.to_string());
    let conn = sea_orm::Database::connect(options).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(cors())
            .app_data(web::Data::new(conn.clone())) //mysql
            .app_data(web::Data::new(redis.clone())) //redis
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
