use crate::utils;
use std::env;

pub fn run() {
  let from = env::args().nth(2).expect("First arg 'from' is required!");
  let to = env::args().nth(3).expect("Second arg 'to' is required!");

  if !utils::is_dir(&to) {
    panic!("Second arg 'to' must be folder!");
  }

  if utils::is_dir(&from) {
    let paths = utils::read_dir(&from);
    for path in paths {
      extract(&path, &to);
    }
  } else {
    extract(&from, &to);
  }

  println!("Extract done.");
}

fn extract(path: &String, to: &String) {
  let data = utils::get_data_from_filename(&path);
  let md_content = utils::get_md_content(&data);
  let header = md_content.header();
  let content = md_content.body();
  let mut name = utils::get_md_prop("name", header);
  if name.is_empty() {
    name = utils::name_file(&path); // original name
  }
  let path_to = format!("{}/{}.md", to, name);
  utils::write_file(&path_to, &content);
  println!("Extract {}...", path_to);
}
