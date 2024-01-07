use clap::Parser;
mod sub_commands;

#[derive(Parser)] // requires `derive` feature
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
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