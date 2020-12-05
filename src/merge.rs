use crate::utils;
use std::env;

pub fn run() {
  let from = env::args().nth(2).expect("First arg 'from' is required!");
  let to = env::args().nth(3).expect("Second arg 'to' is required!");

  let mut content = String::new();
  if utils::is_dir(&from) {
    for path in utils::read_dir(&from) {
      let url = path.unwrap().path();
      if utils::is_markdown(&url) {
        content = format!("{}{}", utils::get_content_from_path(&url), content);
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