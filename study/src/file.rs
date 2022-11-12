use std::fs::File;
use std::io::{ErrorKind, Read, Write};
use std::path::Path;

#[cfg(test)]
mod test {
    use crate::file::read_file_test;

    #[test]
    fn test() {
        println!("文件测试-------------------------------------------------------------------------");
        read_file_test()
    }
}


/// 文件读取
#[allow(dead_code)]
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
        },
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