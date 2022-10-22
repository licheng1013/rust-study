use actix_web::{Responder, web,Result};
use crate::{entity, get};

// 测试不同类
#[get("/one/{id}")]
pub async fn one(id: web::Path<i64>) -> Result<impl Responder> {
    let user = entity::user::User {
        username: id.to_string(),
        password: "".to_string(),
    };
    Ok(web::Json(user))
}