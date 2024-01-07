#[derive(clap::Args)]
#[command(
  about="테스트용 new 커맨드 입니다.", 
  long_about = None)
]
pub struct CliArgs {
    /// 이름을 입력하세요
    #[arg(short='n', long="name")]
    name: Option<String>,
    /// 시간을 입력하세요
    #[arg(short='t', long="time")]
    time: Option<u32>,
}

pub fn run(args: CliArgs) {
  println!("new 커맨드 실행됨!");
  println!("{:?}", args.name);
  println!("{:?}", args.time);
}