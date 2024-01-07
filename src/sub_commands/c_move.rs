use std::fs;

#[derive(clap::Args)]
#[command(
  about="\n
파일 또는 폴더를 다른 경로로 이동시키는 커맨드입니다.
ex) 파일 이동시 : rcs move --before=\"./zz.txt\" --after=\"./src/zz.txt\"
ex) 폴더 이동시 : rcs move --before=\"./my_folder\" --after=\"./src/my_folder\"
  ", 
  long_about = None)
]
pub struct CliArgs {
    /// 이동할 파일 또는 폴더 경로를 입력하세요
    #[arg(short='b', long)]
    before: Option<String>,

    /// 새롭게 위치할 파일 또는 폴더 경로를 입력하세요
    #[arg(short='a', long)]
    after: Option<String>,
}

pub fn run(args: CliArgs) {
  println!("move 커맨드 실행됨!");

  if let None = args.before {
    panic!("before 인자가 주어지지 않았습니다!");
  }

  if let None = args.after {
    panic!("after 인자가 주어지지 않았습니다!");
  }

  if let (Some(before), Some(after)) = (args.before, args.after) {
    println!("before is {}", before);
    println!("after is {}", after);
    let result = fs::rename(before, after);
    match result {
        Ok(_) => {
          println!("****** move 성공!");
        },
        Err(error) => {
          println!("****** move 실패! : {:?}", error);
        },
    }
  }
}