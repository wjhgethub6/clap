use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    a: u32,
    b: u32,
    #[arg(value_parser = convert)]
    c: i32,

    #[arg(long)]
    name: String,

    #[arg(long)]
    verbose: bool,

}

fn convert(s: &str) -> Result<i32, String> {
    let number: i32 = s
    .parse()
    .map_err(|_| format!("`{}` isn't a valid number", s))?;

    return Ok(-number as i32);
}
///转换器，我们自己做个转换器，把输入数字的负数打印出来
/// #Examples
/// '''
/// .\target\debug\examples\convert.exe 1 2 3 --name hello --verbose 
/// '''
fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
}