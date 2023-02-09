#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
                println!("{}",$x);
            )*
            temp_vec
        }
    };
}
macro_rules! hello_world {
    () => {
        println!("Hello, world!");
    };
}

#[cfg(test)]
mod test {
    use crate::macro_demo;

    #[test]
    fn test() {
        println!("宏-------------------------------------------------------------------------");
        macro_demo::macro_test();
        hello_world!();
    }
}


pub fn macro_test(){
    let vec1 = vec!(1,2,3);
    let vec2 = vec!("a","b","c");
    println!("列表: {:?}",vec1);
    println!("列表: {:?}",vec2);
}


