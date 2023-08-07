use actix_web::*;

mod api;
mod util;
mod model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .configure(api::admin_api::admin_api)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}


