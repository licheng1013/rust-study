mod schema;
mod models;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use self::models::{NewPost, Post};

fn main()  {
    let  connection = &mut establish_connection();
    let post = create_post(connection, "Hello", "World");
    println!("{:?}",post)

}


pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &mut MysqlConnection, title: &str, body: &str) -> Post {
    use schema::posts::dsl::{id, posts};

    let new_post = NewPost { title, body };

    diesel::insert_into(posts)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post");

    posts.order(id.desc()).first(conn).unwrap()
}