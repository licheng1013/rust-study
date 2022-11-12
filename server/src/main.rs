use std::net::{TcpListener, TcpStream};

//std::thread库的引入，对输入的每一个流创建一个线程

use std::io::{self, BufReader, Read};


//引入io库，为了处理错误

fn handle_server(stream: TcpStream) -> io::Result<()> {
    loop {
        let mut reader = BufReader::new(&stream);
        //let mut message = "".to_string();
        let mut message = vec![0; 1024 * 8];
        let i = reader.read(&mut message)?;
        if i > 0 {
            println!(
                "服务端数据: {}",
                String::from_utf8((&message[0..i]).to_vec()).unwrap()
            );
            //stream.write("HelloWorld".as_ref()).unwrap();
            //stream.shutdown(Shutdown::Both).unwrap()
        }
    }
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    //定义一个listener，bind函数里面填写的是监听的的ip与端口号,?是一种简写，等价于except,unwrap
    //创建一个容器，用来放线程的句柄
    for stream in listener.incoming() {
        println!("建立连接");
        handle_server(stream?)?;
    }
    Ok(())
}
