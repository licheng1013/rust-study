use salvo::prelude::*;
use crate::util::result::ok_data;

#[handler]
async fn list(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.render(Json(ok_data("Hi user List")));
}