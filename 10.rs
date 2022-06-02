use std::fs::OpenOptions;
use std::io::prelude::*;
use text_io::read;

fn main() {
  let line:String = read!();
  let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("src/my-file.txt") // make a my-file.txt in src folder first
        .unwrap();

    if let Err(e) = writeln!(file,"{}", line) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
