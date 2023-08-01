use std::any::Any;
use rbatis::{crud, impl_delete, impl_select, impl_select_page, impl_update};
use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::{Page, PageRequest};
use crate::model::admin::Admin;
use crate::RB;
use crate::util::page::{PageParam, PageResult};

const TABLE_NAME: &str = "t_admin";

crud!(Admin{},TABLE_NAME);

impl_select!(Admin{select_by_id(id:String) -> Option => "`where id = #{id} limit 1`"},TABLE_NAME);
impl_update!(Admin{update(id:&str) => "`where admin_id = #{id}`"},TABLE_NAME);
impl_delete!(Admin {delete(id:&str) => "`where admin_id = #{id}`"},TABLE_NAME);
impl_select_page!(Admin{page(name:&str) => "``"},TABLE_NAME);

pub async fn list(page: PageParam, admin: Admin) -> PageResult<Vec<Admin>> {
    let result = Admin::page(
        &mut RB.clone(),
        &PageRequest::new(page.page.unwrap_or(1)
                          ,page.size.unwrap_or(10)), "").await.unwrap();
    return PageResult {
        total: result.total,
        list: result.records,
    };
}

pub async fn update(admin: Admin) {
    println!("{admin:?}")
}

pub async fn delete(admin: Admin) {
    println!("{admin:?}")
}

pub async fn insert(mut admin: Admin) {
    admin.create_time = Some(DateTime::now());
    Admin::insert(&mut RB.clone(), &admin).await.unwrap();
}

