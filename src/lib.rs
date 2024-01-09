use clap::Parser;
mod sub_commands;

#[derive(Parser)] // requires `derive` feature
#[command(name = "rust_cli_sample")]
#[command(bin_name = "rust_cli_sample")]
#[command(about = "*************************************\n********** 러스트 cli 샘플 **********\n*************************************")]
enum Cli {
    // Subcommands...
    New(sub_commands::c_new::CliArgs), 
    Check(sub_commands::c_check::CliArgs), 
    Move(sub_commands::c_move::CliArgs),
    Copy(sub_commands::c_copy::CliArgs),
    Mkdir(sub_commands::c_mkdir::CliArgs),
    Wdir(sub_commands::c_wdir::CliArgs),
    Pathjoin(sub_commands::c_pathjoin::CliArgs),
    Fdlist(sub_commands::c_fdlist::CliArgs),
    Exist(sub_commands::c_exist::CliArgs),
}

pub fn run() {
    let parse_cli = Cli::parse();
    match parse_cli {
        Cli::New(args) => sub_commands::c_new::run(args),
        Cli::Check(args) => sub_commands::c_check::run(args),
        Cli::Move(args) => sub_commands::c_move::run(args),
        Cli::Copy(args) => sub_commands::c_copy::run(args),
        Cli::Mkdir(args) => sub_commands::c_mkdir::run(args),
        Cli::Wdir(args) => sub_commands::c_wdir::run(args),
        Cli::Pathjoin(args) => sub_commands::c_pathjoin::run(args),
        Cli::Fdlist(args) => sub_commands::c_fdlist::run(args),
        Cli::Exist(args) => sub_commands::c_exist::run(args),
    }
}