use std::string::ToString;
use rbatis::rbdc::datetime::FastDateTime;
use serde::{Deserialize, Serialize};
use rbatis::{impl_update, impl_delete, impl_select_page, impl_select};

const TABLE_NAME:&str = "t_admin";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct  Admin {
    pub admin_id: Option<i64>,
    pub user_name: Option<String>,
    pub password: Option<String>,
    pub salt: Option<String>,
    pub create_time: Option<FastDateTime>,
    pub nick_name: Option<String>,
}

rbatis::crud!(Admin {},TABLE_NAME);

impl_select!(Admin{select_all_by_id(id:&str) => "`where admin_id = #{id}`"},TABLE_NAME);
impl_select!(Admin{select_by_id(id:String) -> Option => "`where id = #{id} limit 1`"},TABLE_NAME);
impl_update!(Admin{update_by_name(name:&str) => "`where id = 1`"},TABLE_NAME);
impl_delete!(Admin {delete_by_name(name:&str) => "`where name= '2'`"},TABLE_NAME);
//limiting condition：maybe pg/mssql not support sql `limit 0,10` you should add arg  `limit_sql:&str` of set limit_sql = " limit 0 offset 10"
//`impl_select_page!(BizActivity{select_page(name:&str,limit_sql:&str) => "`where name != #{name}`"});`
impl_select_page!(Admin{select_page(name:&str) => "`where name != #{name}`"},TABLE_NAME);