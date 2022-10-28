use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Model {
    #[serde(skip_deserializing)]
    /// 主键
    pub id: i32,
    /// 姓名
    pub name: String,
    /// 创建时间
    pub create_time: DateTime,
    /// 乐观锁
    pub version: i32,
}