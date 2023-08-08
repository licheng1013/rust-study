use actix_web::*;
use actix_web::web::*;

use crate::{get, post};
use crate::model::admin::Admin;
use crate::util::result::ok_data;

pub fn admin_api(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/admin")
            .service(list)
            .service(update)
            .service(delete)
            .service(insert)
    );
}

/// 列表
#[get("/list")]
async fn list(user: Query<Admin>) -> Result<impl Responder> {
    println!("收到数据: {:#?}", user);
    Ok(Json(ok_data(user.into_inner())))
}

/// 修改
#[post ("/update")]
async fn update(user: Query<Admin>) -> Result<impl Responder> {
    Ok(Json(ok_data(user.into_inner())))
}

/// 删除
#[delete("/delete")]
async fn delete(user: Query<Admin>) -> Result<impl Responder> {
    Ok(Json(ok_data(user.into_inner())))
}

/// 插入
#[put("/insert")]
async fn insert(user: Query<Admin>) -> Result<impl Responder> {
    Ok(Json(ok_data(user.into_inner())))
}
