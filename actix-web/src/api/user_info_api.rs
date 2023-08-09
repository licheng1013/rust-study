use std::sync::Arc;
use actix_web::*;
use actix_web::web::*;
use rbatis::RBatis;

use crate::{get, logic, post};
use crate::model::user_info::UserInfo;
use crate::util::page::PageParam;
use crate::util::result::{ok_data, ok_msg};

pub fn user_info_api(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/user/info")
            .service(list)
            .service(update)
            .service(delete)
            .service(insert)
    );
}

/// 列表
#[get("/list")]
async fn list(model: Query<UserInfo>,page:Query<PageParam>, rb: Data<Arc<RBatis>>) -> Result<impl Responder> {
    println!("收到数据: {:#?}, {:#?}", model,page);
    Ok(Json(ok_data(logic::user_info_logic::list(page.into_inner(),model.into_inner(),rb).await)))
}

/// 修改
#[post ("/update")]
async fn update(model: Json<UserInfo>, rb: Data<Arc<RBatis>>) -> Result<impl Responder> {
    logic::user_info_logic::update(model.into_inner(),rb).await;
    Ok(Json(ok_msg("修改成功".to_string())))
}

/// 删除
#[post("/delete")]
async fn delete(model: Json<UserInfo>, rb: Data<Arc<RBatis>>) -> Result<impl Responder> {
    logic::user_info_logic::delete(model.into_inner(),rb).await;
    Ok(Json(ok_msg("删除成功".to_string())))
}

/// 插入
#[post("/insert")]
async fn insert(model: Json<UserInfo>, rb: Data<Arc<RBatis>>) -> Result<impl Responder> {
    logic::user_info_logic::insert(model.into_inner(),rb).await;
    Ok(Json(ok_msg("插入成功".to_string())))
}
