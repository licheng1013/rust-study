// fn main() {
//     println!("Hello, world!");
// }
mod api;
mod entity;
mod util;

use actix_web::{App, HttpServer, get, post, web};
use actix_web::middleware::Logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addrs = "127.0.0.1";
    let port = 8080;
    println!("http://{}:{}", addrs, port);

    //let mut user = web::scope("/user").service(api::user::login);


    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(api::index::hello)
            .service(api::index::echo)


            .service(web::scope("/user")
                .service(api::user::login)
                .service(api::user::test)
                .service(api::user::one)
                .service(api::user::list)
            )
            //.route("/hey", web::get().to(manual_hello))

    })
    .bind((addrs, port))?.run()
    .await
}



// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

