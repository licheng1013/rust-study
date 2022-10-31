use crate::closure::{test1, test2, test3};

mod closure;
mod interface;
mod test;
mod thread;
mod socket;
mod macro_demo;

fn main() {
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

}
