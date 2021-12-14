use std::fs::File;
use std::io::{BufReader, Read};

pub(crate) fn read_file(path: &str) -> String {
    let file = File::open(path).expect("File not found");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).expect("Failed to read file");
    contents
}
