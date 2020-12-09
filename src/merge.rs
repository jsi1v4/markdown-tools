use crate::utils;
use std::env;

pub fn run() {
  let from = env::args().nth(2).expect("First arg 'from' is required!");
  let to = env::args().nth(3).expect("Second arg 'to' is required!");

  let mut content = String::new();
  if utils::is_dir(&from) {
    let mut paths = utils::read_dir(&from);
    paths.sort();
    for path in paths {
      content = format!("{}{}\r\n", content, get_content(&path));
    }
  } else {
    content = get_content(&from);
  }

  if content.is_empty() {
    println!("No content!");
  } else {
    utils::write_file(&to, &content);
    println!("Merge done.");
  }
}

fn get_content(path: &String) -> String {
  let data = utils::get_data_from_filename(&path);
  let md_content = utils::get_md_content(&data);
  md_content.body().to_string()
}
