#[cfg(test)]
mod test_command {
    use std::process::Command;

    #[test]
    fn test() {
        // 解压的目录路径
        let path = r"E:\download\4K";
        // 解压的文件路径
        let file_path = r"E:\download\4K\4K.zip";

        // 解压
        let output = Command::new("powershell")
            .arg("Expand-Archive")
            .arg(file_path)
            .arg(path)
            .output()
            .expect("failed to execute process");

        // 打印解压结果
        println!("status: {}", output.status);
    }
}
