use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

pub(crate) fn read_file(path: &str) -> String {
    let file = File::open(path).expect("File not found");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .expect("Failed to read file");
    contents
}

pub(crate) fn _dump_debug(path: &str, rowlen: usize, data: &[u8]) {
    let file = File::create(path).expect("Path not found");
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write(data
        .iter()
        .map(|val| val.to_string().to_ascii_lowercase())
        .collect::<Vec<String>>()
        .chunks(rowlen)
        .flat_map(|arr| {
            let mut a = arr.to_vec();
            a.push(String::from("\n"));
            a
        })
        .collect::<Vec<String>>()
        .join("")
        .as_bytes()
    ).expect("Write failed");
    buf_writer.flush().expect("Flush failed");
}