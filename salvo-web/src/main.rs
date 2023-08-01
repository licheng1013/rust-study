use salvo::prelude::*;
use crate::util::result::ok_data;

mod util;
mod api;
mod logic;
mod model;

#[handler]
async fn hello(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.render(Json(ok_data("HelloWorld")));
}

#[tokio::main]
async fn main() {
    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    let router =  Router::new().get(hello)
        .push(api::admin_api::router());
    println!("http://127.0.0.1:5800");
    Server::new(acceptor).serve(router).await;
}