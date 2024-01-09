use std::path::Path;
use library::is_exist;

#[derive(clap::Args)]
#[command(
  about="폴더 및 파일이 존재하는지 체크하는 테스트용 커맨드 입니다.
ex) rust_cli_sample exist -p=\"/a/c/text.txt\"
", 
  long_about = None)
]

#[derive(Debug)]
pub struct CliArgs {
  /// 존재하는지 확인하고 싶은 파일 경로 또는 폴더 경로를 입력하세요
  #[arg(short='p', long)]
  path: Option<String>,
}

pub fn run(args: CliArgs) {
  println!("exist 커맨드 실행됨!");

  if let None = args.path {
    panic!("path 인자가 주어지지 않았습니다!");
  }

  if let Some(path) = args.path {
    if is_exist(Path::new(&path)) {
      println!("해당 파일 또는 폴더 존재함!");
    } else {
      println!("해당 파일 또는 폴더 존재하지 않음!");
    }
  }
}