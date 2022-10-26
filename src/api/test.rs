use actix_web::{Responder, Result};
use actix_web::get;
use actix_web::web::{Json, Path};

use crate::service::user::UserService;
use crate::util::r;

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

/// 测试-数组元素
#[get("/3/{list}")]
pub async fn test3(list: Path<String>) -> Result<impl Responder> {
    Ok(Json(r::ok_data(list.into_inner())))
}

// /// 文件上传
// #[get("/4/")]
// pub async fn test4(file: Form<Stdout>) -> Result<impl Responder> {
//
//     Ok(Json(r::ok_data("上传文件成功！")))
// }