#[cfg(test)]
mod test_command {
    use std::process::Command;

    #[test]
    fn test() {
        println!("list-------------------------------------------------------------------------");
        let output = Command::new(r"D:\User\MyVps\v2rayN.exe")
            .output()
            .expect("failed to execute process");

        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
}
