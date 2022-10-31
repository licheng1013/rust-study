use actix_web::{get, Responder, Result, web};
use actix_web::web::{Json, Path};
use sea_orm::DatabaseConnection;

use crate::entity::test::TestDao;
use crate::util::r;

pub fn mysql_api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/mysql")
            .service(list)
            .service(delete)
            .service(create)
            .service(update),
    );
}

/// 列表
#[get("/list")]
pub async fn list(mysql: web::Data<DatabaseConnection>) -> Result<impl Responder> {
    let test_dao = TestDao::list(&mysql).await.unwrap();
    Ok(Json(r::ok_data(test_dao)))
}

/// 删除
#[get("/remove/{id}")]
pub async fn delete(id: Path<i32>, mysql: web::Data<DatabaseConnection>) -> Result<impl Responder> {
    let x = TestDao::delete(id.into_inner(), &mysql).await; //结果未处理
    println!("{:#?}", x);
    Ok(Json(r::ok_data("删除记录")))
}

/// 创建
#[get("/create")]
pub async fn create(mysql: web::Data<DatabaseConnection>) -> Result<impl Responder> {
    let x = TestDao::create(&mysql).await.unwrap();
    println!("{:#?}", x);
    println!("{:?}", mysql);
    Ok(Json(r::ok_data("创建")))
}

/// 修改
#[get("/update/{id}")]
pub async fn update(id: Path<i32>, mysql: web::Data<DatabaseConnection>) -> Result<impl Responder> {
    let x = TestDao::update(id.into_inner(), &mysql).await.unwrap();
    println!("{:#?}", x);
    println!("{:?}", mysql);
    Ok(Json(r::ok_data("修改成功！")))
}
