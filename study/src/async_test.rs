use chrono::Local;

async fn say_world() {
    println!(" world");
}


pub async fn test1(){
    let old_time = Local::now();
    // 调用 `say_world()` 不会执行 `say_world() 的主体
    print!("hello");
    say_world().await;
    let new_time = Local::now();
    println!("耗时: {:?}",new_time.timestamp_millis() - old_time.timestamp_millis())
}

