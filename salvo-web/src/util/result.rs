use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Result<T>{
    pub code: i8,
    pub msg: String,
    pub data: T,
}



/// 正确消息返回
pub fn ok_msg(msg: String) -> Result<()> {
    let r = Result {
        code: 0,
        msg,
        data: (),
    };
    return r;
}

/// 正确结果返回
pub fn ok_data<T>(data: T) -> Result<T> {
    let r = Result {
        code: 0,
        msg: "".to_string(),
        data,
    };
    return r;
}

pub fn fail(msg: String) -> Result<()> {
    let r = Result {
        code: -1,
        msg,
        data: (),
    };
    return r;
}