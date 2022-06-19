use clap::Parser;

pub mod host;
pub mod proxy;

use crate::host::get_host_ip;
use crate::proxy::set_wsl_http_proxy;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "10809")]
    proxy_port: u32,
}

fn main() {
    let args = Args::parse();

    let host_ip = get_host_ip();
    set_wsl_http_proxy(args.proxy_port, &host_ip);
}
