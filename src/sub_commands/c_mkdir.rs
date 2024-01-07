use std::fs;

#[derive(clap::Args)]
#[command(
  about="폴더를 생성하는 커맨드입니다.
ex) rust_cli_sample mkdir --path=\"./my_folder\"
  ", 
  long_about = None)
]
pub struct CliArgs {
    /// 생성 될 폴더 경로를 입력하세요
    #[arg(short='p', long)]
    path: Option<String>,
}

pub fn run(args: CliArgs) {
  println!("mkdir 커맨드 실행됨!");

  if let None = args.path {
    panic!("path 인자가 주어지지 않았습니다!");
  }

  if let Some(path) = args.path {
    println!("path is {}", path);
    let result = fs::create_dir_all(path);
    match result {
        Ok(_) => {
          println!("****** mkdir 성공!");
        },
        Err(error) => {
          println!("****** mkdir 실패! : {:?}", error);
        },
    }
  }
}