use std::env;
use dotenvy::dotenv;
use rbatis::RBatis;
use rbatis::rbdc::datetime::DateTime;
use rbatis::rbdc::uuid::Uuid;
use rbatis::sql::PageRequest;
use rbdc_mysql::driver::MysqlDriver;
use crate::service::admin::Admin;

mod service;

#[tokio::main]
pub async fn main() {
    dotenv().ok();
    let database_url = "mysql://root:root@192.168.101.90:3306/t_gorm".to_string();

    // 日志开启
    //fast_log::init(fast_log::Config::new().console()).expect("初始化日志失败");
    // 初始话联机对象
    let mut rb = RBatis::new();
    //mysql
    rb.init(MysqlDriver{},database_url.as_str()).unwrap();


    // let mut activity = Admin {
    //     admin_id: Some(1.into()),
    //     user_name: Some("用户名称".into()),
    //     password:  Some("密码".into()),
    //     salt:  Some("盐值".into()),
    //     create_time:  Some(DateTime::now()),
    //     nick_name: Some("昵称".into()),
    // };
    // let data = Admin::insert(&mut rb, &activity).await;
    // println!("insert = {:?}", data);

    let data = Admin::list(&mut rb, "3").await;
    println!("查询多条记录 = {:?}", data);

    let data = Admin::one(&mut rb, "3").await;
    println!("查询单条记录 = {:?}", data);

    // activity.user_name = Some(Uuid::new().to_string()[0..7].to_string());
    // let data = Admin::update(&mut rb, &activity, "3").await;
    // println!("修改 = {:?}", data);

    let data = Admin::page(&mut rb, &PageRequest::new(1, 10)).await;
    println!("分页 = {:?}", data);

    let data = service::admin::select_by_condition(&mut rb, "%admin%").await;
    println!("htmlSql = {:?}", data);

}
