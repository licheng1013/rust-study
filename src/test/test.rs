/// 枚举定义
// enum Type {
//     V1,
//     V2,
// }


use std::collections::HashMap;

/// 其他demo
pub fn test_study() {
    println!("开始练习-----------------------------------------------");
    let mut  v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    println!("{}",v[0]);


    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let  score = scores.get(&String::from("Blue"));
    println!("{:?}",score);
    println!("字符串转换: {}", stringify!(123));

    println!("练习结束-----------------------------------------------");
}
