use rbatis::sql::PageRequest;
use salvo::prelude::*;
use crate::logic;
use crate::model::admin::Admin;
use crate::util::page::PageParam;
use crate::util::result::ok_data;

#[handler]
async fn list(_req: &mut Request, res: &mut Response) {
    let model: Admin = _req.parse_queries().unwrap();
    let page: PageParam = _req.parse_queries().unwrap();
    let data = logic::admin_logic::list(page, model).await;
    res.render(Json(ok_data(&data)));
}

#[handler]
async fn update(_req: &mut Request, res: &mut Response) {
    let model: Admin = _req.parse_json().await.unwrap();
    logic::admin_logic::update(model).await;
    res.render(Json(ok_data("修改成功!")));
}

#[handler]
async fn delete(_req: &mut Request, res: &mut Response) {
    let model: Admin = _req.parse_json().await.unwrap();
    logic::admin_logic::delete(model).await;
    res.render(Json(ok_data("删除成功!")));
}

#[handler]
async fn insert(_req: &mut Request, res: &mut Response) {
    let model: Admin = _req.parse_json().await.unwrap();
    logic::admin_logic::insert(model).await;
    res.render(Json(ok_data("插入成功!")));
}

pub fn router() -> Router {
    Router::with_path("admin")
        .push(Router::with_path("list").get(list))
        .push(Router::with_path("update").post(update))
        .push(Router::with_path("delete").post(delete))
        .push(Router::with_path("insert").post(insert))
}