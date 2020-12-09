extern crate regex;
use regex::Regex;
use std::{ffi, fs, path};

pub struct Content {
  header: String,
  body: String,
}

impl Content {
  pub fn new(header: String, body: String) -> Self {
    Content {
      header: header,
      body: body,
    }
  }
  pub fn header(&self) -> &String {
    &self.header
  }
  pub fn body(&self) -> &String {
    &self.body
  }
}

pub fn is_dir(filename: &String) -> bool {
  let msg = format!("{} <= No such file or folder", filename);
  fs::metadata(&filename).expect(&msg).is_dir()
}

pub fn name_file(filename: &String) -> String {
  let os_str = path::Path::new(filename).file_stem().unwrap().to_str();
  String::from(os_str.unwrap())
}

pub fn read_dir(filename: &String) -> Vec<String> {
  fs::read_dir(&filename)
    .unwrap()
    .map(|t| t.unwrap().path())
    .filter(|t| {
      let ext = t.extension().and_then(ffi::OsStr::to_str);
      if let Some("md") = ext {
        true
      } else {
        false
      }
    })
    .map(|t| t.display().to_string())
    .collect()
}

pub fn write_file(filename: &String, content: &String) -> () {
  fs::write(&filename, content).unwrap()
}

pub fn get_data_from_filename(filename: &String) -> String {
  let data = fs::read_to_string(filename);
  match data {
    Ok(t) => t,
    _ => "".to_string(),
  }
}

pub fn get_md_content(content: &String) -> Content {
  let re_header = Regex::new("^(---)").unwrap();
  let re_separator = Regex::new("(---)").unwrap();
  let raw_rows: Vec<&str> = content.split("\r\n").collect();
  let has_meta = re_header.is_match(raw_rows[0]); // has metadata
  let mut row_separator = 0;
  let mut i = 1; // ignore first line (separator)
  let mut header: Vec<&str> = Vec::new();
  if has_meta {
    while i != raw_rows.len() {
      if re_separator.is_match(raw_rows[i]) {
        row_separator = i + 1;
        break;
      }
      header.push(raw_rows[i]);
      i += 1;
    }
  }
  Content::new(
    header.join("\r\n"),
    raw_rows[row_separator..raw_rows.len()].join("\r\n"),
  )
}

pub fn get_md_prop(prop: &str, raw_rows: &str) -> String {
  let prop_name = format!("{}:(.*)", prop);
  let re_prop = Regex::new(&prop_name).unwrap();
  let rows: Vec<&str> = raw_rows.split("\r\n").collect();
  let mut value = String::new();
  for row in rows {
    if re_prop.is_match(row) {
      let raw_prop: Vec<&str> = row.split(":").collect();
      let raw_value = raw_prop[1];
      value = raw_value.replace(" ", "").replace("\r\n", "");
      break;
    }
  }
  value
}
