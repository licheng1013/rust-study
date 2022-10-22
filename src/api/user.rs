// use std::rc::Rc;
// use actix_web::{Route, web};

//static  USER_ROUTER: Route = web::route();

// pub async fn init() -> Route {
//     //USER_ROUTER.get(login)
// }

use actix_web::{HttpResponse, Responder, Result};
use actix_web::web::{Json, Path, Query};
use crate::{ get};
use crate::entity::user::User;
//use crate::util::r::R;

/// 测试
#[get("/test")]
pub async fn test(user: Json<User>) -> Result<String> {
    Ok(format!("Welcome {}!", user.username))
    //println!("Hello Login");
}

/// 登入接口
#[get("/login")]
pub async fn login() -> impl Responder {
    HttpResponse::Ok().body("Hello login!")
}


/// 路径参数解析
#[get("/one/{id}")]
pub async fn one(id: Path<i64>) -> Result<impl Responder> {
    let user = User {
        username: id.to_string(),
        password: "".to_string(),
    };
    Ok(Json(user))
}

/// 查询list
#[get("/list")]
pub async fn list(user: Query<User>) -> impl Responder {
    format!("Your name is {},and Your password is {}", user.username,user.password)



    //Ok(Json(user.to_string()))
}