use std::env;
use dotenvy::dotenv;
use rbatis::Rbatis;
use rbatis::rbdc::datetime::FastDateTime;
use rbatis::sql::PageRequest;
use rbdc_mysql::driver::MysqlDriver;
use crate::service::biz_activity::Admin;

mod service;

#[tokio::main]
pub async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("没有设置数据库连接");

    /// enable log crate to show sql logs
    fast_log::init(fast_log::Config::new().console()).expect("初始化日志失败");
    /// initialize rbatis. also you can call rb.clone(). this is  an Arc point
    let mut rb = Rbatis::new();
    //mysql
    rb.init(MysqlDriver{},database_url.as_str()).await.unwrap();


    let activity = Admin {
        admin_id: Some("1".into()),
        user_name: Some("用户名称".into()),
        password:  Some("密码".into()),
        salt:  Some("盐值".into()),
        create_time:  Some(FastDateTime::now()),
        nick_name: Some("昵称".into()),
    };
    let data = Admin::insert(&mut rb, &activity).await;
    println!("insert = {:?}", data);

    let data = Admin::select_all_by_id(&mut rb, "1", "1").await;
    println!("select_all_by_id = {:?}", data);

    let data = Admin::select_by_id(&mut rb, "1".to_string()).await;
    println!("select_by_id = {:?}", data);

    let data = Admin::update_by_column(&mut rb, &activity, "id").await;
    println!("update_by_column = {:?}", data);

    let data = Admin::update_by_name(&mut rb, &activity, "test").await;
    println!("update_by_name = {:?}", data);

    let data = Admin::delete_by_column(&mut rb, "id", &"2".into()).await;
    println!("delete_by_column = {:?}", data);

    let data = Admin::delete_by_name(&mut rb, "2").await;
    println!("delete_by_column = {:?}", data);

    let data = Admin::select_page(&mut rb, &PageRequest::new(1, 10), "2").await;
    println!("select_page = {:?}", data);
}
