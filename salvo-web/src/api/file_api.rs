use std::path::Path;
use salvo::prelude::*;
use crate::util::result::{fail, ok_msg};

#[handler]
async fn upload(req: &mut Request, res: &mut Response) {
    let file = req.file("file").await;
    if let Some(file) = file {
        // 只能绝对路径
        let dest = format!("D:\\my-study\\rust-study\\salvo-web\\{}", file.name().unwrap_or("file"));
        if let Err(e) = std::fs::copy(&file.path(), Path::new(&dest)) {
            println!("文件错误 {e:?}");
            res.render(Json(fail("上传失败!".to_string())));
            return;
        } else {
            println!("上传路径 {dest:?}");
        };
        res.render(Json(ok_msg("上传成功!".to_string())));
        return;
    }
    res.render(Json(fail("上传失败!".to_string())));
}

pub fn router() -> Router {
    Router::with_path("file")
        .push(Router::with_path("upload").post(upload))
}