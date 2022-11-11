use crate::closure::{test1, test2, test3};

mod async_test;
mod closure;
mod interface;
mod macro_demo;
mod socket;
mod test;
mod thread;

#[tokio::main]
async fn main() {
    test::test_study();
    println!("闭包-------------------------------------------------------------------------");
    test1();
    test2();
    test3();
    println!("线程-------------------------------------------------------------------------");
    thread::test1();
    thread::test2();
    thread::test3();
    thread::test4();
    thread::test5();
    println!("多态-------------------------------------------------------------------------");
    interface::test1();
    println!("宏-------------------------------------------------------------------------");
    macro_demo::macro_test();
    println!("异步-------------------------------------------------------------------------");
    async_test::test1().await;
}
