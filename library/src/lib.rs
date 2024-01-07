use std::{path::{PathBuf, Path}, env, fs::{DirEntry, self}, io::{self, Error}};

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

pub fn get_dir_entry_list(target_folder_path: String) -> Result<Vec<DirEntry>, Error> {
  match fs::read_dir(target_folder_path) {
    Ok(result) => {
      let vec: Vec<io::Result<DirEntry>> = result.collect();
      let mut dir_entry_list: Vec<DirEntry> = Vec::new();
      for item in vec {
        if let Ok(v) = item {
          dir_entry_list.push(v);
        }
      }
      Ok(dir_entry_list)
    },
    Err(error) => Err(error),
  }
}