use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct R<T> {
    /// 验证码
    pub code: i8,
    /// 消息
    pub msg: String,
    /// 类型
    pub data: T,
}

// pub fn ok_msg<T>(msg:T) -> R<T> {
//     let r = R{
//         code: 0,
//         msg,
//         data: ()
//     };
//     return r;
// }

// pub fn ok_data<T>(data:T) -> R<T> {
//     let r = R{
//         code: 0,
//         msg: "".to_string(),
//         data
//     };
//     return r;
// }