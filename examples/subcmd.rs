use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Add {
    name: Option<String>
    }
}

/// 在结构中使用 #[command(subcommand)] 表示以下结构是一个子命令
/// 用 #[derive(Subcommand)] 来声明一个子命令结构
/// 调用的时候，使用 myapp add hello 即可
/// # Examples
/// '''
/// .\target\debug\examples\subcmd.exe add hello
/// '''
fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Add {name} => {
            println!("myapp add was used, name is: {:?}", name)
        }
    }
}

