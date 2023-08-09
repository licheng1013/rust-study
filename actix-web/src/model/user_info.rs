use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserInfo {
    /// 主键
    pub id: Option<i64>,
    /// 名称
    pub name: Option<String>,
    /// 昵称
    pub nickname: Option<String>,
    /// 创建时间
    pub create_time: Option<DateTime>,
}
