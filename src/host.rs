use regex::Regex;
use std::{fs::OpenOptions, io::Read};

// Get host ip in wsl2 enviroment
pub fn get_host_ip() -> String {
    let mut file = OpenOptions::new()
        .read(true)
        .open("/etc/resolv.conf")
        .expect("打开/etc/resolv.conf文件失败!");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("读取文件内容失败");

    let re = Regex::new(r"nameserver (\d+\.\d+.\d+.\d+)").expect("解析nameserver失败");
    let cap = re.captures(&content).expect("解析namerserver失败");
    let ip = cap.get(1).unwrap().as_str();
    return ip.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_host_ip() {
        println!("host ip is:{}", get_host_ip());
    }
}
