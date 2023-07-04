use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(default_value_t = 2020)]
    port: u16
}

///默认值
/// 根据文档上的描述，只需要在字段上加上 #[arg(default_value_t = ?)]
/// #Examples
/// '''
/// .\target\debug\examples\default.exe
/// .\target\debug\examples\default.exe 10
/// '''
fn main() {
    let cli = Cli::parse();
    println!("port: {:?}", cli.port);
}