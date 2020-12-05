use crate::utils;
use std::env;

pub fn run() {
  let from = env::args().nth(2).expect("First arg 'from' is required!");
  let to = env::args().nth(3).expect("Second arg 'to' is required!");

  let mut content = String::new();
  if utils::is_dir(&from) {
    let paths = utils::read_dir(&from);
    for path in paths {
      if utils::is_markdown(&path) {
        content = format!("{}{}", utils::get_content_from_path(&path), content);
      }
    }
  } else {
    content = utils::get_content_from_filename(&from);
  }

  if !content.is_empty() {
    utils::write_file(&to, &content);
    println!("Merge done.");
  } else {
    println!("No content!");
  }
}
