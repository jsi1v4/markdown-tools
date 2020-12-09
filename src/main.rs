mod extract;
mod merge;
mod utils;
use std::env;

fn main() {
    let cmd = env::args().nth(1).expect("Command arg is required!");

    match cmd.as_str() {
        "merge" => merge::run(),
        "extract" => extract::run(),
        _ => println!("Command not found!"),
    }
}
