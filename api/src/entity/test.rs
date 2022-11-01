use chrono::Local;
use sea_orm::{DeleteResult, Set};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::entity::page::Page;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, Default)]
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
    pub async fn create(db: &DbConn) -> Result<ActiveModel, DbErr> {
        let time = Local::now();

        ActiveModel {
            name: Set("你好".to_string()),
            version: Set(1),
            create_time: Set(time.naive_local()),
            ..Default::default()
        }
            .save(db)
            .await
    }

    pub async fn list(page: Page, db: &DbConn) -> Result<(Vec<Model>, usize), DbErr> {
        println!("分页参数: {:?}",page);
        let paginator = Entity::find()
            .paginate(db, page.size);

        let num_items = paginator.num_items().await?;

        // 获取分页数据
        let v = paginator.fetch_page(page.page - 1).await.map(|p| (p, num_items));

        //let chocolate: Vec<Model> = Entity::find().all(db).await?; //查询所有
        //return Ok(chocolate);

        return v;
    }

    pub async fn update(id: i32, db: &DbConn) -> Result<Model, DbErr> {
        let update: ActiveModel = Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("查询不到对象！".to_owned()))
            .map(Into::into)?;

        ActiveModel {
            id: update.id,
            name: Set("修改数据示例".to_string()),
            ..Default::default()
        }
            .update(db)
            .await
    }

    pub async fn delete(ids: Vec<usize>, db: &DbConn) -> Result<String, DbErr> {

        let mut b = false;

        for x in ids {
            let res: DeleteResult = Entity::delete_by_id(x as i32).exec(db).await?;
            if res.rows_affected != 1 {
                b = true;
                break
            }
        }
        if b {
            Err(DbErr::RecordNotFound("找不到数据！".to_string()))
        }else{
            Ok("删除成功！".to_string())
        }

        // Entity 是红色的不要紧，2022/10/28 因为这是黑魔法。很夸张！
        // let del: ActiveModel = Entity::find_by_id(id)
        //     .one(db)
        //     .await?
        //     .ok_or(DbErr::Custom("找不到数据！.".to_owned()))
        //     .map(Into::into)?;
        //
        // del.delete(db).await
    }
}
