// 设置当前http proxy
pub fn set_wsl_http_proxy(port: u32, ip: &str) {
    println!("export http_proxy=http://{}:{}", ip, port);
    println!("export https_proxy=http://{}:{}", ip, port);
}
