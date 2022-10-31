use std::sync::{Arc, mpsc, Mutex};
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
pub fn test3() {
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

/// 列表消息传递 -> 多个线程消息传递
pub fn test4() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

/// 共享变量线程示例
pub fn test5() {
    // 互斥锁
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // 阻塞获取锁
            let mut num = counter.lock().unwrap();
            *num += 100;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
