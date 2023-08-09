use std::sync::{Arc, OnceLock};
use actix_web::*;
use rbatis::RBatis;
use rbdc_mysql::driver::MysqlDriver;

mod api;
mod util;
mod model;
mod logic;


#[actix_web::main]
async fn main() -> std::io::Result<()> {


    println!("http://localhost:8080");
    let mysql_uri = "mysql://root:root@192.168.101.90/t_gorm";

    let rb = RBatis::new();
    rb.init(MysqlDriver {}, mysql_uri).unwrap();
    let rb = Arc::new(rb);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(rb.to_owned())) //mysql
            .configure(api::admin_api::admin_api)
            .configure(api::user_info_api::user_info_api)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}


