//
//@author Bayek
//@dev server用于监听
//
use std::net::{TcpListener, TcpStream};
use std::thread;
//std::thread库的引入，对输入的每一个流创建一个线程
use std::time;
use std::io::{self, Read, Write};
//引入io库，为了处理错误

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    //该函数用来处理client（就是这个流），流的格式或者说他的类型就是TcpStream
    let mut buf = [0; 512];
    //创建一个叫buf的数组，内容为0，长度为512
    loop {
        //该循环表示server端永久提供服务，因为默认服务器为永不关闭的
        let bytes_read = stream.read(&mut buf)?;
        //从流里面读内容，读到buf中
        if bytes_read == 0 {
            return Ok(());
            //如果读到的为空（即0），则说明已经结束了
        }
        stream.write(&buf[..bytes_read])?;
        //否则把它写回去
        thread::sleep(time::Duration::from_secs(1));
        //调用sleep函数实现服务的间隔，间隔1s
    }
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    //定义一个listener，bind函数里面填写的是监听的的ip与端口号,?是一种简写，等价于except,unwrap
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    //创建一个容器，用来放线程的句柄
    for stream in listener.incoming() {
        println!("建立连接");
        let stream = stream.expect("failed");
        //转换一下stream流，出现问题，提示“失败”，没有问题，继续下面的操作
        let handle = thread::spawn(move || {
            handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
        });
        //对输入的每一个流来创建一个线程，利用必包进行一个处理
        thread_vec.push(handle);
        //把handle加到容器里面
    }

    for handle in thread_vec {
        //此循环为了等待线程的结束
        handle.join().unwrap();
        //等待结束的具体实现
    }
    Ok(())
}

