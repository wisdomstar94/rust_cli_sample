use std::{path::{PathBuf, Path}, env};

pub fn path_join(vec: &Vec<String>) -> PathBuf {
  let path_obj = Path::new("");
  let mut path_buffer = path_obj.join("");
  for item in vec {
    path_buffer = path_buffer.join(item);
  }
  path_buffer
}

pub fn get_current_working_directory_path() -> Option<String> {
  let mut result: Option<String> = None;
  if let Ok(path) = env::current_dir() {
    if let Some(p) = path.to_str() {
      result = Some(p.to_string());
    }
  }
  result
}
