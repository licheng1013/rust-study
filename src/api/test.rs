use crate::service::user::UserService;
use crate::util::r;
use actix_web::get;
use actix_web::web::Json;
use actix_web::{Responder, Result};

/// 测试错误返回
#[get("/1")]
pub async fn test1() -> Result<impl Responder> {
    UserService::hello();
    Ok(Json(r::fail("test1".to_string())))
}

/// 测试异常
#[get("/2")]
pub async fn test2() -> Result<impl Responder> {
    let sum = 10 / 0;
    Ok(Json(r::fail(sum.to_string())))
}
