use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// 线程阻塞示例
pub fn test1() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("线程: {} 号产生的子线程!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("线程: {} 从主线程!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 阻塞至线程执行完成
    handle.join().unwrap();
}


/// 所有权测试
pub fn test2() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("列表: {:?}", v);
    });
    // drop(v); // -> 这里注释取消得话会引发报错，因为所有权已经被转移到了线程里面了
    handle.join().unwrap();
}

/// 消息传递在线程通信
pub fn test3(){
    // mpsc 是 多个生产者，单个消费者
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); //如果接受端已经没了，那么就会出错
        // println!("val is {}", val); // 这里取消注释运行，则会看到所有权被转移
    });

    let received = rx.recv().unwrap(); // 阻塞等待数据回来;
    println!("收到线程数据: {}", received);

}

/// 列表消息传递
pub fn  test4(){
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}