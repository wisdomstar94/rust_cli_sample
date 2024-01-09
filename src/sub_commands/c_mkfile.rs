use std::{fs, path::Path};

#[derive(clap::Args)]
#[command(
  about="파일을 생성하는 커맨드입니다.
ex) rust_cli_sample mkfile --path=\"./my_folder\" --content=\"테스트 내용..\"
  ", 
  long_about = None)
]
pub struct CliArgs {
    /// 생성 될 파일 경로를 입력하세요
    #[arg(short='p', long)]
    path: Option<String>,
    /// 파일 내용을 입력하세요
    #[arg(short='c', long)]
    content: Option<String>,
}

pub fn run(args: CliArgs) {
  println!("mkfile 커맨드 실행됨!");

  if let None = args.path {
    panic!("path 인자가 주어지지 않았습니다!");
  }

  if let None = args.content {
    panic!("content 인자가 주어지지 않았습니다!");
  }

  if let (Some(path), Some(content)) = (args.path, args.content) {
    println!("path is {}", path);
    let result = fs::write(Path::new(&path), String::from(content));
    match result {
      Ok(_) => {
        println!("****** mkfile 성공!");
      },
      Err(error) => {
        println!("****** mkfile 실패! : {:?}", error);
      },
    }
  }
}