use clap::Parser;

#[derive(Parser)] // requires `derive` feature
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum Cli {
    // Subcommands...
    New(NewArgs),
    Check(CheckArgs),
}

#[derive(clap::Args)]
#[command(author, version, about, long_about = None)]
struct NewArgs {
    #[arg(short, long)]
    name: String,
}

#[derive(clap::Args)]
#[command(author, version, about, long_about = None)]
struct CheckArgs {
    #[arg(short, long)]
    time: u32,
}

pub fn run() {
    let parse_cli = Cli::parse();

    match parse_cli {
        Cli::New(args) => {
            println!("new 커맨드 실행됨!");
            println!("{:?}", args.name);
        },
        Cli::Check(args) => {
            println!("check 커맨드 실행됨!");
            println!("{:?}", args.time);
        },
    }
}