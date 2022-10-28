use actix_web::{error, get, Responder, web};
use actix_web::web::{Json, Path};

use crate::util::r::{fail, ok_msg};

#[get("set/{path}")]
pub async fn set(path: Path<String>, redis: web::Data<redis::Client>)
                 -> actix_web::Result<impl Responder> {
    let mut conn = redis.get_tokio_connection_manager().await
        .map_err(error::ErrorInternalServerError)?;

    let res = redis::Cmd::set("K", path.into_inner()).query_async::<_, String>(&mut conn)
        .await
        .map_err(error::ErrorInternalServerError)?;

    // 不是绝对必要的，但成功的 SET 操作会返回“OK”
    if res == "OK" {
        Ok(Json(ok_msg("插入成功".to_string())))
    } else {
        Ok(Json(fail("插入失败".to_string())))
    }
}


#[get("get/{path}")]
pub async fn get(path: Path<String>, redis: web::Data<redis::Client>)
                 -> actix_web::Result<impl Responder> {
    let mut conn = redis.get_tokio_connection_manager().await
        .map_err(error::ErrorInternalServerError)?;

    println!("key: {:?}",path);
    let res = redis::Cmd::get(path.into_inner()).query_async::<_, String>(&mut conn)
        .await.map_err(error::ErrorInternalServerError)?;

    Ok(Json(ok_msg(res.to_string())))
}


#[get("del")]
pub async fn del(redis: web::Data<redis::Client>) -> actix_web::Result<impl Responder> {
    let mut conn = redis
        .get_tokio_connection_manager()
        .await
        .map_err(error::ErrorInternalServerError)?;

    let res = redis::Cmd::del("K")
        .query_async::<_, usize>(&mut conn)
        .await
        .map_err(error::ErrorInternalServerError)?;

    // 不是绝对必要的，但成功的 DEL 操作会返回删除的键数
    if res == 1 {
        Ok(Json(ok_msg("删除成功！".to_string())))
    } else {
        println!("deleted {res} keys");
        Ok(Json(ok_msg("已删除: {0} 键！".to_string())))
    }
}

