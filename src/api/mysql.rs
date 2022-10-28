use actix_web::{get, Responder, Result, web};
use actix_web::web::Json;
use sea_orm::DatabaseConnection;

use crate::util::r;

/// 列表
#[get("/list")]
pub async fn list( mysql: web::Data<DatabaseConnection>) -> Result<impl Responder> {
    println!("{:?}",mysql);
    Ok(Json(r::ok_data("查询列表")))
}