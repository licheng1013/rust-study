#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}


pub fn macro_test(){
    let vec1 = vec!(1,2,3);
    println!("列表: {:?}",vec1);
}


#[cfg(test)]
mod test {
    use crate::macro_demo;

    #[test]
    fn test() {
        println!("宏-------------------------------------------------------------------------");
        macro_demo::macro_test();
    }
}