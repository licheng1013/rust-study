use rbatis::rbdc::datetime::FastDateTime;
use serde::{Deserialize, Serialize};
use rbatis::{impl_update, impl_delete, impl_select_page, impl_select};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct  Admin {
    pub admin_id: Option<i64>,
    pub user_name: Option<String>,
    pub password: Option<String>,
    pub salt: Option<String>,
    pub create_time: Option<FastDateTime>,
    pub nick_name: Option<String>,
}

rbatis::crud!(Admin {},"t_admin");

impl_select!(Admin{select_all_by_id(id:&str) => "`where admin_id = #{id}`"},"t_admin");
impl_select!(Admin{select_by_id(id:String) -> Option => "`where id = #{id} limit 1`"},"t_admin");
impl_update!(Admin{update_by_name(name:&str) => "`where id = 1`"},"t_admin");
impl_delete!(Admin {delete_by_name(name:&str) => "`where name= '2'`"},"t_admin");
//limiting condition：maybe pg/mssql not support sql `limit 0,10` you should add arg  `limit_sql:&str` of set limit_sql = " limit 0 offset 10"
//`impl_select_page!(BizActivity{select_page(name:&str,limit_sql:&str) => "`where name != #{name}`"});`
impl_select_page!(Admin{select_page(name:&str) => "`where name != #{name}`"},"t_admin");