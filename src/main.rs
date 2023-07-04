use clap::{Parser,Args,Subcommand};

mod api;

#[derive(Parser)]

#[command(author,version)]

#[command(about = "stringer - a simple CLI to some usage things")]

struct Cli {
    #[command(subcommand)]
    command:Option<Commands>,
}


#[derive(Subcommand)]
enum Commands {
    /// Reverses a string (1) 
    Reverse(Reverse),
    /// Inspects a string (2)
    Inspect(Inspect),
    /// For Test
    Test(Test),
}



#[derive(Args)]
struct Test {
    string:Option<String>,
}

#[derive(Args)]
struct Reverse {
    string: Option<String>,
}

#[derive(Args)]
struct Inspect {
    /// Tje tsing to inspect.
    string:Option<String>,
    #[arg(short = 'd',long = "digits")]
    only_digits:bool,
}


fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Reverse(name)) => {
            match name.string {
                Some(ref _name) => {
                    let reverse = api::stringer::reverse(_name);
                    println!("{}",reverse);
                }
                None => {
                    println!("Please provide a string to reverse");
                }
            }
        }
        Some(Commands::Inspect(name)) => {
            match name.string {
                Some(ref _name) => {
                    let (res, kind) = api::stringer::inspect(_name, name.only_digits);

                    let mut plural_s = "s";
                    if res == 1 {
                        plural_s = "";
                    }

                    println!("{:?} has {} {}{}.", _name, res, kind, plural_s);
                }
                None => {
                    println!("Please provide a string to inspect");
                }
            }
        }
        Some(Commands::Test(name)) => {
            match name.string {
                Some(ref name) => {
                    let reverse = api::stringer::reverse(name);
                    println!("{}",reverse);
                }
                None => {
                    println!("Please provide a string to Test");
                }
            }
        }

        None => {}
    }
}

