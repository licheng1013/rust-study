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

/// 不可变闭包示例 -> 如果在 “不可变闭包” 中修改其他可变变量会导致错误。
pub fn test1() {
    let v = |a: i32, b: i32| -> i32{
        a + b
    };
    let result = fun_test1(5, 5, &v);
    println!("不可变闭包结果: {}", result);
}


/// 可变闭包示例
pub fn test2() {
    let mut str = "Hello ".to_string();
    let mut v = |a: i32, b: i32| -> i32{
        str += " World";
        a + b
    };
    let result = fun_test2(5, 5, &mut v);
    println!("可变闭包结果: {},修改外部数据: {}", result, str);
}

/// 所有权移动
pub fn test3() {
    let a = Box::new(10);
    let b = Box::new(10);

    let v = Box::new(|a: i32, b: i32| -> i32{
        return a + b;
    });
    let result = fun_test3(a, b, v);

    println!("转移所有权闭包结果: {}", result);
   // println!("变量打印: {:?}-{:?}", a, b) // 这里取消注释运行会出错误，因为a，b已经被的所有权已经被转移了
}