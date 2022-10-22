use crate::util::r::ok_msg;
use crate::{get, post};
use actix_web::web::Json;
use actix_web::{Responder, Result};

#[get("/")]
pub async fn hello() -> Result<impl Responder> {
    Ok(Json(ok_msg("HelloWorld".to_string())))
}

#[post("/echo")]
pub async fn echo(req_body: String) -> Result<impl Responder> {
    Ok(Json(ok_msg(req_body.to_string())))
}
