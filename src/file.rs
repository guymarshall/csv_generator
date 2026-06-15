use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn file_to_vector(filename: &str) -> Vec<String> {
    let file: File = File::open(filename).expect("File should be opened");
    let reader: BufReader<File> = BufReader::new(file);

    let words: Vec<String> = reader
        .lines()
        .map(|line: Result<String, Error>| line.expect("Line should be valid"))
        .collect();

    words
}
