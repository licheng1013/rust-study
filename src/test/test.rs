/// 枚举定义
// enum Type {
//     V1,
//     V2,
// }


use std::collections::HashMap;

/// 其他demo
pub fn test_study() {
    println!("开始练习-----------------------------------------------");
    println!("list-----------------------------------------------");
    // 列表
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    println!("{}", v[0]);

    // map
    println!("\nmap-----------------------------------------------");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let score = scores.get(&String::from("Blue"));
    println!("{:?}", score);
    println!("字符串转换: {}", stringify!(123));

    // 所有权
    println!("\n所有权-----------------------------------------------");
    println!("获取字符串长度: {}", ownership(&String::from("Hello")));
    let mut str = "Hello".to_string();
    change_str(&mut str);
    println!("可变引用: {}", str);

    // 结构体
    println!("\n结构体-----------------------------------------------");
    let mut user = User {
        id: 1,
        username: "小明".to_string(),
        password: "123456".to_string(),
        nick_name: "明明".to_string(),
    };
    user.id = 2;
    user.nick_name = "小王".to_string();
    //println!("结构体: {:#?}",user);
    // 宏打印
    dbg!(&user);
    println!("结构体方法{}", user.get_username_and_password());

    println!("\n枚举-----------------------------------------------");
    println!("枚举对象: {:?}", Type::MONKEY);
    println!("枚举对象: {:?}", Type::CAT);
    println!("枚举对比: {:?},{:?}", Type::CAT == Type::CAT, Type::MONKEY == Type::CAT);
    println!("枚举方法: {:?}", get_type(Type::CAT));

    println!("练习结束-----------------------------------------------");
}


/// 所有权测试
fn ownership(s: &String) -> usize {
    return s.len();
}

/// 可变引用
fn change_str(s: &mut String) {
    s.push_str(",你好")
}

/// 结构体的字段必须被使用否则就会警告
#[derive(Debug)]
struct User {
    id: i64,
    username: String,
    password: String,
    nick_name: String,
}

impl User {
    fn get_username_and_password(&self) -> String {
        let string = self.password.to_string();
        return self.username.to_string() + &string;
    }
}

/// 开启打印
#[derive(Debug)]
#[derive(PartialEq)]
enum Type {
    CAT,
    MONKEY,
}

/// 模式匹配
fn get_type(t: Type) -> i32 {
    match t {
        Type::CAT => {
            println!("获取了小猫！");
            return 1;
        }
        Type::MONKEY => 2,
    }
}
