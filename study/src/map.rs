#[cfg(test)]
mod test {
    use std::collections::HashMap;

    #[test]
    fn test() {
        println!("map-------------------------------------------------------------------------");
        let mut map = HashMap::new();
        map.insert("Blue".to_string(), 10); //插入
        map.insert("Yellow".to_string(), 50); //插入
        map.insert("Blue".to_string(), 30); //替换
        map.remove("Yellow");//删除
        println!("获取: {:?},map: {:?}", map.get(&String::from("Blue")), map);
    }
}