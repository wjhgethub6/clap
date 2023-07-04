use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(value_parser = port_in_range)]
    port: u16
}
///验证
/// 我们手动制定一个验证器函数，这个函数要返回一个 Result 类型，接受一个 &str 参数
/// Result 的左值类型要与字段类型匹配
/// #Examples
/// '''
/// .\target\debug\examples\verify.exe 65535
/// .\target\debug\examples\verify.exe 65536
/// '''
fn main() {
    let cli = Cli::parse();
    println!("port: {:?}", cli.port);
}

use std::ops::RangeInclusive;

const PORT_RANGE: RangeInclusive<usize> = 1..=65535;

fn port_in_range(s: &str) -> Result<u16, String> {
    let port: usize = s
    .parse()
    .map_err(|_| format!("`{}` isn't a port number", s))?;

    if PORT_RANGE.contains(&port) {
    Ok(port as u16)
    } else {
    Err(format!(
        "Port not in range {} - {}",
        PORT_RANGE.start(),
        PORT_RANGE.end()
    ))
    }
}