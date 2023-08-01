use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Admin {
    pub id: Option<i64>,
    pub user_name: Option<String>,
    pub password: Option<String>,
    pub salt: Option<String>,
    pub create_time: Option<DateTime>,
    pub nick_name: Option<String>,
}
