use std::future::IntoFuture;
use rbatis::{crud, impl_delete, impl_select, impl_select_page, impl_update};
use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::PageRequest;
use rbs::Value;
use rbs::value::map::ValueMap;
use crate::model::admin::Admin;
use crate::RB;
use crate::util::page::{PageParam, PageResult};

const TABLE_NAME: &str = "t_admin";

crud!(Admin{},TABLE_NAME);
impl_select_page!(Admin{page(where_str:&str) => "${where_str}"},TABLE_NAME);

pub async fn list(page: PageParam, admin: Admin) -> PageResult<Vec<Admin>> {
    let condition = where_condition(admin);
    println!("{condition:?}");
    let result = Admin::page(
        &mut RB.clone(),
        &PageRequest::new(page.page.unwrap_or(1)
                          , page.size.unwrap_or(10)), &condition).await.unwrap();
    return PageResult {
        total: result.total,
        list: result.records,
    };
}

pub async fn update(admin: Admin) {
    let result = Admin::update_by_column(&mut RB.clone()
                                         , &admin, "id").await.unwrap();
    println!("{result:?}")
}

pub async fn delete(admin: Admin) {
    let result = Admin::delete_by_column(&mut RB.clone(), "id"
                                         , admin.id.unwrap_or(0)).await.unwrap();
    println!("{result:?}")
}

pub async fn insert(mut admin: Admin) {
    admin.create_time = Some(DateTime::now());
    let result = Admin::insert(&mut RB.clone(), &admin).await.unwrap();
    println!("{result:?}")
}

pub fn where_condition(admin: Admin) -> String {
    let mut where_str = String::from("");
    if admin.id != None {
        where_str.push_str(format!("and id = \"{}\"", admin.id.unwrap()).as_str())
    }
    if admin.nick_name != None {
        where_str.push_str(format!("and nick_name = \"{}\"", admin.nick_name.unwrap()).as_str())
    }
    if admin.user_name != None {
        where_str.push_str(format!("and user_name = \"{}\"", admin.user_name.unwrap()).as_str())
    }
    if admin.salt != None {
        where_str.push_str(format!("and salt = \"{}\"", admin.salt.unwrap()).as_str())
    }
    if admin.password != None {
        where_str.push_str(format!("and password = \"{}\"", admin.password.unwrap()).as_str())
    }
    if admin.create_time != None {
        where_str.push_str(format!("and create_time = \"{}\"", admin.create_time.unwrap()).as_str())
    }
    if where_str.len() == 0 {
        return "".to_string();
    }
    where_str = where_str[3..where_str.len()].to_string();
    return format!("where{}",where_str)
}


