use std::env;

#[derive(clap::Args)]
#[command(
  about="현재 본 커맨드를 실행하고 있는 현재 working directory 경로를 확인하는 커맨드 입니다.", 
  long_about = None)
]
pub struct CliArgs {}

pub fn run(_: CliArgs) {
  println!("wdir 커맨드 실행됨!");
  if let Ok(path) = env::current_dir() {
    if let Some(p) = path.to_str() {
      println!("current working directory path = {:?}", p);
    }
  }
}