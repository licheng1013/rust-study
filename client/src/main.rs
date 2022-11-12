
use std::io::{self, prelude::*, BufReader};
use std::net::TcpStream;
use std::str;

fn main() -> io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:8080")?;
    //创建变量stream，直接连接sever端
    loop {
        let mut reader = BufReader::new(&stream);
        //从stream流创建一个读，目的是要从我们的server端读，
        let mut buffer: Vec<u8> = Vec::new();
        reader.read_until(b'\n', &mut buffer)?;
        //一直读到换行为止（b'\n'中的b表示字节），读到buffer里面
        println!("服务端数据: {}", str::from_utf8(&buffer).unwrap());
    }
}
