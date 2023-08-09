use std::future::IntoFuture;
use std::sync::Arc;
use actix_web::web::Data;
use rbatis::{crud, impl_select_page, RBatis};
use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::PageRequest;
use crate::model::admin::Admin;
use crate::util::page::{PageParam, PageResult};

const TABLE_NAME: &str = "t_admin";

crud!(Admin{},TABLE_NAME);
impl_select_page!(Admin{page(where_str:&str) => "${where_str}"},TABLE_NAME);

pub async fn list(page: PageParam, model: Admin,  rb: Data<Arc<RBatis>>) -> PageResult<Vec<Admin>> {
    let mut executor = rb.acquire().await.expect("加载失败");
    let condition = where_condition(model);
    println!("{condition:?}");
    let result = Admin::page(
        &mut executor,
        &PageRequest::new(page.page.unwrap_or(1)
                          , page.size.unwrap_or(10)), &condition).await.unwrap();
    return PageResult {
        total: result.total,
        list: result.records,
    };
}

pub async fn update(model: Admin,  rb: Data<Arc<RBatis>>) {
    let mut executor = rb.acquire().await.expect("加载失败");
    let result = Admin::update_by_column(&mut executor
                                         , &model, "id").await.unwrap();
    println!("{result:?}")
}

pub async fn delete(model: Admin,  rb: Data<Arc<RBatis>>) {
    let mut executor = rb.acquire().await.expect("加载失败");
    let result = Admin::delete_by_column(&mut executor, "id"
                                         , model.id.unwrap_or(0)).await.unwrap();
    println!("{result:?}")
}

pub async fn insert(mut model: Admin,  rb: Data<Arc<RBatis>>) {
    let mut executor = rb.acquire().await.expect("加载失败");
    model.create_time = Some(DateTime::now());
    let result = Admin::insert(&mut executor, &model).await.unwrap();
    println!("{result:?}")
}

pub fn where_condition(model: Admin) -> String {
    let mut where_str = String::from("");
    if model.id != None {
        where_str.push_str(format!("and id = \"{}\"", model.id.unwrap()).as_str())
    }
    if model.nick_name != None {
        where_str.push_str(format!("and nick_name = \"{}\"", model.nick_name.unwrap()).as_str())
    }
    if model.user_name != None {
        where_str.push_str(format!("and user_name = \"{}\"", model.user_name.unwrap()).as_str())
    }
    if model.salt != None {
        where_str.push_str(format!("and salt = \"{}\"", model.salt.unwrap()).as_str())
    }
    if model.password != None {
        where_str.push_str(format!("and password = \"{}\"", model.password.unwrap()).as_str())
    }
    if model.create_time != None {
        where_str.push_str(format!("and create_time = \"{}\"", model.create_time.unwrap()).as_str())
    }
    if where_str.len() == 0 {
        return "".to_string();
    }
    where_str = where_str[3..where_str.len()].to_string();
    return format!("where{}",where_str)
}


