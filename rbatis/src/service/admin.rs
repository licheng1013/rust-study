use std::string::ToString;
use rbatis::{rbdc::datetime::{ DateTime}, html_sql, executor::Executor};
use serde::{Deserialize, Serialize};
use rbatis::{impl_update, impl_delete, impl_select_page, impl_select};

const TABLE_NAME:&str = "t_admin";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct  Admin {
    pub admin_id: Option<i64>,
    pub user_name: Option<String>,
    pub password: Option<String>,
    pub salt: Option<String>,
    pub create_time: Option<DateTime>,
    pub nick_name: Option<String>,
}

rbatis::crud!(Admin {},TABLE_NAME);

impl_select!(Admin{list(id:&str) => "`where id = #{id}`"},TABLE_NAME);
impl_select!(Admin{one(id:&str) -> Option => "`where id = #{id} limit 1`"},TABLE_NAME);
impl_update!(Admin{update(id:&str) => "`where id = #{id}`"},TABLE_NAME);
impl_delete!(Admin {delete(id:&str) => "`where id = #{id}`"},TABLE_NAME);
impl_select_page!(Admin{page() => ""},TABLE_NAME);

#[html_sql("rbatis/example.html")]
pub async fn select_by_condition(rb: &mut dyn Executor, user_name: &str) -> Vec<Admin> {
    impled!()
}