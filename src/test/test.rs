/// 枚举定义
// enum Type {
//     V1,
//     V2,
// }


use std::collections::HashMap;
use std::fs::File;
use std::io::{ErrorKind, Read, Write};
use std::path::Path;

/// 其他demo
pub fn test_study() {
    println!("开始练习-----------------------------------------------");
    list();
    map();

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


    println!("\n文件读取-----------------------------------------------");
    read_file_test();
    // println!("\n泛型测试-----------------------------------------------");
    // println!("泛型的最大值: {}",get_max(&[1,2,3,4]));


    println!("练习结束-----------------------------------------------");
}

/// 列表
fn list() {
    println!("list-----------------------------------------------");
    // 列表
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2); //增加
    v.remove(1);//删除
    v.insert(v.len(), 3);
    println!("获取: {:?},长度: {:?},列表: {:?}", v.get(0), v.len(), v);
}

fn map() {
    // map
    println!("\nmap-----------------------------------------------");
    let mut map = HashMap::new();
    map.insert("Blue".to_string(), 10);//插入
    map.insert("Yellow".to_string(), 50);//插入
    map.insert("Blue".to_string(), 30);//替换
    map.remove("Yellow");
    println!("获取: {:?},map: {:?}", map.get(&String::from("Blue")), map);
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

/// 文件读取
fn read_file_test() {
    let file_name = "Hello.txt";

    let f = File::open(file_name);
    let mut f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_name) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            err => {
                panic!("Problem opening the file: {:?}", err)
            }
        }
    };

    let _result = f.write("HelloWorld".as_ref());
    println!("写入的字符数: {:?}", _result);

    let mut read_info = String::new();
    let result = f.read_to_string(&mut read_info);
    println!("读取文件: {}", read_info);
    println!("读取的字符数: {:?}", result);
    println!("文件信息: {:#?}", f.metadata());

    let string = file_name.to_owned() + "1";
    let path = Path::new(&string);
    println!("文件的存在与否需要通过: is_file来进行判断");
    println!("是文件: {},是目录: {}", path.is_file(), path.is_dir());
    println!("文件名: {:#?}", path.file_name())
}

// 获取最大数测试失败！
// fn get_max<T>(list: &[T]) -> T {
//     let mut v :T;
//     v = &list[0];
//     return v
// }
