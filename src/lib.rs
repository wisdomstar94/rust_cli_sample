use clap::Parser;
mod sub_commands;

#[derive(Parser)] // requires `derive` feature
#[command(name = "rcs")]
#[command(bin_name = "rcs")]
#[command(about = "*************************************\n********** 러스트 cli 샘플 **********\n*************************************")]
enum Cli {
    // Subcommands...
    New(sub_commands::c_new::CliArgs), 
    Check(sub_commands::c_check::CliArgs), 
}

pub fn run() {
    let parse_cli = Cli::parse();
    match parse_cli {
        Cli::New(args) => sub_commands::c_new::run(args),
        Cli::Check(args) => sub_commands::c_check::run(args),
    }
}