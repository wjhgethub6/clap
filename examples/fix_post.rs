use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    a: u32,
    b: u32,
    c: u32,
    name: String
}
///固定位置参数
/// 程序的解析顺序好像是按照结构体中字段的声明顺序决定的
/// #Examples
/// '''
/// .\target\debug\examples\fix_post.exe 1 2 3 hello
/// '''
fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
}