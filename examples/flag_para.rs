use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    a: u32,
    b: u32,
    c: u32,
    #[arg(long)]
    name: String,

    #[arg(long)]
    verbose: bool
}

///Flag 参数
/// 这个很简单，先添加 #[arg(long)] 或者 #[arg(short)] 宏，再将字段类型改为 bool 即可
/// # Examples
/// '''
/// .\target\debug\examples\flag_para.exe 1 2 3 --name hello --verbose
/// '''
fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
}