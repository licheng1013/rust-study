use std::io::Write;

use actix_multipart::Multipart;
use actix_web::{HttpResponse, Responder, Result};
use actix_web::get;
use actix_web::post;
use actix_web::web::{Json, Path};
use actix_web::web;
use futures::TryStreamExt;
use uuid::Uuid;

use crate::service::user::UserService;
use crate::util::r;

/// 测试错误返回
#[get("/1")]
pub async fn test1() -> Result<impl Responder> {
    UserService::hello();
    Ok(Json(r::fail("test1".to_string())))
}

/// 测试异常
#[get("/2")]
pub async fn test2() -> Result<impl Responder> {
    let sum = 10 / 0;
    Ok(Json(r::fail(sum.to_string())))
}

/// 测试-数组元素
#[get("/3/{list}")]
pub async fn test3(list: Path<String>) -> Result<impl Responder> {
    Ok(Json(r::ok_data(list.into_inner())))
}

/// 文件上传
#[post("/4")]
pub async fn test4(mut payload: Multipart) -> Result<impl Responder> {
    println!("收到请求！");
    // 迭代多部分流
    while let Some(mut field) = payload.try_next().await? {
        // 多部分表单数据流必须包含“内容处置”
        let content_disposition = field.content_disposition();

        let filename = content_disposition.get_filename()
            .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);
        println!("文件名: {}",filename);
        let filepath = format!("./{filename}");
        // File::create 是阻塞操作，使用线程池
        let mut f = web::block(|| std::fs::File::create(filepath)).await??;
        // 字段又是字节对象流
        while let Some(chunk) = field.try_next().await? {
            // 文件系统操作被阻塞，我们必须使用线程池
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
    }
    Ok(Json(r::ok_data("上传文件成功！")))
}

#[get("/4")]
pub async fn file() -> HttpResponse {
    // 注意: 下面的 / 是相对于当前请求的路径也就是 => #[get("/4")] 这个路径的根。
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}