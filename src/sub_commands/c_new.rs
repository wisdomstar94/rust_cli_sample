#[derive(clap::Args)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[arg(short, long)]
    name: Option<String>,
    #[arg(short, long)]
    time: Option<u32>,
}

pub fn run(args: CliArgs) {
  println!("new 커맨드 실행됨!");
  println!("{:?}", args.name);
  println!("{:?}", args.time);
}