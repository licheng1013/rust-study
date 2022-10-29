// use std::rc::Rc;
// use actix_web::{Route, web};

//static  USER_ROUTER: Route = web::route();

// pub async fn init() -> Route {
//     //USER_ROUTER.get(login)
// }

use actix_web::{Responder, Result, web};
use actix_web::web::{Json, Path, Query};

use crate::{get, post};
use crate::entity::user::User;
use crate::util::r;

pub fn user_api(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user")
            .service(login)
            .service(one)
            .service(list),
    );
}


/// 登入接口
#[get("/login")]
pub async fn login() -> Result<impl Responder> {
    let result = r::ok_msg("login".to_string());
    Ok(Json(result))
}

/// 单个 - 示例: 路径参数解析
#[get("/one/{id}")]
pub async fn one(id: Path<i64>) -> Result<impl Responder> {
    let user = User {
        username: id.to_string(),
        password: "".to_string(),
    };
    let result = r::ok_data(user);
    Ok(Json(result))
}

/// 列表
#[get("/list")]
pub async fn list( user: Query<User>) -> Result<impl Responder> {
    println!("收到数据: {:#?}",user);
    Ok(Json(r::ok_data(user.into_inner())))
}

/// 修改
#[post("/update")]
pub async fn update(user: Query<User>) -> Result<impl Responder> {
    let result = r::ok_data(user.into_inner());
    Ok(Json(result))
}

/// 删除
#[post("/delete")]
pub async fn delete(user: Query<User>) -> Result<impl Responder> {
    let result = r::ok_data(user.into_inner());
    Ok(Json(result))
}
