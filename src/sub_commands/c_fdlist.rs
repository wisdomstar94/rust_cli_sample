use library::get_dir_entry_list;

#[derive(clap::Args)]
#[command(
  about="특정 폴더 경로 밑에 존재하는 파일 및 폴더 목록을 확인하는 커맨드입니다.
ex) rust_cli_sample fdlist --path=\"./my_folder\"
  ", 
  long_about = None)
]
pub struct CliArgs {
    /// 파일 및 폴더 목록을 확인할 폴더 경로를 입력하세요
    #[arg(short='p', long)]
    path: Option<String>,
}

pub fn run(args: CliArgs) {
  println!("fdlist 커맨드 실행됨!");

  if let None = args.path {
    panic!("path 인자가 주어지지 않았습니다!");
  }

  if let Some(path) = args.path {
    println!("path is {}", path);
    match get_dir_entry_list(path) {
      Ok(result) => {
        for item in result {
          let path_buffer = item.path();
          let path_obj = path_buffer.as_path();
          if let Some(path_str) = path_obj.to_str() {
            println!("path_str is {}", path_str);
          }
          if let Ok(metadata) = path_obj.metadata() {
            println!("is_dir = {:?}", metadata.is_dir());
          }
          // if let Some(parent) = path_obj.parent() {

          // }
        }
      },
      Err(error) => {
        println!("****** fdlist 실패! : {:?}", error);
      },
    }
  }
}