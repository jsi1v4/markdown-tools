use std::{ffi, fs, path};

pub fn is_dir(filename: &String) -> bool {
  fs::metadata(&filename).unwrap().is_dir()
}

pub fn read_dir(filename: &String) -> Vec<path::PathBuf> {
  fs::read_dir(&filename)
    .unwrap()
    .map(|t| t.unwrap().path())
    .collect()
}

pub fn write_file(filename: &String, content: &String) -> () {
  fs::write(&filename, content).unwrap()
}

pub fn is_markdown(path: &path::PathBuf) -> bool {
  let extension = path.extension().and_then(ffi::OsStr::to_str);
  match extension {
    Some("md") => true,
    _ => false,
  }
}

pub fn get_content_from_filename(filename: &String) -> String {
  let path = path::Path::new(filename);
  get_content_from_path(&path)
}

pub fn get_content_from_path(path: &path::Path) -> String {
  let data = fs::read_to_string(path);
  match data {
    Ok(t) => t,
    _ => "".to_string(),
  }
}
