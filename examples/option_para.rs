use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    a: u32,
    b: u32,
    c: Option<u32>,

    #[arg(long)]
    name: String,

    #[arg(long)]
    verbose: bool,
}
///可选的参数
/// 这个也很简单，将字段类型改为 Option 即可，这样程序找不到匹配的项时，会将值设置为 None
/// #Examples
/// '''
/// .\target\debug\examples\option_para.exe 1 2 --name hello --verbose
/// '''
fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
}