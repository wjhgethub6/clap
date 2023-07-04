use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    a: u32,
    b: u32,
    c: u32,
    #[arg(long)]
    name: String
}

///命名选项参数 {#命名选项参数}
/// 使用这个的时候，需要在字段上添加一个 #[arg(short, long)] ，就可以生成短选项和长选项
/// #Examples
/// '''
/// .\target\debug\examples\arg_para.exe 1 2 3 --name hello
/// '''
fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
}