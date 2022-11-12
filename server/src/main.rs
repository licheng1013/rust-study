use std::net::{TcpListener, TcpStream};
use std::thread;
//std::thread库的引入，对输入的每一个流创建一个线程
use std::time;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::str;
//引入io库，为了处理错误

fn handle_client(stream: TcpStream) -> io::Result<()> {
    loop {
        let mut reader = BufReader::new(&stream);
        let mut message = "".to_string();
        reader.read_to_string( &mut message)?;
        println!("服务端数据: {}", message);
        //stream.write("HelloWorld\n".as_ref())?;
        //否则把它写回去
        //thread::sleep(time::Duration::from_secs(1));
        //调用sleep函数实现服务的间隔，间隔1s
    }
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    //定义一个listener，bind函数里面填写的是监听的的ip与端口号,?是一种简写，等价于except,unwrap
    //创建一个容器，用来放线程的句柄
    for stream in listener.incoming() {
        println!("建立连接");
        handle_client(stream?)?;
    }
    Ok(())
}

