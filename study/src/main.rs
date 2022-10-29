use crate::closure::{test1, test2, test3};

mod test;
mod closure;

fn main() {
    test::test_study();
    test1();
    test2();
    test3();
    println!("-------------------------------------------------------------------------");
}
