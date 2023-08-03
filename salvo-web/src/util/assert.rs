use std::future::IntoFuture;
use serde_json::Number;

pub struct Assert{

}

impl Assert {
    /// 空
    pub fn empty(msg: Option<String>) {
        if msg == None || msg.unwrap().len() == 0 {
            panic!("字符串为空");
        }
    }
}