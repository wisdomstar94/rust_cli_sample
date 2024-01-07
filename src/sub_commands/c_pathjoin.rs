use std::path::{Path, PathBuf};

#[derive(clap::Args)]
#[command(
  about="폴더 및 파일 단위로 나눠진 path fragment 들을 조합하는 테스트용 커맨드 입니다.
ex) rust_cli_sample pathjoin -f=\"/a/c\" -f=\"b\" -f=\"tt.txt\"
", 
  long_about = None)
]

#[derive(Debug)]
pub struct CliArgs {
  /// path 조각들을 입력하세요
  #[arg(short='f', long)]
  fragment: Option<Vec<String>>,
}

pub fn run(args: CliArgs) {
  println!("pathjoin 커맨드 실행됨!");

  if let None = args.fragment {
    panic!("fragment 인자가 주어지지 않았습니다!");
  }

  if let Some(vec) = args.fragment {
    let path_buffer = path_join(&vec);
    if let Some(joined_path) = path_buffer.to_str() {
      println!("joined_path = {:?}", joined_path);
    }
  }
}

fn path_join(vec: &Vec<String>) -> PathBuf {
  let path_obj = Path::new("");
  let mut path_buffer = path_obj.join("");
  for item in vec {
    path_buffer = path_buffer.join(item);
  }
  path_buffer
}