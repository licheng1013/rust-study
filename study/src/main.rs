mod async_test;
#[allow(dead_code)]
mod closure;
#[allow(dead_code)]
mod file;
#[allow(dead_code)]
mod interface;
#[allow(dead_code)]
mod list;
#[allow(dead_code)]
mod macro_demo;
#[allow(dead_code)]
mod map;
mod test;
#[allow(dead_code)]
mod thread;

#[allow(dead_code)]
mod command;

#[tokio::main]
async fn main() {
    test::test_study();

    println!("异步-------------------------------------------------------------------------");
    async_test::test1().await;
}
