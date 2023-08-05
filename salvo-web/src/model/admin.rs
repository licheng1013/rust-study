use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Admin {
    /// 管理员id
    pub id: Option<i64>,
    /// 账号
    pub user_name: Option<String>,
    /// 密码
    pub password: Option<String>,
    /// 盐
    pub salt: Option<String>,
    /// 创建时间
    pub create_time: Option<DateTime>,
    /// 昵称
    pub nick_name: Option<String>,
}
