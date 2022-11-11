//
//@author Bayek
//@dev server端进行监听，在client端发起链接
//
use std::io::{self, prelude::*, BufReader, Write};
use std::net::TcpStream;
use std::str;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    //创建变量stream，直接连接sever端
    for _ in 0..10 {
        let mut input = String::new();
        //定义一个String类型的输入
        io::stdin().read_line(&mut input).expect("Failed to read!");
        //从标准输入读入一行，读入input里面，如果有问题的话，提示“读取失败”
        stream.write(input.as_bytes()).expect("Failed to write!");
        //把input读取的内容，转换成bytes后，写到stream流里面去，如果写入失败，提示“写入失败”

        let mut reader = BufReader::new(&stream);
        //从stream流创建一个读，目的是要从我们的server端读，
        let mut buffer: Vec<u8> = Vec::new();
        //用Vector创建一个buffer变量
        reader.read_until(b'\n', &mut buffer).expect("Failed to read into buffer");
        //一直读到换行为止（b'\n'中的b表示字节），读到buffer里面
        println!("read from server: {}", str::from_utf8(&buffer).unwrap());
        //把读取到buffer中的内容打印出来
    }
    Ok(())
}
