use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PageParam {
    pub page: Option<u64>,
    pub size: Option<u64>,
}


#[derive(Serialize, Deserialize)]
pub struct PageResult<T> {
    pub list: T,
    pub total: u64,
}