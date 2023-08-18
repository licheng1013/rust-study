
use rbatis::{htmlsql_select_page, RBatis};
use rbdc_mysql::driver::MysqlDriver;
use salvo::__private::once_cell::sync::Lazy;
use salvo::prelude::*;
use crate::util::result::ok_data;

mod util;
mod api;
mod logic;
mod model;
mod common;


pub static RB: Lazy<RBatis> = Lazy::new(RBatis::new);

#[handler]
async fn hello(_req: &mut Request,  res: &mut Response) {
    res.render(Json(ok_data("HelloWorld"))
);
}

#[tokio::main]
async fn main() {
    // mysql connect info
    let mysql_uri = "mysql://root:root@192.168.101.90/t_gorm";
    RB.init(MysqlDriver {}, mysql_uri).unwrap();


    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    let router =  Router::new().get(hello)
        .push(api::file_api::router())
        .push(api::admin_api::router());

    println!("http://127.0.0.1:5800");
    Server::new(acceptor).serve(router).await;
}