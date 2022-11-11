//
//@author Bayek
//@dev server用于监听
//
use std::net::{TcpListener, TcpStream};
use std::thread;
//std::thread库的引入，对输入的每一个流创建一个线程
use std::time;
use std::io::{self, Write};
//引入io库，为了处理错误

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    //该函数用来处理client（就是这个流），流的格式或者说他的类型就是TcpStream
    //let mut buf = [0; 512];
    //创建一个叫buf的数组，内容为0，长度为512
    loop {
        stream.write("HelloWorld\n".as_ref())?;
        //否则把它写回去
        thread::sleep(time::Duration::from_secs(1));
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

