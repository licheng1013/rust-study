#[cfg(test)]
mod test {

    #[test]
    fn test() {
        println!("list-------------------------------------------------------------------------");
        // 列表
        let mut v: Vec<i32> = Vec::new();
        // 增加
        v.push(1);
        v.push(2);
        //删除
        v.remove(1);
        //插入
        v.insert(0, 3);
        println!("获取: {:?},长度: {:?},列表: {:?}", v.get(0), v.len(), v);
        println!("切片: {:?}", &v[0..v.len()]);
    }
}
