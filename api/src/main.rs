extern crate core;

use std::env;
use actix_web::{App, get, HttpServer, post, web};
use dotenvy::dotenv;
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
    // 配置需要在根目录
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("没有设置数据库连接");
    let port = env::var("PORT").expect("没有设置端口");
    println!("数据库连接: {}",database_url);
    util::token::test();//测试Token
    println!("访问地址: http://localhost:{}", port);
    let redis = redis::Client::open("redis://:root@192.168.101.8:6379").unwrap();
    let options = ConnectOptions::new(database_url);
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
    .bind(("0.0.0.0", port.parse().expect("不是数字")))?
    .run()
    .await
}
