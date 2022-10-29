use actix_web::{Responder, Result};
use actix_web::web::Json;

use crate::{get, post};
use crate::util::r::ok_msg;

#[get("/")]
pub async fn hello() -> Result<impl Responder> {
    Ok(Json(ok_msg("HelloWorld".to_string())))
}

#[post("/echo")]
pub async fn echo(req_body: String) -> Result<impl Responder> {
    Ok(Json(ok_msg(req_body.to_string())))
}
