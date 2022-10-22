// use std::rc::Rc;
// use actix_web::{Route, web};

//static  USER_ROUTER: Route = web::route();

// pub async fn init() -> Route {
//     //USER_ROUTER.get(login)
// }

use actix_web::{HttpResponse, Responder, web, Result};
use crate::{entity, get};

/// 测试
#[get("/test")]
pub async fn test(user: web::Json<entity::user::User>) -> Result<String> {
    Ok(format!("Welcome {}!", user.username))
    //println!("Hello Login");
}

/// 登入接口
#[get("/login")]
pub async fn login() -> impl Responder {
    HttpResponse::Ok().body("Hello login!")
}


/// 调试接口
#[get("/one/{id}")]
pub async fn one(id: web::Path<i64>) -> Result<impl Responder> {
    let user = entity::user::User {
        username: id.to_string(),
        password: "".to_string(),
    };
    Ok(web::Json(user))
}