use std::fs;

#[derive(clap::Args)]
#[command(
  about="파일을 다른 경로로 복사시키는 커맨드입니다.
(폴더 복사는 지원되지 않습니다.)
ex) rust_cli_sample copy --original=\"./zz.txt\" --copied=\"./src/zz.txt\"  
", 
  long_about = None)
]
pub struct CliArgs {
    /// 복사할 파일 또는 폴더 경로를 입력하세요
    #[arg(short='o', long)]
    original: Option<String>,

    /// 복사 후의 파일 또는 폴더 경로를 입력하세요
    #[arg(short='t', long)]
    copied: Option<String>,
}

pub fn run(args: CliArgs) {
  println!("copy 커맨드 실행됨!");

  if let None = args.original {
    panic!("original 인자가 주어지지 않았습니다!");
  }

  if let None = args.copied {
    panic!("copied 인자가 주어지지 않았습니다!");
  }

  if let (Some(original), Some(copied)) = (args.original, args.copied) {
    println!("original is {}", original);
    println!("copied is {}", copied);
    let result = fs::copy(original, copied);
    match result {
        Ok(_) => {
          println!("****** copy 성공!");
        },
        Err(error) => {
          println!("****** copy 실패! : {:?}", error);
        },
    }
  }
}