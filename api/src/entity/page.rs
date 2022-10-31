use serde::{Deserialize, Serialize};

/// 分页
#[derive(Serialize, Deserialize, Debug)]
pub struct Page{
    pub page:usize,
    pub size:usize
}