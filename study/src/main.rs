use crate::closure::{test1, test2, test3};

mod test;
mod closure;
mod thread;

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
}
