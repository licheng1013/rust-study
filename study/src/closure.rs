#[cfg(test)]
mod test {
    use crate::closure::{fun_test1, fun_test2, fun_test3};

    #[test]
    /// 不可变闭包示例 -> 使用场景: 不希望能使用外部的值,通常用于扩展规则,例如解码器...
    fn test() {
        println!("闭包-------------------------------------------------------------------------");
        //  如果在 “不可变闭包” 中修改其他可变变量会导致错误,-> 例如这下面两行注释的代码，如果开启就会出现错误

        //let mut str = "Hello ".to_string();
        let v = |a: i32, b: i32| -> i32 {
            //str += " World";
            a + b
        };
        let result = fun_test1(5, 5, &v);
        println!("不可变闭包结果: {}", result);
    }


    #[test]
    /// 可变闭包示例 -> 使用场景: 用于匿名方法使用,可在匿名方法修改外部的变量值
    fn test_2() {
        let mut str = "Hello ".to_string();
        let mut v = |a: i32, b: i32| -> i32 {
            str += " World";
            a + b
        };
        let result = fun_test2(5, 5, &mut v);
        println!("可变闭包结果: {},修改外部数据: {}", result, str);
    }

    #[test]
    /// 所有权移动 -> 使用场景: 上层传入的变量不希望传递函数后，还能在使用。
    fn test_3() {
        let a = Box::new(10);
        let b = Box::new(10);

        let v = Box::new(|a: i32, b: i32| -> i32 {
            return a + b;
        });
        let result = fun_test3(a, b, v);

        println!("转移所有权闭包结果: {}", result);
        //println!("变量打印: {:?}-{:?}", a, b) // 这里取消注释运行会出错误，因为a，b已经被的所有权已经被转移了
    }
}

/// 对于闭包的演示示例

///  Fn -> 不可变闭包,
pub fn fun_test1(a: i32, b: i32, f: &dyn Fn(i32, i32) -> i32) -> i32 {
    f(a, b)
}

/// FnMut -> 可变闭包 = 能够修改调用时的变量,
pub fn fun_test2(a: i32, b: i32, f: &mut dyn FnMut(i32, i32) -> i32) -> i32 {
    f(a, b)
}

///  FnOnce -> 获取所有权,其原先变量就不可以访问了,
pub fn fun_test3(a: Box<i32>, b: Box<i32>, f: Box<dyn FnOnce(i32, i32) -> i32>) -> i32 {
    f(*a, *b)
}









