use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
///泛型返回数据
pub struct R<T> {
    /// 验证码
    pub code: i8,
    /// 消息
    pub msg: String,
    /// 类型
    pub data: T,
}

/// 正确消息返回
pub fn ok_msg(msg: String) -> R<()> {
    let r = R {
        code: 0,
        msg,
        data: (),
    };
    return r;
}

/// 正确结果返回
pub fn ok_data<T>(data: T) -> R<T> {
    let r = R {
        code: 0,
        msg: "".to_string(),
        data,
    };
    return r;
}

/// 错误结果返回 TODO 目前还未测试
pub fn fail(msg: String) -> R<()> {
    let r = R {
        code: -1,
        msg,
        data: (),
    };
    return r;
}
