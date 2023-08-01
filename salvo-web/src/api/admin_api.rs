use salvo::prelude::*;
use crate::util::result::ok_data;

#[handler]
async fn list(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.render(Json(ok_data("Hi admin list")));
}

#[handler]
async fn update(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.render(Json(ok_data("Hi admin update")));
}

#[handler]
async fn delete(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.render(Json(ok_data("Hi admin delete")));
}

#[handler]
async fn insert(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.render(Json(ok_data("Hi admin insert")));
}

pub fn router() -> Router {
    Router::with_path("admin")
        .push(Router::with_path("list").get(list))
        .push(Router::with_path("update").post(update))
        .push(Router::with_path("delete").post(delete))
        .push(Router::with_path("insert").post(insert))
}