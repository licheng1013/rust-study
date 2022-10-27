use actix_web::{error, get, HttpResponse, Responder, web};
use actix_web::web::Path;

#[get("set/{path}")]
pub async fn cache_stuff(
    path:Path<String>,
    redis: web::Data<redis::Client>,
) -> actix_web::Result<impl Responder> {
    let mut conn = redis.get_tokio_connection_manager().await
        .map_err(error::ErrorInternalServerError)?;


    let res = redis::Cmd::set("K",path.into_inner()).query_async::<_, String>(&mut conn)
        .await
        .map_err(error::ErrorInternalServerError)?;


    // not strictly necessary, but successful SET operations return "OK"
    if res == "OK" {
        Ok(HttpResponse::Ok().body("successfully cached values"))
    } else {
        Ok(HttpResponse::InternalServerError().finish())
    }
}

#[get("del")]
pub async fn del_stuff(redis: web::Data<redis::Client>) -> actix_web::Result<impl Responder> {
    let mut conn = redis
        .get_tokio_connection_manager()
        .await
        .map_err(error::ErrorInternalServerError)?;

    let res = redis::Cmd::del(&["my_domain:one", "my_domain:two", "my_domain:three"])
        .query_async::<_, usize>(&mut conn)
        .await
        .map_err(error::ErrorInternalServerError)?;

    // not strictly necessary, but successful DEL operations return the number of keys deleted
    if res == 3 {
        Ok(HttpResponse::Ok().body("successfully deleted values"))
    } else {
        println!("deleted {res} keys");
        Ok(HttpResponse::InternalServerError().finish())
    }
}