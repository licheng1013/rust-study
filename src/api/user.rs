// use std::rc::Rc;
// use actix_web::{Route, web};

//static  USER_ROUTER: Route = web::route();

// pub async fn init() -> Route {
//     //USER_ROUTER.get(login)
// }

use actix_web::{HttpResponse, Responder};
use crate::get;

/// 登入
pub async fn test() {
    println!("Hello Login")
}


#[get("/login")]
pub async fn login() -> impl Responder {
    HttpResponse::Ok().body("Hello login!")
}
