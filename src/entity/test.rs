use chrono::Local;
use sea_orm::entity::prelude::*;
use sea_orm::Set;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize,Default)]
#[sea_orm(table_name = "t_test")]
pub struct Model {
    #[sea_orm(primary_key)]
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

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}


pub struct TestDao;

impl TestDao {
    pub async fn create_post(
        db: &DbConn,
    ) -> Result<ActiveModel, DbErr> {
        let time = Local::now();

        ActiveModel {
            name: Set("你好".to_string()),
            version: Set(1),
            create_time: Set( time.naive_local()),
            ..Default::default()
        }
            .save(db)
            .await
    }
}